use http_body_util::BodyExt;
use hyper::{body::Body, header::CONTENT_TYPE, Response, StatusCode};

use crate::{
    cfg::Cfg,
    gpt::gpt_api,
    server::{
        error::{bad_request, internal_error},
        form_body, IncReqst, RGptResp,
    },
};

pub async fn api_prompt(cfg: &Cfg, req: IncReqst) -> RGptResp {
    api_prompt_inner(cfg, req).await.unwrap_or_else(|x| x)
}

pub async fn api_prompt_inner(cfg: &Cfg, req: IncReqst) -> Result<RGptResp, RGptResp> {
    if req.body().size_hint().upper().unwrap_or(u64::MAX) > cfg.max_req_size {
        return Err(bad_request("request body is too large"));
    }

    let bytes = req
        .collect()
        .await
        .map_err(|_| internal_error())?
        .to_bytes()
        .to_vec();
    let prompt = std::str::from_utf8(&bytes).map_err(|_| internal_error())?;

    let recvd = gpt_api(cfg, prompt).await.map_err(|_| internal_error())?;
    let parsed = serde_json::from_str::<serde_json::Value>(&recvd).map_err(|_| internal_error())?;

    let resp = match parsed
        .get("choices")
        .ok_or(internal_error())?
        .get(0)
        .ok_or(internal_error())?
        .get("message")
        .ok_or(internal_error())?
        .get("content")
        .ok_or(internal_error())?
    {
        serde_json::Value::String(string) => String::from(string),
        _ => Err(internal_error())?,
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/plain")
        .body(form_body(resp))
        .map_err(|_| internal_error())
}
