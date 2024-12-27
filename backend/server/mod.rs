pub mod endpoint;

use std::net::SocketAddr;

use hyper::{server::conn::http1, service::service_fn, Request};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub async fn run_server() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on: {}", addr);
    loop {
        let (stream, _addr) = listener.accept().await.unwrap();

        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
        });
    }
}
pub async fn handle_request(req: Request<hyper::body::Incoming>) -> endpoint::ServiceResult {
    println!("{}", req.uri().path());
    endpoint::static_dir("static/", req).await
}

