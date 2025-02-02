/// Main logic for RetroGPT
pub mod cfg;
pub mod db;
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
