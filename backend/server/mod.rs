pub mod endpoint;
pub mod error;
pub mod predicate;

use crate::cfg::Cfg;

use std::error::Error;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};

use http_body_util::Full;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub type IncReqst = Request<hyper::body::Incoming>;
pub type RGptResp = Response<Full<Bytes>>;

#[macro_export]
macro_rules! handle_endpoint {
    ($pred:expr, $handler:expr, $cfg:expr, $req:expr) => {
        if $pred(&$cfg, &$req) {
            return Ok($handler(&$cfg, $req).await);
        }
    };
}

pub async fn handle_request(cfg: Arc<Cfg>, req: IncReqst) -> Result<RGptResp, Infallible> {
    println!("{}", req.uri().path());

    handle_endpoint!(predicate::api_prompt, endpoint::api_prompt, cfg, req);
    handle_endpoint!(predicate::serve_static, endpoint::serve_static, cfg, req);

    Ok(error::bad_request("request did not match any endpoints"))
}

fn form_body<B: Into<Bytes>>(bytes: B) -> Full<Bytes> {
    Full::new(bytes.into())
}

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    let global_cfg = Arc::new(Cfg::get().await?);

    let addr = SocketAddr::from(([0, 0, 0, 0], global_cfg.port));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (stream, _addr) = listener.accept().await?;
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
