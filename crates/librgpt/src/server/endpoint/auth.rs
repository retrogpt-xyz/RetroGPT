use std::convert::identity;

use diesel::{prelude::Insertable, Selectable};
use futures::stream;
use http_body_util::BodyExt;
use hyper::{
    body::{Body, Bytes, Frame},
    Response, StatusCode,
};
use serde::Deserialize;

use rgpt_db::{schema::users, user::User};

use crate::{
    cfg::Cfg,
    server::{
        error::{error_400, error_500},
        form_stream_body, IncReqst, OutResp,
    },
};

pub async fn auth(cfg: &Cfg, req: IncReqst) -> OutResp {
    auth_inner(cfg, req).await.unwrap_or_else(identity)
}

#[derive(Deserialize, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct DesUser {
    google_id: String,
    email: String,
    name: String,
}

pub async fn auth_inner(cfg: &Cfg, req: IncReqst) -> Result<OutResp, OutResp> {
    if req.body().size_hint().upper().unwrap_or(u64::MAX) > cfg.max_req_size {
        return Err(error_400("request body is too large"));
    }

    let bytes = req
        .collect()
        .await
        .map_err(|_| error_500())?
        .to_bytes()
        .to_vec();

    let recvd = std::str::from_utf8(&bytes).map_err(|_| error_500())?;
    let parsed: DesUser = serde_json::from_str(recvd).map_err(|_| error_500())?;

    let existing_user = User::get_by_google_id(&cfg.db_url, &parsed.google_id)
        .await
        .ok();

    if let Some(user) = existing_user {
        let stream =
            stream::once(async move { Ok(Frame::data(Bytes::from(user.user_id.to_string()))) });
        let body = form_stream_body(Box::pin(stream));

        return Response::builder()
            .status(StatusCode::OK)
            .body(body)
            .map_err(|_| error_500());
    }

    // If user does not exist, insert a new user
    let user = User::create(&cfg.db_url, parsed.google_id, parsed.email, parsed.name)
        .await
        .map_err(|_| error_500())?;

    let stream =
        stream::once(async move { Ok(Frame::data(Bytes::from(user.user_id.to_string()))) });
    let body = form_stream_body(Box::pin(stream));

    Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .map_err(|_| error_500())
}
