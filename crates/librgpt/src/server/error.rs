use hyper::{
    body::{Bytes, Frame},
    header::{HeaderValue, CONTENT_TYPE},
    Response, StatusCode,
};
use serde_json::json;

use crate::server::OutResp;

pub fn error_500() -> OutResp {
    let stream = futures::stream::once(async {
        Ok(Frame::data(Bytes::from(
            json!({
                "error": {
                    "type": "Internal Server Error",
                    "reason": "Internal Server Error. Please try again later or make a bug report"
                }
            })
            .to_string(),
        )))
    });
    let mut resp = Response::new(crate::server::form_stream_body(Box::pin(stream)));
    *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    resp.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    resp
}

pub fn error_400(reason: &str) -> OutResp {
    let reason = String::from(reason);
    let stream = futures::stream::once(async move {
        Ok(Frame::data(Bytes::from(
            json!({
                "error": {
                    "type": "Bad Request",
                    "reason": reason
                }
            })
            .to_string(),
        )))
    });
    let mut resp = Response::new(crate::server::form_stream_body(Box::pin(stream)));
    *resp.status_mut() = StatusCode::BAD_REQUEST;
    resp.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    resp
}
