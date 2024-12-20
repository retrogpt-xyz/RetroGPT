use std::path::Path;

use http_body_util::Full;
use hyper::{body::Bytes, header::CONTENT_TYPE, Response, StatusCode};
use tokio::{fs::File, io::AsyncReadExt};

pub type ServiceResult =
    Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Sync + Send + 'static>>;

async fn static_file(path: impl AsRef<Path>, content_type: &'static str) -> ServiceResult {
    let mut bytes = Vec::new();
    File::open(path)
        .await
        .inspect_err(|e| println!("{e}"))?
        .read_to_end(&mut bytes)
        .await
        .inspect_err(|e| println!("{e}"))?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, content_type)
        .body(Full::new(Bytes::from(bytes)))?)
}

pub async fn home() -> ServiceResult {
    println!("running home fn");
    static_file("static/index.html", "text/html").await
}

pub async fn not_found() -> ServiceResult {
    static_file("static/not_found.html", "text/html").await
}

pub async fn chat_logo() -> ServiceResult {
    static_file("static/chatlogo.png", "image/png").await
}

pub async fn bwvid() -> ServiceResult {
    static_file("static/tmp_bw.mp4", "video/mp4").await
}

pub async fn secondclip() -> ServiceResult {
    static_file("static/tmp_secondclip.mp4", "video/mp4").await
}
