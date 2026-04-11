// Client network interface

use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, sync::mpsc, net::TcpStream};
use crate::Packet;

pub struct NetworkInterface {
    pub client_transmitter: mpsc::Sender<Packet>,
    pub in_receiver: mpsc::Receiver<Packet>,
}

impl NetworkInterface {
    // If anything is unexplained, first check and see if it was explained in server.rs
    pub async fn start(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(address).await?;
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        
        let (ct, mut cr ) = mpsc::channel::<Packet>(100); // Outbound
        let (it, ir) = mpsc::channel::<Packet>(100); // Inbound
        
        // [Log network interface started]

        tokio::spawn(async move {
            let mut line = String::new();
            loop {
                tokio::select! {
                    read = reader.read_line(&mut line) => {
                        let mut line = String::new();
                        if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                            break;
                        } // Break w/o panic on disconnect
                        if let Ok(packet) = serde_json::from_str::<Packet>(&line) {
                            // Send to message handler
                            let _ = it.send(packet).await;
                        }
                    }
                    Some(send) = cr.recv() => {
                        if let Ok(json) = send.to_json() {
                            let _ = writer.write_all(json.as_bytes()).await;
                        }
                    }
                }
            }
        });
        Ok(Self {
            client_transmitter: ct,
            in_receiver: ir
        })
    }
}