pub mod endpoint;

use std::net::SocketAddr;

use hyper::{server::conn::http1, service::service_fn, Method, Request};
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
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => endpoint::home().await,
        (&Method::GET, "/chatlogo.png") => endpoint::chat_logo().await,
        (&Method::GET, "/bw.mp4") => endpoint::bwvid().await,
        (&Method::GET, "/secondclip.mp4") => endpoint::secondclip().await,
        _ => endpoint::not_found().await,
    }
}
