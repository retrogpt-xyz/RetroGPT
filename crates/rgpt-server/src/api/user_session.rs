use std::sync::Arc;

use hyper::{Response, StatusCode};
use libserver::{single_frame_body, DynRoute, PathEqRouter, Route};
use rgpt_cfg::Context;
use rgpt_db::{session::Session, user::User};

pub fn get_user_session_route(cx: Arc<Context>) -> DynRoute {
    Route::from_parts(PathEqRouter::new("/api/session"), GetUserSession::new(cx)).make_dyn()
}

#[derive(Clone)]
pub struct GetUserSession {
    cx: Arc<Context>,
}

impl GetUserSession {
    pub fn new(cx: Arc<Context>) -> Self {
        Self { cx }
    }
}

impl tower::Service<libserver::Request> for GetUserSession {
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
        Box::pin(async move { get_user_session(req, cx).await })
    }
}

pub async fn get_user_session(
    req: libserver::Request,
    cx: Arc<Context>,
) -> Result<libserver::ServiceResponse, libserver::ServiceError> {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let user_id = crate::collect_body_string(req).await?.parse::<i32>()?;
    let user = &User::n_get_by_id(cx.db(), user_id).await?;
    let session = Session::n_get_for_user(cx.db(), user).await?;

    let resp = Response::builder()
        .status(StatusCode::OK)
        .body(single_frame_body(session.session_token))?;
    Ok(resp)
}
