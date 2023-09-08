use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    anything_events::cli::start().await?;

    Ok(())
}
