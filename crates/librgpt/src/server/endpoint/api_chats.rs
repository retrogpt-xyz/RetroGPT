use std::convert::identity;

use http_body_util::BodyExt;
use hyper::{body::Body, Response, StatusCode};
use rgpt_db::{session::Session, user::User};
use serde_json::json;

use crate::{
    cfg::Cfg,
    server::{
        error::{error_400, error_500},
        form_stream_body, IncReqst, OutResp,
    },
};

pub async fn api_chats(cfg: &Cfg, req: IncReqst) -> OutResp {
    api_chats_inner(cfg, req).await.unwrap_or_else(identity)
}

pub async fn api_chats_inner(cfg: &Cfg, req: IncReqst) -> Result<OutResp, OutResp> {
    let chats_lock = cfg.chts_mutex.lock().await;
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

    // Get the user ID from the request body
    let bytes = req
        .collect()
        .await
        .map_err(|_| error_500())?
        .to_bytes()
        .to_vec();

    let user_id = String::from_utf8_lossy(&bytes);
    let user_id = user_id.as_ref().parse().map_err(|_| error_500())?;

    // Validate that the session belongs to the requested user
    if session.user_id != user_id {
        return Err(error_400("session token does not match requested user"));
    }

    // Get the default user
    let default_user = User::default(&cfg.db_url).await.unwrap();

    let chat_ids = if user_id == default_user.user_id {
        Vec::new()
    } else {
        User::get_by_id(&cfg.db_url, user_id)
            .await
            .map_err(|_| error_500())?
            .get_chats(&cfg.db_url)
            .await
            .map_err(|_| error_500())?
            .into_iter()
            .map(|chat| {
                (
                    chat.id,
                    match chat.name {
                        Some(name) => name,
                        None => "Untitled Chat".into(),
                    },
                )
            })
            .map(|(id, name)| json!({"id": id, "name": name}))
            .collect::<Vec<_>>()
    };

    // Create a stream with the JSON response
    let json_response = json!(chat_ids).to_string();
    let stream = futures::stream::once(async move {
        Ok(hyper::body::Frame::data(hyper::body::Bytes::from(
            json_response,
        )))
    });
    let body = form_stream_body(Box::pin(stream));

    drop(chats_lock);

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|_| error_500())
}
