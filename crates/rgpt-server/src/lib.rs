use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use api::{default_session::get_default_session_route, user_session::get_user_session_route};
use http_body_util::BodyExt;
use hyper::{body::Body, StatusCode};
use libserver::{DynRoute, Route, ServiceBuilder, StaticDirRouter, StaticService};
use rgpt_cfg::Context;
use tokio::net::TcpListener;

pub mod api;
pub mod serve_static;

use serve_static::StaticAssetService;

pub async fn run_server(cx: Arc<Context>) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], cx.port()));
    let listener = TcpListener::bind(addr).await?;

    ServiceBuilder::new()
        .with_dyn_route(static_asset_route(cx.static_dir()))
        .with_dyn_route(get_default_session_route(cx.db()))
        .with_dyn_route(get_user_session_route(cx.clone()))
        .with_fallback(StaticService::new("404 not found", StatusCode::NOT_FOUND))
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

#[derive(Debug, thiserror::Error)]
#[error("Request Too Large")]
pub struct RequestTooLarge;
