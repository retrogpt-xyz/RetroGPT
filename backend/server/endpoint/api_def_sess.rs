// use hyper::Response;

// use crate::{
// cfg::Cfg,
// server::{form_body, IncReqst, RGptResp},
// };

// pub async fn api_def_sess(cfg: &Cfg, req: IncReqst) -> RGptResp {
// api_def_sess_inner(cfg, req).await.unwrap_or_else(|x| x)
// }

// pub async fn api_def_sess_inner(cfg: &Cfg, _req: IncReqst) -> Result<RGptResp, RGptResp> {
// let mut conn = cfg.db_conn.lock().await;
// let def = crate::db::users::get_default_user(&mut conn).await;
// let session = crate::db::sessions::get_session(&mut conn, &def).await;
//
// Ok(Response::new(form_body(session.session_token)))
// }
