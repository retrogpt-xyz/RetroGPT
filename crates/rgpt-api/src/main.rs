use std::{error::Error, net::SocketAddr, sync::Arc};

fn main() -> Result<(), Box<dyn Error>> {
    librgpt::startup::startup();

    main_inner()?;

    Ok(())
}

#[tokio::main]
async fn main_inner() -> Result<(), Box<dyn Error>> {
    let cx = Arc::new(rgpt_cfg::Context::new().await?);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4002));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    rgpt_server::api_service(cx).serve(listener).await?;

    Ok(())
}
