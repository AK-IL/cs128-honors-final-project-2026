use ata_messenger::network_interface::NetworkInterface;
use ata_messenger::{Packet, Navigation, Data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:8080";
    let ni = NetworkInterface::start(address).await?;

    Ok(())
}