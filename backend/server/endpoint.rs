use std::path::Path;

use http_body_util::Full;
use hyper::{body::Bytes, header::CONTENT_TYPE, Request, Response, StatusCode};
use tokio::{fs::File, io::AsyncReadExt};

pub type ServiceResult =
    Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Sync + Send + 'static>>;

pub async fn static_file(path: impl AsRef<Path>, content_type: &str) -> ServiceResult {
    let mut bytes = Vec::new();
    File::open(path).await?.read_to_end(&mut bytes).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, content_type)
        .body(Full::new(Bytes::from(bytes)))?)
}

pub async fn static_dir(
    dir: impl AsRef<Path>,
    req: Request<hyper::body::Incoming>,
) -> ServiceResult {
    let path = dir.as_ref().join(match req.uri().path() {
        "/" => "index.html",
        p => &p[1..],
    });
    let mime_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();
    static_file(path, &mime_type).await
}
