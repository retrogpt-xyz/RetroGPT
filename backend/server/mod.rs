pub mod endpoint;
pub mod error;
pub mod predicate;

use crate::cfg::Cfg;

use std::error::Error;
use std::io;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};

use futures::Stream;
use http_body_util::{Full, StreamBody};
use hyper::body::Frame;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub type IncReqst = Request<hyper::body::Incoming>;
#[deprecated]
pub type RGptResp = Response<Full<Bytes>>;
pub type OutResp = Response<
    StreamBody<
        Box<dyn Unpin + Send + futures::Stream<Item = Result<Frame<Bytes>, std::io::Error>>>,
    >,
>;

#[macro_export]
macro_rules! handle_endpoint {
    ($pred:expr, $handler:expr, $cfg:expr, $req:expr) => {
        if $pred(&$cfg, &$req) {
            return Ok($handler(&$cfg, $req).await);
        }
    };
}

pub async fn handle_request(cfg: Arc<Cfg>, req: IncReqst) -> Result<OutResp, Infallible> {
    println!("{}", req.uri().path());

    handle_endpoint!(predicate::api_prompt, endpoint::api_prompt, cfg, req);
    handle_endpoint!(predicate::serve_static, endpoint::serve_static, cfg, req);
    // handle_endpoint!(predicate::api_def_sess, endpoint::api_def_sess, cfg, req);

    Ok(error::error_400("request did not match any endpoints"))
}

fn form_stream_body<S>(
    stream: S,
) -> StreamBody<Box<dyn Send + Unpin + 'static + Stream<Item = Result<Frame<Bytes>, io::Error>>>>
where
    S: Stream<Item = Result<Frame<Bytes>, io::Error>> + Unpin + Send + 'static,
{
    StreamBody::new(Box::new(stream)
        as Box<
            dyn Unpin + Send + Stream<Item = Result<Frame<Bytes>, std::io::Error>>,
        >)
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
