use std::sync::Arc;

use hyper::Response;
use libserver::{DynRoute, PathEqRouter, Request, Route, single_frame_body};
use rgpt_cfg::Context;
use rgpt_db::chat::Chat;
use serde::Deserialize;
use serde_json::json;

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathEqRouter::new("/api/v0.0.1/chat_msgs");

    Route::from_parts(router, ChatMsgService::new(cx)).make_dyn()
}

pub async fn chat_msgs(req: Request, cx: Arc<Context>) -> libserver::ServiceResult {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let headers = req.headers().to_owned();
    let body = crate::collect_body_string(req).await?;

    let ChatMsgServiceInput { chat_id } = serde_json::from_str(&body)?;
    let chat = Chat::get_by_id(cx.db(), chat_id).await?;
    let _session = crate::validate_session_header(cx.db(), &headers, Some(chat.user_id)).await?;

    let msgs = chat.msg_chain(cx.db()).await?;

    let fmted_mgs = json!(
        msgs.into_iter()
            .map(|msg| {
                json!({
                    "text": msg.body,
                    "sender": msg.sender
                })
            })
            .collect::<Vec<_>>()
    )
    .to_string();

    let body = single_frame_body(fmted_mgs);
    Ok(Response::new(body))
}

#[derive(Deserialize)]
struct ChatMsgServiceInput {
    chat_id: i32,
}

#[derive(Clone)]
struct ChatMsgService {
    cx: Arc<Context>,
}

impl ChatMsgService {
    fn new(cx: Arc<Context>) -> Self {
        ChatMsgService { cx }
    }
}

impl tower::Service<libserver::Request> for ChatMsgService {
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
        Box::pin(async move { chat_msgs(req, cx).await })
    }
}
