use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    eventurous::cli::start().await?;

    Ok(())
}
