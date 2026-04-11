use ata_messenger::relay::Relay;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:8080";
    let buffer = 32;
    let runner = Relay::new(address, buffer);
    runner.run().await?;
    Ok(())
}