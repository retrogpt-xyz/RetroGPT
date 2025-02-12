use std::sync::Arc;

use diesel::{prelude::Insertable, Selectable};
use hyper::{Response, StatusCode};
use libserver::{static_body, DynRoute, PathEqRouter, Route};
use rgpt_cfg::Context;
use rgpt_db::user::User;
use serde::Deserialize;

pub fn auth_route(cx: Arc<Context>) -> DynRoute {
    Route::from_parts(PathEqRouter::new("/api/auth"), GetAuth::new(cx)).make_dyn()
}

#[derive(Clone)]
pub struct GetAuth {
    cx: Arc<Context>,
}

impl GetAuth {
    pub fn new(cx: Arc<Context>) -> Self {
        Self { cx }
    }
}

impl tower::Service<libserver::Request> for GetAuth {
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
        Box::pin(async move { authenticate(req, cx).await })
    }
}

#[derive(Deserialize, Selectable, Insertable)]
#[diesel(table_name = rgpt_db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserInfoDes {
    google_id: String,
    email: String,
    name: String,
}

pub async fn authenticate(
    req: libserver::Request,
    cx: Arc<Context>,
) -> Result<libserver::ServiceResponse, libserver::ServiceError> {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let body = crate::collect_body_string(req).await?;
    let UserInfoDes {
        google_id,
        email,
        name,
    } = serde_json::from_str(&body)?;

    let user = match User::n_get_by_google_id(cx.db(), &google_id).await {
        Ok(user) => user,
        // TODO: Match on not found
        Err(_) => User::n_create(cx.db(), google_id, email, name).await?,
    };

    let resp = Response::builder()
        .status(StatusCode::OK)
        .body(static_body(user.user_id.to_string()))?;
    Ok(resp)
}
