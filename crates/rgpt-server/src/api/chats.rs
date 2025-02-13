use std::sync::Arc;

use hyper::{Response, StatusCode};
use libserver::{single_frame_body, DynRoute, PathEqRouter, Route};
use rgpt_cfg::Context;
use rgpt_db::user::User;
use serde_json::json;

pub fn get_user_chats_route(cx: Arc<Context>) -> DynRoute {
    Route::from_parts(PathEqRouter::new("/api/chats"), GetUserChats::new(cx)).make_dyn()
}

#[derive(Clone)]
pub struct GetUserChats {
    cx: Arc<Context>,
}

impl GetUserChats {
    pub fn new(cx: Arc<Context>) -> Self {
        Self { cx }
    }
}

impl tower::Service<libserver::Request> for GetUserChats {
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
        Box::pin(async move { get_user_chats(req, cx).await })
    }
}

pub async fn get_user_chats(
    req: libserver::Request,
    cx: Arc<Context>,
) -> Result<libserver::ServiceResponse, libserver::ServiceError> {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let headers = req.headers().to_owned();
    let body = crate::collect_body_string(req).await?;
    let user_id = body.parse::<i32>()?;
    let session = crate::validate_session(cx.db(), &headers, Some(user_id)).await?;

    let user = User::n_get_by_id(cx.db(), user_id).await?;

    let mut user_chats: Vec<serde_json::Value> = vec![];

    if session.user_id != 1 {
        user_chats.extend(user.n_get_chats(cx.db()).await?.into_iter().map(|chat| {
            json!({
                "id": chat.id,
                "name": chat.name.unwrap_or("Untitled Chat".into())
            })
        }))
    }

    let resp = Response::builder()
        .status(StatusCode::OK)
        .body(single_frame_body(json!(user_chats).to_string()))?;
    Ok(resp)
}
