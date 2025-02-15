pub mod startup;

/// The main entrypoint for the application
///
/// Invoked by the binary crate.
#[tokio::main]
pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    // Run startup logic before starting the backend server
    tokio::task::spawn_blocking(startup::startup).await?;

    let cx = rgpt_cfg::Context::new().await?.into();
    rgpt_server::run_server(cx).await
}
