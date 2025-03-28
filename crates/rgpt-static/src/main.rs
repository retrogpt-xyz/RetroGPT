use std::{error::Error, net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cx = Arc::new(rgpt_cfg::Context::new().await?);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4001));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    let service = rgpt_server::static_asset_service(cx);

    service.serve(listener).await?;

    Ok(())
}
