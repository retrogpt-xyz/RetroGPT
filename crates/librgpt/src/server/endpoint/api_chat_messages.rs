use std::convert::identity;

use futures::stream;
use http_body_util::BodyExt;
use hyper::{body::Body, header::CONTENT_TYPE, Response, StatusCode};
use rgpt_db::{chat::Chat, msg::Msg, session::Session};
use serde_json::{json, Value};

use crate::{
    cfg::Cfg,
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

    let session = Session::get_by_token(&cfg.db_url, session_token.to_string())
        .await
        .map_err(|_| error_400("bad token"))?;

    let bytes = req
        .collect()
        .await
        .map_err(|_| error_500())?
        .to_bytes()
        .to_vec();

    let chat_id = String::from_utf8_lossy(&bytes);
    let chat_id = chat_id.as_ref().parse().map_err(|_| error_500())?;

    let chat = Chat::get_by_id(&cfg.db_url, chat_id)
        .await
        .map_err(|_| error_400("bad chad id"))?;

    if session.user_id != chat.user_id {
        return Err(error_400("unauthorized access to chat"));
    }

    let head_msg = Msg::get_by_id(&cfg.db_url, chat.head_msg.unwrap())
        .await
        .map_err(|_| error_500())?;
    let messages = Msg::get_msg_chain(head_msg, &cfg.db_url)
        .await
        .map_err(|_| error_500())?;

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
