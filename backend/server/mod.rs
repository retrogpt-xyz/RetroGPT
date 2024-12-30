pub mod endpoint;

use crate::cfg::Cfg;

use std::net::SocketAddr;
use std::sync::Arc;

use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub async fn run_server() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on: {}", addr);

    let global_cfg = Arc::new(Cfg::new());

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();

        let io = TokioIo::new(stream);
        let local_cfg = Arc::clone(&global_cfg);
        tokio::task::spawn(async move {
            http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| handle_request(Arc::clone(&local_cfg), req)),
                )
                .await
        });
    }
}

pub async fn handle_request(cfg: Arc<Cfg>, req: Request<hyper::body::Incoming>) -> ServiceResult {
    println!("{}", req.uri().path());
    if req.uri().path() == "/api/gpt" {
        return endpoint::gpt_req_inner(&cfg, req).await;
    }
    endpoint::static_dir(&cfg.static_dir, req).await
}

pub type ServiceResult =
    Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Sync + Send + 'static>>;

fn form_body<B: Into<Bytes>>(bytes: B) -> Full<Bytes> {
    Full::new(bytes.into())
}
