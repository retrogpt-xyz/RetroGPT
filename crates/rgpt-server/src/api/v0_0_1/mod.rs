use std::sync::Arc;

use libserver::{DynRoute, NOT_FOUND, PathPrefixRouter, Route, ServiceBuilder};
use rgpt_cfg::Context;

pub mod attach;
pub mod auth;
pub mod chat_msgs;
pub mod prompt;
pub mod user_chats;

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathPrefixRouter::new("/api/v0.0.1");

    let service = ServiceBuilder::new()
        .with_dyn_route(auth::route(cx.clone()))
        .with_dyn_route(chat_msgs::route(cx.clone()))
        .with_dyn_route(user_chats::route(cx.clone()))
        .with_dyn_route(prompt::route(cx.clone()))
        .with_dyn_route(attach::route(cx.clone()))
        .with_fallback(NOT_FOUND);

    Route::from_parts(router, service).make_dyn()
}
