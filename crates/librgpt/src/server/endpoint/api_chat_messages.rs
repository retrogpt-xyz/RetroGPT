use std::convert::identity;

use futures::stream;
use http_body_util::BodyExt;
use hyper::{body::Body, header::CONTENT_TYPE, Response, StatusCode};
use serde_json::{json, Value};

use crate::{
    cfg::Cfg,
    db,
    server::{
        error::{error_400, error_500},
        form_stream_body, IncReqst, OutResp,
    },
};

pub async fn api_chat_messages(cfg: &Cfg, req: IncReqst) -> OutResp {
    api_chat_messages_inner(cfg, req)
        .await
        .unwrap_or_else(identity)
}

pub async fn api_chat_messages_inner(cfg: &Cfg, req: IncReqst) -> Result<OutResp, OutResp> {
    if req.body().size_hint().upper().unwrap_or(u64::MAX) > cfg.max_req_size {
        return Err(error_400("request body is too large"));
    }

    let session_token = match req.headers().get("X-Session-Token") {
        Some(s) => s.to_str().map_err(|_| error_500())?,
        None => return Err(error_400("no session token provided")),
    };

    let mut conn = db::make_conn().await;
    let session = match db::sessions::get_session_by_token(&mut conn, session_token).await {
        Some(sess) => sess,
        None => return Err(error_400("invalid session token")),
    };

    // Get the chat ID from the request body
    let bytes = req
        .collect()
        .await
        .map_err(|_| error_500())?
        .to_bytes()
        .to_vec();

    let chat_id = String::from_utf8_lossy(&bytes);
    let chat_id = chat_id.as_ref().parse().map_err(|_| error_500())?;

    // Get the chat
    let chat = db::dep_chats::get_chat_by_id(&mut conn, chat_id).await;

    // Validate that the session user owns the chat
    if session.user_id != chat.user_id {
        return Err(error_400("unauthorized access to chat"));
    }

    // Get all messages in the chat's message tree
    let head_msg = db::msgs::get_msg_by_id(&mut conn, chat.head_msg).await;
    let mut messages = db::msgs::get_all_parents(&mut conn, head_msg).await;
    messages.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    // Transform messages to only include body and sender
    let messages: Vec<Value> = messages
        .into_iter()
        .map(|msg| {
            json!({
                "text": msg.body,
                "sender": msg.sender
            })
        })
        .collect();

    let stream = stream::once(async move {
        Ok(hyper::body::Frame::data(hyper::body::Bytes::from(
            json!(messages).to_string(),
        )))
    });

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(form_stream_body(Box::pin(stream)))
        .map_err(|_| error_500())
}
