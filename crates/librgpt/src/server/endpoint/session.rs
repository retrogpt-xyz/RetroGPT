use std::convert::identity;

use futures::stream;
use http_body_util::BodyExt;
use hyper::{
    body::{Body, Bytes, Frame},
    Response, StatusCode,
};
use rgpt_db::{session::Session, user::User};

use crate::{
    cfg::Cfg,
    server::{
        error::{error_400, error_500},
        form_stream_body, IncReqst, OutResp,
    },
};

pub async fn session(cfg: &Cfg, req: IncReqst) -> OutResp {
    session_inner(cfg, req).await.unwrap_or_else(identity)
}

pub async fn session_inner(cfg: &Cfg, req: IncReqst) -> Result<OutResp, OutResp> {
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

    let user = User::get_by_id(
        &cfg.db_url,
        recvd.parse().map_err(|_| error_400("bad user id"))?,
    )
    .await
    .map_err(|_| error_500())?;

    let session = Session::get_session_for_user(&cfg.db_url, user)
        .await
        .map_err(|_| error_500())?;

    let stream =
        stream::once(
            async move { Ok(Frame::data(Bytes::from(session.session_token.to_string()))) },
        );

    let body = form_stream_body(Box::pin(stream));

    Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .map_err(|_| error_500())
}
