use hyper::Response;
use rgpt_db::{session::Session, user::User};

use crate::{
    cfg::Cfg,
    server::{form_stream_body, IncReqst, OutResp},
};

pub async fn api_def_sess(cfg: &Cfg, req: IncReqst) -> OutResp {
    api_def_sess_inner(cfg, req).await.unwrap_or_else(|x| x)
}

pub async fn api_def_sess_inner(cfg: &Cfg, _req: IncReqst) -> Result<OutResp, OutResp> {
    let default_user = User::default(&cfg.db_url).await.unwrap();
    let session = Session::get_session_for_user(&cfg.db_url, default_user)
        .await
        .unwrap();

    let stream = futures::stream::once(async move {
        Ok(hyper::body::Frame::data(hyper::body::Bytes::from(
            session.session_token,
        )))
    });
    let body = form_stream_body(Box::pin(stream));

    Response::builder()
        .status(hyper::StatusCode::OK)
        .body(body)
        .map_err(|_| crate::server::error::error_500())
}
