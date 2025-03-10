use std::sync::Arc;

use libserver::{single_frame_body, DynRoute, PathEqRouter, Route};
use rgpt_cfg::Context;
use rgpt_db::{session::Session, user::User};
use serde::{Deserialize, Serialize};

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathEqRouter::new("/api/v0.0.1/auth");

    Route::from_parts(router, AuthService::new(cx)).make_dyn()
}

pub async fn auth(req: libserver::Request, cx: Arc<Context>) -> libserver::ServiceResult {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let body = crate::collect_body_string(req).await?;

    let AuthServiceInput { user_access_token } = serde_json::from_str(&body)?;

    let user_info_response = cx
        .state
        .reqwest_client
        .get(format!(
            "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
            user_access_token
        ))
        .header("Authorization", format!("Bearer {}", user_access_token))
        .header("Accept", "application/json")
        .send()
        .await?;

    // TODO: Better solution to this???
    debug_assert!(user_info_response.status() == 200);

    let GoogleUserInfo {
        google_id,
        email,
        name,
    } = user_info_response.json::<GoogleUserInfo>().await?;

    let user = if let Ok(user) = User::n_get_by_google_id(cx.db(), &google_id).await {
        user
    } else {
        User::n_create(cx.db(), google_id, email, name).await?
    };

    let session = Session::n_get_for_user(cx.db(), &user).await?;

    let return_body = AuthServiceReturn::new(&session.session_token);

    Ok(hyper::Response::new(single_frame_body(
        serde_json::to_string(&return_body)?,
    )))
}

#[derive(Serialize)]
struct AuthServiceReturn<'a> {
    session_token: &'a str,
}

impl<'a> AuthServiceReturn<'a> {
    fn new(session_token: &'a str) -> Self {
        AuthServiceReturn { session_token }
    }
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    google_id: String,
    email: String,
    name: String,
}

#[derive(Clone)]
struct AuthService {
    cx: Arc<Context>,
}

impl AuthService {
    fn new(cx: Arc<Context>) -> Self {
        AuthService { cx }
    }
}

impl tower::Service<libserver::Request> for AuthService {
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
        Box::pin(async move { auth(req, cx).await })
    }
}

#[derive(Deserialize)]
struct AuthServiceInput<'a> {
    user_access_token: &'a str,
}
