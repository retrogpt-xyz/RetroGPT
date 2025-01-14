use hyper::{header::CONTENT_TYPE, Response, StatusCode};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    cfg::Cfg,
    server::{error::internal_error, form_body, IncReqst, RGptResp},
};

pub async fn serve_static(cfg: &Cfg, req: IncReqst) -> RGptResp {
    serve_static_inner(cfg, req).await.unwrap_or_else(|x| x)
}

pub async fn serve_static_inner(cfg: &Cfg, req: IncReqst) -> Result<RGptResp, RGptResp> {
    let join = cfg.static_dir.join(&req.uri().path()[1..]);
    let mut path = join;
    if path.is_dir() {
        path = path.join("index.html");
    }

    let mime_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    let mut buf = Vec::new();
    File::open(&path)
        .await
        .map_err(|_| internal_error())?
        .read_to_end(&mut buf)
        .await
        .map_err(|_| internal_error())?;

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime_type)
        .body(form_body(buf))
        .map_err(|_| internal_error())
}
