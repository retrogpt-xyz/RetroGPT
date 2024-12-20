use retro_gpt_backend::run_server;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("running");
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(run_server());
    Ok(())
}

async fn _send_req() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let resp = reqwest::Client::new()
        .get("http://localhost:8080")
        .send()
        .await?;
    let y = resp.headers();
    println!("{y:?}");
    let x = resp.text().await?;
    println!("{x}");
    Ok(())
}
