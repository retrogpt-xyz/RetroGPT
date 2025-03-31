use std::sync::Arc;

use fastwebsockets::{Frame, OpCode, Payload, upgrade::upgrade};
use futures::{StreamExt, channel::mpsc::UnboundedReceiver};
use hyper::body::Bytes;
use libserver::{DynRoute, PathPrefixRouter, Request, Route, single_frame_body};
use rgpt_cfg::Context;
use uuid::Uuid;

pub fn route(cx: Arc<Context>) -> DynRoute {
    let router = PathPrefixRouter::new("/api/v0.0.1/attach");

    Route::from_parts(router, AttachService::new(cx)).make_dyn()
}

pub async fn attach(req: Request, cx: Arc<Context>) -> libserver::ServiceResult {
    crate::check_body_size(&req, cx.config.max_req_size)?;

    // Extract session token from query param since WebSockets can't set headers
    let session_token = match crate::extract_query_param(req.uri(), "token") {
        Some(token) => token,
        None => {
            // Fall back to header for backward compatibility or non-WebSocket requests
            let headers = req.headers().to_owned();
            match headers.get("X-Session-Token") {
                Some(token) => token.to_str()?.to_owned(),
                None => return Err(crate::InvalidSessionTokenHeader.into()),
            }
        }
    };

    crate::validate_session_token(cx.db(), session_token, None).await?;

    let path = req.uri().path();
    let attach_token_str = path.strip_prefix("/api/v0.0.1/attach/").unwrap_or("");

    // Extract just the chat ID part, removing any trailing slashes or query parameters
    let attach_token_clean = attach_token_str.split('/').next().unwrap_or("");

    if attach_token_clean.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Attach token missing from path",
        )));
    }

    let attach_token = Uuid::parse_str(attach_token_clean)
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid attach token: {}", attach_token_clean),
            )
        })?;

    let rx = cx
        .state
        .stream_registry
        .lock()
        .await
        .try_attach(attach_token)
        .expect("better err handling later");

    let (resp, ws_fut) = upgrade(req)?;
    let resp = resp.map(|_| single_frame_body(""));

    tokio::spawn(stream_model_response(ws_fut, rx));

    Ok(resp)
}

async fn stream_model_response(
    ws_fut: fastwebsockets::upgrade::UpgradeFut,
    mut rx: UnboundedReceiver<Bytes>,
) -> Result<(), libserver::ServiceError> {
    let mut ws = ws_fut
        .await
        .inspect_err(|err| {
            dbg!(err);
        })
        .unwrap();
    while let Some(bytes) = rx.next().await {
        let bc = bytes.clone().to_vec();
        String::from_utf8(bc).ok();
        let frame = Frame::new(true, OpCode::Text, None, Payload::Owned(bytes.to_vec()));
        // let frame = Frame::text(Payload::from("hello ".as_bytes()));
        ws.write_frame(frame)
            .await
            .inspect_err(|err| {
                dbg!(err);
            })
            .unwrap();
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
