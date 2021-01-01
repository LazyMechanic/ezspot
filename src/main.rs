#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ezspot_lib::run().await?;

    Ok(())
}
