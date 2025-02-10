use std::path::PathBuf;

use futures::StreamExt;
use hyper::{
    body::{Bytes, Frame},
    header::CONTENT_TYPE,
    Response, StatusCode,
};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub mod cfg;
pub mod gpt;
pub mod server;
pub mod startup;

/// The main entrypoint for the application
///
/// Invoked by the binary crate.
#[tokio::main]
pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    // Run startup logic before starting the backend server
    tokio::task::spawn_blocking(startup::startup).await?;

    server::run_server().await
}

async fn s(r: libserver::Request) -> Result<libserver::ServiceResponse, libserver::ServiceError> {
    let mut path = PathBuf::from("static/").join(&r.uri().path()[1..]);

    if path.is_dir() {
        path = path.join("index.html");
    }

    let mime_type = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();

    let file = File::open(path).await.unwrap();

    let stream = FramedRead::new(file, BytesCodec::new()).map(|f| Ok(Frame::data(Bytes::from(f?))));

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime_type)
        .body(crate::server::form_stream_body(stream))
        .unwrap())
}
