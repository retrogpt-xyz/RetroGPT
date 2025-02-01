use hyper::Response;

use crate::{
    cfg::Cfg,
    server::{form_stream_body, IncReqst, OutResp},
};

pub async fn api_def_sess(cfg: &Cfg, req: IncReqst) -> OutResp {
    api_def_sess_inner(cfg, req).await.unwrap_or_else(|x| x)
}

pub async fn api_def_sess_inner(cfg: &Cfg, _req: IncReqst) -> Result<OutResp, OutResp> {
    let mut conn = cfg.db_conn.lock().await;
    let def = crate::db::users::get_default_user(&mut conn).await;
    let session = crate::db::sessions::get_session(&mut conn, &def).await;

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
