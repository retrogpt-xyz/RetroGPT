use std::sync::Arc;

use hyper::Response;
use libserver::{DynRoute, PathEqRouter, Request, Route, single_frame_body};
use rgpt_cfg::Context;
use rgpt_db::chat::Chat;
use serde::Deserialize;

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathEqRouter::new("/api/v0.0.1/delete_chat");

    Route::from_parts(router, DeleteChatService::new(cx)).make_dyn()
}

pub async fn delete_chat(req: Request, cx: Arc<Context>) -> libserver::ServiceResult {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let headers = req.headers().to_owned();
    let body = crate::collect_body_string(req).await?;

    let DeleteChatInput { chat_id } = serde_json::from_str(&body)?;

    let chat = Chat::get_by_id(cx.db(), chat_id).await?;

    crate::validate_session_header(cx.db(), &headers, Some(chat.user_id)).await?;

    chat.delete(cx.db()).await?;

    Ok(Response::new(single_frame_body("")))
}

#[derive(Clone)]
pub struct DeleteChatService {
    cx: Arc<Context>,
}

#[derive(Deserialize)]
struct DeleteChatInput {
    chat_id: i32,
}

impl DeleteChatService {
    pub fn new(cx: Arc<Context>) -> Self {
        DeleteChatService { cx }
    }
}

impl tower::Service<libserver::Request> for DeleteChatService {
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
        Box::pin(async move { delete_chat(req, cx).await })
    }
}
