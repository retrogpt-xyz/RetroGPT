use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use http_body_util::BodyExt;
use hyper::{HeaderMap, body::Body};
use libserver::{DynRoute, NOT_FOUND, Route, ServiceBuilder, StaticDirRouter};
use rgpt_cfg::Context;
use rgpt_db::{Database, session::Session, user::User};
use tokio::net::TcpListener;

pub mod api;
pub mod serve_static;

use serve_static::StaticAssetService;

pub fn static_asset_service(cx: Arc<Context>) -> libserver::Service {
    ServiceBuilder::new()
        .with_dyn_route(static_asset_route(cx.static_dir()))
        .with_fallback(NOT_FOUND)
}

pub fn api_service(cx: Arc<Context>) -> libserver::Service {
    ServiceBuilder::new()
        .with_dyn_route(api::v0_0_1::route(cx))
        .with_fallback(NOT_FOUND)
}

pub async fn run_server(cx: Arc<Context>) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], cx.port()));
    let listener = TcpListener::bind(addr).await?;

    ServiceBuilder::new()
        .with_dyn_route(static_asset_route(cx.static_dir()))
        .with_dyn_route(api::v0_0_1::route(cx.clone()))
        .with_fallback(NOT_FOUND)
        .serve(listener)
        .await?;

    Ok(())
}

pub fn static_asset_route(path: PathBuf) -> DynRoute {
    Route::from_parts(StaticDirRouter::new(&path), StaticAssetService::new(&path)).make_dyn()
}

pub fn check_body_size(req: &libserver::Request, max_size: u64) -> Result<(), RequestTooLarge> {
    if req.body().size_hint().upper().unwrap_or(u64::MAX) > max_size {
        return Err(RequestTooLarge);
    }
    Ok(())
}

pub async fn collect_body_bytes(
    req: libserver::Request,
) -> Result<Vec<u8>, libserver::ServiceError> {
    let bytes = req.collect().await?.to_bytes().to_vec();
    Ok(bytes)
}

pub async fn collect_body_string(
    req: libserver::Request,
) -> Result<String, libserver::ServiceError> {
    let bytes = collect_body_bytes(req).await?;
    let string = String::from_utf8(bytes)?;
    Ok(string)
}

pub async fn validate_session_header(
    db: Arc<Database>,
    headers: &HeaderMap,
    user_id: Option<i32>,
) -> Result<Session, libserver::ServiceError> {
    let session_token = match headers.get("X-Session-Token") {
        Some(token) => token.to_str()?.to_owned(),
        None => Err(InvalidSessionTokenHeader)?,
    };

    validate_session_token(db, session_token, user_id).await
}

pub fn extract_query_param(uri: &hyper::Uri, param_name: &str) -> Option<String> {
    uri.query().and_then(|query| {
        query
            .split('&')
            .find_map(|pair| {
                let mut parts = pair.split('=');
                if parts.next() == Some(param_name) {
                    parts
                        .next()
                        .map(|value| urlencoding::decode(value).ok().map(|v| v.into_owned()))
                } else {
                    None
                }
            })
            .flatten()
    })
}

pub async fn validate_session_token(
    db: Arc<Database>,
    session_token: String,
    user_id: Option<i32>,
) -> Result<Session, libserver::ServiceError> {
    if session_token == "__default__" {
        let default_user = User::default(db.clone()).await?;
        let session = Session::get_for_user(db.clone(), &default_user).await?;
        return Ok(session);
    }

    let session = Session::get_by_token(db.clone(), session_token).await?;

    if let Some(user_id) = user_id {
        if session.user_id != user_id {
            Err(InvalidSessionTokenHeader)?;
        };
    }

    if !session.validate() {
        session.delete(db.clone()).await?;
        Err(InvalidSessionTokenHeader)?
    } else {
        Ok(session)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Request Too Large")]
pub struct RequestTooLarge;

#[derive(Debug, thiserror::Error)]
#[error("Invalid Session Token Header")]
pub struct InvalidSessionTokenHeader;
