use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use api::default_session::get_default_session_route;
use hyper::StatusCode;
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
        .with_fallback(StaticService::new("404 not found", StatusCode::NOT_FOUND))
        .serve(listener)
        .await?;

    Ok(())
}

fn static_asset_route(path: PathBuf) -> DynRoute {
    Route::from_parts(StaticDirRouter::new(&path), StaticAssetService::new(&path)).make_dyn()
}
