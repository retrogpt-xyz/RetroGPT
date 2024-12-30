use retro_gpt_backend::run_server;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_server());

    Ok(())
}
