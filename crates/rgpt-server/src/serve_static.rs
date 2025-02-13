use futures::{Stream, StreamExt};
use hyper::{
    body::{Bytes, Frame},
    header::CONTENT_TYPE,
    Response, StatusCode,
};
use libserver::{
    make_body_from_stream, BodyInner, Request, ServiceBoxFuture, ServiceError, ServiceResponse,
};
use std::{io, path::PathBuf};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Clone)]
pub struct StaticAssetService {
    path: PathBuf,
}

impl StaticAssetService {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl tower::Service<Request> for StaticAssetService {
    type Response = ServiceResponse;
    type Error = ServiceError;
    type Future = ServiceBoxFuture;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        Box::pin(serve_static(self.path.clone(), req))
    }
}

async fn serve_static(path: PathBuf, req: Request) -> Result<ServiceResponse, ServiceError> {
    let path = parse_path(path, &req.uri().path()[1..]);
    let mime_type = parse_mime(&path);
    let stream = get_file_stream(path).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime_type)
        .body(make_body_from_stream(stream))?)
}

fn parse_path(base: PathBuf, path: impl Into<PathBuf>) -> PathBuf {
    let mut path = base.join(path.into());
    if path.is_dir() {
        path.push("index.html")
    }
    dbg!(path)
}

fn parse_mime(path: &PathBuf) -> String {
    mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string()
}

async fn get_file_stream(path: PathBuf) -> io::Result<impl Stream<Item = BodyInner>> {
    let file = File::open(path).await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    Ok(stream.map(|f| Ok(Frame::data(Bytes::from(f?)))))
}
