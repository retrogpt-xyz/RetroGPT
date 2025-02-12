use std::sync::Arc;

use hyper::Response;
use libserver::{static_body, DynRoute, PathEqRouter, Route};
use rgpt_db::{session::Session, user::User, Database};

pub fn get_default_session_route(db: Arc<Database>) -> DynRoute {
    Route::from_parts(
        PathEqRouter::new("/api/get_def_sess"),
        DefaultSession { db },
    )
    .make_dyn()
}

#[derive(Clone)]
pub struct DefaultSession {
    db: Arc<Database>,
}

impl tower::Service<libserver::Request> for DefaultSession {
    type Response = libserver::ServiceResponse;
    type Error = libserver::ServiceError;
    type Future = libserver::ServiceBoxFuture;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: libserver::Request) -> Self::Future {
        let db = self.db.clone();
        Box::pin(async move {
            let session = get_default_session(db).await.unwrap();
            let response = Response::new(static_body(session.session_token));
            Ok(response)
        })
    }
}

pub async fn get_default_session(db: Arc<Database>) -> Result<Session, libserver::ServiceError> {
    let user = User::n_default(db.clone()).await?;
    let session = Session::n_get_for_user(db.clone(), &user).await?;
    Ok(session)
}
