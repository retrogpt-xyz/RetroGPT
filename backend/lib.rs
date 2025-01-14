pub mod cfg;
pub mod server;
pub mod gpt;

use server::run_server;

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_server())?;

    Ok(())
}
