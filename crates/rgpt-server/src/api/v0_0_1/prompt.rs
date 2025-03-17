use std::sync::Arc;

use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
};
use diesel::{ExpressionMethods, QueryDsl};
use futures::{StreamExt, channel::mpsc::UnboundedSender};
use hyper::{Response, body::Bytes};
use libserver::{DynRoute, PathEqRouter, Request, Route, single_frame_body};
use rgpt_cfg::Context;
use rgpt_db::{RunQueryDsl, chat::Chat, msg::Msg};
use rgpt_stream::AttachHandle;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathEqRouter::new("/api/v0.0.1/user_chats");

    Route::from_parts(router, PromptService::new(cx)).make_dyn()
}

pub async fn prompt(req: Request, cx: Arc<Context>) -> libserver::ServiceResult {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let headers = req.headers().to_owned();
    let body = crate::collect_body_string(req).await?;

    let PromptServiceInput { text, chat_id } = serde_json::from_str(&body)?;

    let is_first_message_in_chat = chat_id.is_none();

    let (_session, mut chat) = match chat_id {
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

    let user_msg = Msg::create(cx.db(), text, "ai", chat.user_id, chat.head_msg).await?;

    let chat_title = if is_first_message_in_chat {
        let chat_title = generate_chat_name(cx.clone(), &user_msg).await?;
        diesel::update(rgpt_db::schema::chats::table.find(chat.id))
            .set(rgpt_db::schema::chats::name.eq(chat_title.clone()))
            .execute(cx.db())
            .await?;
        Some(chat_title)
    } else {
        None
    };

    chat = chat.append_to_chat(cx.db(), &user_msg).await?;

    let chat_msgs = chat.msg_chain(cx.db()).await?;

    let model_request = create_chat_request(cx.clone(), chat_msgs)?;

    tokio::spawn(stream_model_response(chat.id, model_request, cx.clone()));

    let response = serde_json::to_string(&PromptServiceResponse {
        chat_id: chat.id,
        chat_title: chat_title.as_deref(),
    })?;

    let body = single_frame_body(response);
    Ok(Response::new(body))
}

#[derive(Serialize)]
struct PromptServiceResponse<'a> {
    chat_id: i32,
    chat_title: Option<&'a str>,
}

async fn generate_chat_name(
    cx: Arc<Context>,
    user_msg: &Msg,
) -> Result<String, libserver::ServiceError> {
    let prompt = ChatCompletionRequestUserMessageArgs::default()
        .content(format!(
            r#"
           Generate a title for the following chat to be displayed. It must be less than 5 words.
           Do not respond with anything but the title

           Chat Content:
           User: {}
        "#,
            user_msg.body,
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
        .unwrap()
        .message
        .content
        .unwrap();

    Ok(chat_title)
}

fn create_chat_request(
    cx: Arc<Context>,
    msgs: Vec<Msg>,
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

pub async fn stream_model_response(
    chat_id: i32,
    completion_request: CreateChatCompletionRequest,
    cx: Arc<Context>,
) -> Result<(), libserver::ServiceError> {
    let (attach_tx, attach_rx) = oneshot::channel();
    let attach_handle = AttachHandle::new(attach_tx);
    cx.state
        .stream_registry
        .lock()
        .await
        .register(chat_id, attach_handle)
        .expect("unreachable");

    let mut channel = Err::<UnboundedSender<Bytes>, _>(attach_rx);
    let mut buf = String::new();

    let mut stream = Box::pin(
        cx.state
            .openai_client
            .chat()
            .create_stream(completion_request)
            .await?
            .filter_map(|c| async move {
                c.ok()
                    .and_then(|stream_chunk| stream_chunk.choices.into_iter().next())
                    .and_then(|stream_chunk| stream_chunk.delta.content)
            }),
    );

    loop {
        if let Err(ref mut rx) = channel {
            if let Ok(tx) = rx.try_recv() {
                tx.unbounded_send(buf.clone().into())?;
                channel = Ok(tx);
            }
        }

        if let Some(chunk) = stream.next().await {
            buf.push_str(&chunk);
            if let Ok(ref mut tx) = channel {
                tx.unbounded_send(chunk.into())?;
            }
        } else {
            match channel {
                Ok(_) => return Ok(()),
                Err(ref mut rx) => {
                    let tx = rx.await?;
                    tx.unbounded_send(buf.into())?;
                    return Ok(());
                }
            }
        };
    }
}

#[derive(Deserialize)]
struct PromptServiceInput<'a> {
    pub text: &'a str,
    pub chat_id: Option<i32>,
}

#[derive(Clone)]
pub struct PromptService {
    cx: Arc<Context>,
}

impl PromptService {
    pub fn new(cx: Arc<Context>) -> Self {
        PromptService { cx }
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
