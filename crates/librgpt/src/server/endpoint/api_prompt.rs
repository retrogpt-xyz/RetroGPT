use std::convert::identity;

use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use futures::StreamExt;
use http_body_util::BodyExt;
use hyper::{
    body::{Body, Bytes, Frame},
    Response, StatusCode,
};
use rgpt_db::{chat::Chat, msg::Msg, session::Session};

use crate::{
    cfg::Cfg,
    server::{
        error::{error_400, error_500},
        IncReqst, OutResp,
    },
};

pub async fn api_prompt(cfg: &Cfg, req: IncReqst) -> OutResp {
    api_prompt_inner(cfg, req).await.unwrap_or_else(identity)
}

pub async fn api_prompt_inner(cfg: &Cfg, req: IncReqst) -> Result<OutResp, OutResp> {
    if req.body().size_hint().upper().unwrap_or(u64::MAX) > cfg.max_req_size {
        return Err(error_400("request body is too large"));
    }

    let session_token = match req.headers().get("X-Session-Token") {
        Some(s) => s.to_str().map_err(|_| error_500())?,
        None => return Err(error_400("no session token provided")),
    };

    let session = Session::get_by_token(&cfg.db_url, session_token.to_string())
        .await
        .map_err(|_| error_500())?;

    let bytes = req
        .collect()
        .await
        .map_err(|_| error_500())?
        .to_bytes()
        .to_vec();

    let prompt = std::str::from_utf8(&bytes).map_err(|_| error_500())?;

    let recv_json: crate::gpt::BackendQueryMsg<'_> =
        serde_json::from_str(prompt).ok().ok_or_else(error_500)?;

    if let Some(chat_id) = recv_json.chatId {
        let chat = Chat::get_by_id(&cfg.db_url, chat_id)
            .await
            .map_err(|_| error_500())?;

        if session.user_id != chat.user_id {
            return Err(error_400("session user ID does not match chat user ID"));
        }
    }

    let (msg_chain, chat_id) = get_msgs(cfg, recv_json, session.user_id).await;

    let client = async_openai::Client::with_config(
        async_openai::config::OpenAIConfig::new().with_api_key(&cfg.api_key),
    );

    // Create a system prompt
    let system_prompt = ChatCompletionRequestSystemMessageArgs::default()
        .content(cfg.system_message.clone())
        .build()
        .unwrap()
        .into();

    // Map over the msg_chain to create messages for the model
    let messages: Vec<_> = msg_chain
        .into_iter()
        .filter_map(|msg| match msg.sender.as_str() {
            "ai" => Some(
                ChatCompletionRequestAssistantMessageArgs::default()
                    .content(msg.body)
                    .build()
                    .unwrap()
                    .into(),
            ),
            "user" => Some(
                ChatCompletionRequestUserMessageArgs::default()
                    .content(msg.body)
                    .build()
                    .unwrap()
                    .into(),
            ),
            _ => None,
        })
        .collect();

    let mut messages_with_system = vec![system_prompt];
    messages_with_system.extend(messages);

    let request = CreateChatCompletionRequestArgs::default()
        .model(&cfg.model_name)
        .max_tokens(cfg.max_tokens)
        .messages(messages_with_system)
        .build()
        .unwrap();

    let (stream_tx, mut rx) = futures::channel::mpsc::unbounded::<String>();

    // let (body_tx, msg) = crate::db::msgs::create_placeholder_msg(
    // &mut conn,
    // "ai",
    // session.user_id,
    // Some(new_head_id),
    // )
    // .await;

    let resp_stream = client
        .chat()
        .create_stream(request)
        .await
        .unwrap()
        .filter_map(|c| async move {
            c.ok()
                .and_then(|x| x.choices.into_iter().next())
                .and_then(|x| x.delta.content)
        })
        .inspect(move |x| stream_tx.unbounded_send(x.clone()).unwrap())
        .map(|x| Ok(Frame::data(Bytes::from(x))));

    // let _ = add_to_chat(&mut conn, &chat, &msg).await;
    let chat = Chat::get_by_id(&cfg.db_url, chat_id)
        .await
        .map_err(|_| error_500())?;

    let url = cfg.db_url.clone();
    let lock = cfg.msgs_mutex.clone();
    tokio::spawn(async move {
        let lock = lock.lock().await;
        let mut resp = String::new();
        while let Some(x) = rx.next().await {
            resp.push_str(&x);
        }
        println!("{resp}");
        let msg = Msg::create(
            &url,
            resp.clone(),
            "ai".into(),
            session.user_id,
            chat.head_msg,
        )
        .await
        .unwrap();
        let chat = Chat::get_by_id(&url, chat.id).await.unwrap();
        chat.append_to_chat(&url, msg).await.unwrap();
        println!("appended ai msg to db");
        drop(lock);
        // body_tx.send(resp).unwrap();
    });

    Response::builder()
        .status(StatusCode::OK)
        .header("X-Chat-ID", chat_id.to_string())
        .body(crate::server::form_stream_body(Box::pin(resp_stream)))
        .map_err(|_| error_500())
}

async fn get_msgs(
    cfg: &Cfg,
    recvd: crate::gpt::BackendQueryMsg<'_>,
    user_id: i32,
) -> (Vec<Msg>, i32) {
    let (chat, msg) = match recvd.chatId {
        Some(id) => {
            println!("I received a chat id reference of {}", id);
            // let chat = get_chat_by_id(&mut conn, id).await;
            let chat = Chat::get_by_id(&cfg.db_url, id).await.unwrap();
            // let msg = create_msg(&mut conn, &recvd.text, "user", user_id, chat.head_msg).await;
            let msg = Msg::create(
                &cfg.db_url,
                recvd.text.into(),
                "user".into(),
                user_id,
                chat.head_msg,
            )
            .await
            .unwrap();
            // let chat = add_to_chat(&mut conn, &chat, &msg).await;
            let chat = chat.append_to_chat(&cfg.db_url, msg.clone()).await.unwrap();
            println!("appended user msg to db");
            (chat, msg)
        }
        None => {
            let msg = Msg::create(&cfg.db_url, recvd.text.into(), "user".into(), user_id, None)
                .await
                .unwrap();
            let chat = Chat::create(&cfg.db_url, msg.user_id, "Untitled Chat".into())
                .await
                .unwrap();
            let chat = chat.append_to_chat(&cfg.db_url, msg.clone()).await.unwrap();
            println!("appended user msg to db");
            // let chat = create_chat(&mut conn, &msg).await;
            println!("Created new DB chat instance with id {}", chat.id);
            (chat, msg)
        }
    };

    // println!("new head id is {}", new_head_id.unwrap());
    // println!("created measage id is {}", msg.id);

    // let msg_chain = crate::db::msgs::get_all_parents(&mut conn, msg).await;
    let msg_chain = msg.get_msg_chain(&cfg.db_url).await.unwrap();

    (msg_chain, chat.id)
}
