use std::{future::Future, pin::Pin, sync::Arc};

use bytes::Bytes;
use http_body_util::StreamBody;
use hyper::{
    body::{Frame, Incoming},
    server::conn::http1,
};
use hyper_util::{rt::TokioIo, service::TowerToHyperService};
use tokio::net::TcpListener;
use tower::{util::BoxCloneSyncService, Service as TowerService};

type Request = hyper::Request<Incoming>;

pub trait Router: Send + Sync + 'static {
    fn matches(&self, req: &Request) -> bool;
}

pub struct Route<R, S> {
    router: R,
    service: S,
}

impl<R, S> Route<R, S> {
    pub fn from_parts(router: R, service: S) -> Route<R, S> {
        Route { router, service }
    }

    pub fn map_router<N>(self, f: impl FnOnce(R) -> N) -> Route<N, S> {
        Route::from_parts(f(self.router), self.service)
    }

    pub fn map_service<U>(self, f: impl FnOnce(S) -> U) -> Route<R, U> {
        Route::from_parts(self.router, f(self.service))
    }
}
impl<R: Router, S: TowerService<Request> + Send + Sync + Clone + 'static> Route<R, S>
where
    S::Future: Send + 'static,
{
    pub fn make_dyn(
        self,
    ) -> Route<Arc<dyn Router>, BoxCloneSyncService<Request, S::Response, S::Error>> {
        self.map_router(|r| Arc::new(r) as Arc<dyn Router>)
            .map_service(|s| BoxCloneSyncService::new(s))
    }
}

pub type ServiceResponse = hyper::Response<
    StreamBody<
        Box<dyn Unpin + Send + futures::Stream<Item = Result<Frame<Bytes>, std::io::Error>>>,
    >,
>;
pub type ServiceError = Arc<dyn std::error::Error + Send + Sync>;

pub struct ServiceBuilder {
    routes:
        Vec<Route<Arc<dyn Router>, BoxCloneSyncService<Request, ServiceResponse, ServiceError>>>,
}

impl ServiceBuilder {
    pub fn new() -> ServiceBuilder {
        ServiceBuilder { routes: vec![] }
    }

    pub fn with_route(
        mut self,
        route: Route<Arc<dyn Router>, BoxCloneSyncService<Request, ServiceResponse, ServiceError>>,
    ) -> ServiceBuilder {
        self.routes.push(route);
        self
    }

    pub fn with_fallback(
        self,
        fallback: BoxCloneSyncService<Request, ServiceResponse, ServiceError>,
    ) -> Service {
        Service {
            routes: self
                .routes
                .into_iter()
                .map(|r| r.map_service(TowerToHyperService::new))
                .collect(),
            fallback: TowerToHyperService::new(fallback),
        }
    }
}

pub struct Service {
    routes: Vec<
        Route<
            Arc<dyn Router>,
            TowerToHyperService<BoxCloneSyncService<Request, ServiceResponse, ServiceError>>,
        >,
    >,
    fallback: TowerToHyperService<BoxCloneSyncService<Request, ServiceResponse, ServiceError>>,
}

impl Service {
    pub async fn serve(self, listener: TcpListener) -> Result<(), std::io::Error> {
        let service = Arc::new(self);
        loop {
            let service = service.clone();
            let stream = TokioIo::new(listener.accept().await?.0);

            tokio::spawn(async move {
                http1::Builder::new()
                    .serve_connection(stream, service)
                    .await
                    .ok();
            });
        }
    }
}

impl hyper::service::Service<Request> for Service {
    type Response = ServiceResponse;
    type Error = ServiceError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    fn call(&self, req: Request) -> Self::Future {
        for route in self.routes.iter() {
            if route.router.matches(&req) {
                return Box::pin(route.service.call(req));
            }
        }
        Box::pin(self.fallback.call(req))
    }
}
