use std::sync::Arc;
use std::future::Future;

use fastwebsockets::{Frame, OpCode, Payload, WebSocket, WebSocketError, upgrade::upgrade};
use futures::{channel::mpsc::UnboundedReceiver, StreamExt};
use hyper::{Response, body::Bytes, upgrade::Upgraded};
use hyper_util::rt::TokioIo;
use libserver::{DynRoute, PathPrefixRouter, Request, Route, single_frame_body};
use rgpt_cfg::Context;
use rgpt_db::chat::Chat;

use crate::validate_session_header;

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathPrefixRouter::new("/api/v0.0.1/attach");

    Route::from_parts(router, AttachService::new(cx)).make_dyn()
}

pub async fn attach(req: Request, cx: Arc<Context>) -> libserver::ServiceResult {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let headers = req.headers().to_owned();
    validate_session_header(cx.db(), &headers, None).await?;

    let chat_id = req
        .uri()
        .path()
        .strip_prefix("/api/v0.0.1/attach")
        .unwrap()
        .parse()
        .unwrap();

    let chat = Chat::get_by_id(cx.db(), chat_id).await?;

    let rx = cx
        .state
        .stream_registry
        .lock()
        .await
        .try_attach(chat.id)
        .expect("better err handling later");

    let (_, ws_fut) = upgrade(req)?;

    tokio::spawn(stream_model_response(ws_fut, rx));

    Ok(Response::new(single_frame_body("")))
}

async fn stream_model_response(
    ws_fut: impl Future<Output = Result<WebSocket<TokioIo<Upgraded>>, WebSocketError>>,
    mut rx: UnboundedReceiver<Bytes>,
) -> Result<(), libserver::ServiceError> {
    let mut ws = ws_fut.await?;
    
    while let Some(bytes) = rx.next().await {
        let frame = Frame::new(true, OpCode::Binary, None, Payload::Owned(bytes.to_vec()));
        ws.write_frame(frame).await?;
    }
    
    Ok(())
}

#[derive(Clone)]
pub struct AttachService {
    cx: Arc<Context>,
}

impl AttachService {
    pub fn new(cx: Arc<Context>) -> Self {
        AttachService { cx }
    }
}

impl tower::Service<libserver::Request> for AttachService {
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
        Box::pin(async move { attach(req, cx).await })
    }
}
