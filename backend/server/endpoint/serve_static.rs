use std::convert::identity;

use futures::StreamExt;
use hyper::{
    body::{Bytes, Frame},
    header::CONTENT_TYPE,
    Response, StatusCode,
};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    cfg::Cfg,
    server::{error::error_500, IncReqst, OutResp},
};

// pub async fn serve_static(cfg: &Cfg, req: IncReqst) -> RGptResp {
// serve_static_inner(cfg, req).await.unwrap_or_else(|x| x)
// }

pub async fn serve_static(cfg: &Cfg, req: IncReqst) -> OutResp {
    serve_static_inner(cfg, req).await.unwrap_or_else(identity)
}

pub async fn serve_static_inner(cfg: &Cfg, req: IncReqst) -> Result<OutResp, OutResp> {
    let join = cfg.static_dir.join(&req.uri().path()[1..]);
    let mut path = join;
    if path.is_dir() {
        path = path.join("index.html");
    }

    let mime_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    let file = File::open(path).await.map_err(|_| error_500())?;

    let stream = FramedRead::new(file, BytesCodec::new()).map(|f| Ok(Frame::data(Bytes::from(f?))));

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime_type)
        .body(crate::server::form_stream_body(stream))
        .map_err(|_| error_500())
}
