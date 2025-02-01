pub mod cfg;
pub mod db;
pub mod gpt;
pub mod server;
pub mod startup;

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    startup::startup();

    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async { server::run_server().await })
}
