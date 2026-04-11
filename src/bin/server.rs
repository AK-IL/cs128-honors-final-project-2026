use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use ata_messenger::MsgProtocol;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Return type based on https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
    // Using tokio for async since synchronous networking would block threads and break with many users

}