use std::error::Error;

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    let cx = rgpt_cfg::Context::new().await?.into();
    rgpt_server::run_server(cx).await
}
