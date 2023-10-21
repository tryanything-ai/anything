use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    anything_events::cli::start().await?;

    Ok(())
}
