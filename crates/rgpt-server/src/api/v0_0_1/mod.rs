use std::sync::Arc;

use libserver::{DynRoute, PathPrefixRouter, Route, ServiceBuilder, NOT_FOUND};
use rgpt_cfg::Context;

pub mod auth;

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathPrefixRouter::new("/api/v0.0.1");

    let service = ServiceBuilder::new()
        .with_dyn_route(auth::route(cx.clone()))
        .with_fallback(NOT_FOUND);

    Route::from_parts(router, service).make_dyn()
}
