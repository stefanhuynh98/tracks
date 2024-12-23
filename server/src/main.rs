use anyhow::Result;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    Ok(())
}
