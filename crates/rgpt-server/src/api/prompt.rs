use std::sync::Arc;

use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
};
use diesel::{ExpressionMethods, QueryDsl};
use futures::{StreamExt, channel::mpsc::UnboundedReceiver};
use hyper::{Response, StatusCode};
use libserver::{DynRoute, PathEqRouter, Route, make_body_from_stream, make_frame};
use rgpt_cfg::Context;
use rgpt_db::{RunQueryDsl, chat::Chat, msg::Msg};
use serde::Deserialize;

pub fn prompt_route(cx: Arc<Context>) -> DynRoute {
    Route::from_parts(PathEqRouter::new("/api/prompt"), PromptService::new(cx)).make_dyn()
}

#[derive(Clone)]
pub struct PromptService {
    cx: Arc<Context>,
}

impl PromptService {
    pub fn new(cx: Arc<Context>) -> Self {
        Self { cx }
    }
}

impl tower::Service<libserver::Request> for PromptService {
    type Response = libserver::ServiceResponse;
    type Error = libserver::ServiceError;
    type Future = libserver::ServiceBoxFuture;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: libserver::Request) -> Self::Future {
        let cx = self.cx.clone();
        Box::pin(async move { prompt(req, cx).await })
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PromptBodyDes {
    pub text: String,
    pub chatId: Option<i32>,
    pub sessionToken: String,
}

pub async fn prompt(
    req: libserver::Request,
    cx: Arc<Context>,
) -> Result<libserver::ServiceResponse, libserver::ServiceError> {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let headers = req.headers().to_owned();
    let body = crate::collect_body_string(req).await?;
    let prompt_body: PromptBodyDes = serde_json::from_str(&body)?;

    let is_first_message_in_chat = prompt_body.chatId.is_some();

    let (_session, chat) = match prompt_body.chatId {
        Some(id) => {
            let chat = Chat::get_by_id(cx.db(), id).await?;
            let session = crate::validate_session(cx.db(), &headers, Some(chat.user_id)).await?;
            (session, chat)
        }
        None => {
            let session = crate::validate_session(cx.db(), &headers, None).await?;
            let chat = Chat::create(cx.db(), session.user_id, None).await?;
            (session, chat)
        }
    };

    let user_msg = Msg::create(
        cx.db(),
        prompt_body.text,
        "user",
        chat.user_id,
        chat.head_msg,
    )
    .await?;
    let chat = chat.append_to_chat(cx.db(), &user_msg).await?;

    let msgs = chat.msg_chain(cx.db()).await?;

    let request = create_chat_request(msgs, cx.clone())?;

    let (response_tx, rx) = futures::channel::mpsc::unbounded::<String>();

    let resp_stream = cx
        .state
        .openai_client
        .chat()
        .create_stream(request)
        .await?
        .filter_map(|c| async move {
            c.ok()
                .and_then(|stream_chunk| stream_chunk.choices.into_iter().next())
                .and_then(|stream_chunk| stream_chunk.delta.content)
        })
        .inspect(move |stream_chunk| {
            response_tx
                .unbounded_send(stream_chunk.to_owned())
                .expect("receiver was dropped prematurely")
        })
        .map(make_frame);

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("X-Chat-ID", chat.id.to_string())
        .body(make_body_from_stream(resp_stream))?;

    tokio::spawn(buffer_response(
        rx,
        cx.clone(),
        chat,
        user_msg,
        is_first_message_in_chat,
    ));

    Ok(resp)
}

fn create_chat_request(
    msgs: Vec<Msg>,
    cx: Arc<Context>,
) -> Result<CreateChatCompletionRequest, libserver::ServiceError> {
    let built_msgs = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(cx.config.system_message.clone())
            .build()?
            .into(),
    ]
    .into_iter()
    .chain(msgs.into_iter().filter_map(|msg| {
        match msg.sender.as_str() {
            "ai" => ChatCompletionRequestAssistantMessageArgs::default()
                .content(msg.body)
                .build()
                .ok()
                .map(Into::into),
            "user" => ChatCompletionRequestUserMessageArgs::default()
                .content(msg.body)
                .build()
                .ok()
                .map(Into::into),
            _ => None,
        }
    }))
    .collect::<Vec<ChatCompletionRequestMessage>>();

    let request = CreateChatCompletionRequestArgs::default()
        .model(&cx.config.model_name)
        .max_tokens(cx.config.max_tokens)
        .messages(built_msgs)
        .build()?;
    Ok(request)
}

async fn buffer_response(
    mut rx: UnboundedReceiver<String>,
    cx: Arc<Context>,
    chat: Chat,
    user_msg: Msg,
    needs_title: bool,
) -> Result<(), libserver::ServiceError> {
    let mut buf = String::new();
    while let Some(chunk) = rx.next().await {
        buf.push_str(&chunk);
    }

    let ai_msg = Msg::create(cx.db(), buf, "ai", chat.user_id, chat.head_msg).await?;
    let chat = chat.append_to_chat(cx.db(), &ai_msg).await?;

    if needs_title {
        generate_chat_name(cx.clone(), chat, user_msg, ai_msg).await?;
    }

    Ok(())
}

async fn generate_chat_name(
    cx: Arc<Context>,
    chat: Chat,
    user_msg: Msg,
    ai_msg: Msg,
) -> Result<(), libserver::ServiceError> {
    let prompt = ChatCompletionRequestUserMessageArgs::default()
        .content(format!(
            r#"
           Generate a title for the following chat to be displayed. It must be less than 5 words.
           Do not respond with anything but the title

           Chat Content:
           User: {}

           Assistant: {}
        "#,
            user_msg.body, ai_msg.body
        ))
        .build()?
        .into();

    let request = CreateChatCompletionRequestArgs::default()
        .model(&cx.config.model_name)
        .max_tokens(cx.config.max_tokens)
        .messages([prompt])
        .build()
        .unwrap();

    let chat_title = cx
        .state
        .openai_client
        .chat()
        .create(request)
        .await?
        .choices
        .into_iter()
        .next()
        .ok_or(NotFound)?
        .message
        .content;

    diesel::update(rgpt_db::schema::chats::table.find(chat.id))
        .set(rgpt_db::schema::chats::name.eq(chat_title))
        .execute(cx.db())
        .await?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
#[error("Something was not found")]
struct NotFound;
