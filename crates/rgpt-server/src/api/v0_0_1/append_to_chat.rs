use std::sync::Arc;

use hyper::Response;
use libserver::{DynRoute, PathEqRouter, Request, Route, single_frame_body};
use rgpt_cfg::Context;
use rgpt_db::{chat::Chat, msg::Msg};
use serde::Deserialize;

use crate::{collect_body_string, validate_session};

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathEqRouter::new("/api/v0.0.1/append_to_chat");

    Route::from_parts(router, AppendToChatService::new(cx)).make_dyn()
}

pub async fn append_to_chat(req: Request, cx: Arc<Context>) -> libserver::ServiceResult {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let headers = req.headers().to_owned();
    validate_session(cx.db(), &headers, None).await?;

    let body = collect_body_string(req).await?;

    let AppendToChatInput {
        sender,
        body,
        chat_id,
    } = serde_json::from_str(&body)?;

    let chat = Chat::get_by_id(cx.db(), chat_id).await?;

    let msg = Msg::create(cx.db(), body, sender, chat.user_id, chat.head_msg).await?;

    chat.append_to_chat(cx.db(), &msg).await?;

    Ok(Response::new(single_frame_body("")))
}

#[derive(Deserialize)]
struct AppendToChatInput<'a> {
    sender: &'a str,
    body: &'a str,
    chat_id: i32,
}

#[derive(Clone)]
pub struct AppendToChatService {
    cx: Arc<Context>,
}

impl AppendToChatService {
    pub fn new(cx: Arc<Context>) -> Self {
        AppendToChatService { cx }
    }
}

impl tower::Service<libserver::Request> for AppendToChatService {
    type Response = libserver::ServiceResponse;
    type Error = libserver::ServiceError;
    type Future = libserver::ServiceBoxFuture;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: libserver::Request) -> Self::Future {
        let cx = self.cx.clone();
        Box::pin(async move { append_to_chat(req, cx).await })
    }
}
