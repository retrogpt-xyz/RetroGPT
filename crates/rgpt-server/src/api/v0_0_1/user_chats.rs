use std::sync::Arc;

use hyper::Response;
use libserver::{DynRoute, PathEqRouter, Request, Route, single_frame_body};
use rgpt_cfg::Context;
use rgpt_db::user::User;
use serde::Deserialize;
use serde_json::json;

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathEqRouter::new("/api/v0.0.1/user_chats");

    Route::from_parts(router, UserChatsService::new(cx)).make_dyn()
}

pub async fn user_chats(req: Request, cx: Arc<Context>) -> libserver::ServiceResult {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let body = crate::collect_body_string(req).await?;

    let UserChatsServiceInput { user_id } = serde_json::from_str(&body)?;
    let user = User::get_by_id(cx.db(), user_id).await?;

    let chats = if user.user_id == 1 {
        vec![]
    } else {
        user.get_chats(cx.db()).await?
    };

    let fmted_chats = json!(
        chats
            .into_iter()
            .map(|chat| {
                json!({
                    "id": chat.id,
                    "name": chat.name.unwrap_or("Untitled Chat".into())
                })
            })
            .collect::<Vec<_>>()
    )
    .to_string();

    let body = single_frame_body(fmted_chats);
    Ok(Response::new(body))
}

#[derive(Deserialize)]
struct UserChatsServiceInput {
    user_id: i32,
}

#[derive(Clone)]
pub struct UserChatsService {
    cx: Arc<Context>,
}

impl UserChatsService {
    pub fn new(cx: Arc<Context>) -> Self {
        UserChatsService { cx }
    }
}

impl tower::Service<libserver::Request> for UserChatsService {
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
        Box::pin(async move { user_chats(req, cx).await })
    }
}
