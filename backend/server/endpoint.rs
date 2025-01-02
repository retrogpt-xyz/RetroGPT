use std::{path::Path, str};

use http_body_util::BodyExt;
use hyper::{body::Body, header::CONTENT_TYPE, Request, Response, StatusCode};
use tokio::{fs::File, io::AsyncReadExt};

use crate::cfg::Cfg;

pub async fn static_file(path: impl AsRef<Path>, content_type: &str) -> super::ServiceResult {
    let mut bytes = Vec::new();
    File::open(path).await?.read_to_end(&mut bytes).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, content_type)
        .body(super::form_body(bytes))?)
}

pub async fn static_dir(
    dir: impl AsRef<Path>,
    req: Request<hyper::body::Incoming>,
) -> super::ServiceResult {
    let mut path = dir.as_ref().join(&req.uri().path()[1..]);
    if path.is_dir() {
        path = path.join("index.html");
    }
    let mime_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();
    static_file(path, &mime_type).await
}

pub async fn prompt_gpt(
    cfg: &Cfg,
    req: Request<hyper::body::Incoming>,
) -> Result<Request<hyper::body::Incoming>, super::ServiceResult> {
    match req.uri().path().starts_with("/api/prompt") {
        true => Err(prompt_gpt_inner(cfg, req).await),
        false => Ok(req),
    }
}

pub async fn prompt_gpt_inner(
    cfg: &Cfg,
    req: Request<hyper::body::Incoming>,
) -> super::ServiceResult {
    if req.body().size_hint().upper().unwrap_or(u64::MAX) > cfg.max_req_size {
        todo!()
    }

    let bytes = req.collect().await?.to_bytes().to_vec();
    let prompt = str::from_utf8(&bytes)?;
    let resp = query_gpt(cfg, prompt).await;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/plain")
        .body(super::form_body(resp))?)
}

pub async fn query_gpt(cfg: &Cfg, prompt: &str) -> String {
    let res = cfg
        .client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", cfg.api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 50
        }))
        .send()
        .await
        .unwrap();
    res.text().await.unwrap()
}
