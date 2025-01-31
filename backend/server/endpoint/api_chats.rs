use std::convert::identity;

use http_body_util::BodyExt;
use hyper::{body::Body, Response, StatusCode};
use serde_json::json;

use crate::{
    cfg::Cfg,
    db::{
        self,
        users::{get_default_user, get_user_by_id, get_users_chats},
    },
    server::{
        error::{error_400, error_500},
        form_stream_body, IncReqst, OutResp,
    },
};

pub async fn api_chats(cfg: &Cfg, req: IncReqst) -> OutResp {
    api_chats_inner(cfg, req).await.unwrap_or_else(identity)
}

pub async fn api_chats_inner(cfg: &Cfg, req: IncReqst) -> Result<OutResp, OutResp> {
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
    let default_user = get_default_user(&mut conn).await;

    let chat_ids = if user_id == default_user.user_id {
        Vec::new()
    } else {
        let user = get_user_by_id(&mut conn, user_id).await;
        get_users_chats(&mut conn, &user)
            .await
            .into_iter()
            .map(|chat| json!({"id": chat.id, "name": chat.name}))
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

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|_| error_500())
}
