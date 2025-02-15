use std::sync::Arc;

use hyper::{Response, StatusCode};
use libserver::{single_frame_body, DynRoute, PathEqRouter, Route};
use rgpt_cfg::Context;
use rgpt_db::{chat::Chat, msg::Msg};
use serde_json::json;

pub fn get_chat_msgs_route(cx: Arc<Context>) -> DynRoute {
    Route::from_parts(
        PathEqRouter::new("/api/chat/messages"),
        GetChatMsgs::new(cx),
    )
    .make_dyn()
}

#[derive(Clone)]
pub struct GetChatMsgs {
    cx: Arc<Context>,
}

impl GetChatMsgs {
    pub fn new(cx: Arc<Context>) -> Self {
        Self { cx }
    }
}

impl tower::Service<libserver::Request> for GetChatMsgs {
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
        Box::pin(async move { get_chat_msgs(req, cx).await })
    }
}

pub async fn get_chat_msgs(
    req: libserver::Request,
    cx: Arc<Context>,
) -> Result<libserver::ServiceResponse, libserver::ServiceError> {
    crate::check_body_size(&req, cx.config.max_req_size)?;
    let headers = req.headers().to_owned();
    let body = crate::collect_body_string(req).await?;
    let chat_id = body.parse::<i32>()?;
    let chat = Chat::n_get_by_id(cx.db(), chat_id).await?;
    let _session = crate::validate_session(cx.db(), &headers, Some(chat.user_id)).await?;

    let msgs = get_chat_message_chain(cx.db(), &chat)
        .await?
        .into_iter()
        .map(|msg| {
            json!({
                "text": msg.body,
                "sender": msg.sender
            })
        })
        .collect::<Vec<_>>();

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("X-Chat-ID", chat_id.to_string())
        .body(single_frame_body(json!(msgs).to_string()))?;
    Ok(resp)
}

pub async fn get_chat_message_chain(
    db: Arc<rgpt_db::Database>,
    chat: &Chat,
) -> Result<Vec<Msg>, libserver::ServiceError> {
    Ok(match chat.head_msg {
        Some(msg_id) => {
            Msg::n_get_by_id(db.clone(), msg_id)
                .await?
                .n_get_msg_chain(db.clone())
                .await?
        }
        None => vec![],
    })
}
