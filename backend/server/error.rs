use std::str;

use hyper::{Response, StatusCode};
use serde_json::json;

use crate::server::{form_body, RGptResp};

pub fn internal_error() -> RGptResp {
    let mut resp = Response::new(form_body(
        json!({
            "error": {
                "type": "Internal Server Error",
                "reason": "Internal Server Error. Please try again later or make a bug report"
            }
        })
        .to_string(),
    ));
    *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    resp
}

pub fn bad_request(reason: &str) -> RGptResp {
    let mut resp = Response::new(form_body(
        json!({
            "error": {
                "type": "Bad Request",
                "reason": reason
            }
        })
        .to_string(),
    ));
    *resp.status_mut() = StatusCode::BAD_REQUEST;
    resp
}
