pub mod endpoint;
pub mod error;
pub mod predicate;

use crate::cfg::Cfg;

use std::error::Error;
use std::io;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};

use futures::Stream;
use http_body_util::StreamBody;
use hyper::body::Frame;
use hyper::{body::Bytes, Request, Response};
use libserver::{body_stream, once_body};
use tokio::net::TcpListener;

pub type IncReqst = Request<hyper::body::Incoming>;
pub type OutResp = Response<
    StreamBody<
        Box<dyn Unpin + Send + futures::Stream<Item = Result<Frame<Bytes>, std::io::Error>>>,
    >,
>;

#[macro_export]
macro_rules! handle_endpoint {
    ($pred:expr, $handler:expr, $cfg:expr, $req:expr) => {
        if $pred(&$cfg, &$req).await {
            return Ok($handler(&$cfg, $req).await);
        }
    };
}

pub async fn handle_request(cfg: Arc<Cfg>, req: IncReqst) -> Result<OutResp, Infallible> {
    let path = req.uri().path();
    println!("{path}");

    handle_endpoint!(predicate::api_prompt, endpoint::api_prompt, cfg, req);

    handle_endpoint!(predicate::serve_static, endpoint::serve_static, cfg, req);
    handle_endpoint!(predicate::auth, endpoint::auth, cfg, req);
    handle_endpoint!(predicate::session, endpoint::session, cfg, req);
    handle_endpoint!(predicate::api_def_sess, endpoint::api_def_sess, cfg, req);
    handle_endpoint!(predicate::api_chats, endpoint::api_chats, cfg, req);
    handle_endpoint!(
        predicate::api_chat_messages,
        endpoint::api_chat_messages,
        cfg,
        req
    );

    Ok(error::error_400("request did not match any endpoints"))
}

pub fn form_stream_body<S>(
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
    let fallback =
        tower::service_fn(|_| async { Ok(Response::new(body_stream(once_body("404 not found")))) });

    let service = tower::service_fn(super::s);

    println!("listening");

    let global_cfg = Arc::new(Cfg::get().await?);
    let addr = SocketAddr::from(([0, 0, 0, 0], global_cfg.port));
    let listener = TcpListener::bind(addr).await?;

    libserver::ServiceBuilder::new()
        .with_static_dir("static/", service)
        .with_fallback(fallback)
        .serve(listener)
        .await?;

    Ok(())
}
