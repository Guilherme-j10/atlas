mod types;

use std::sync::Arc;

use serde_json::Value;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::types::types::MessageProtocol;

async fn read_protocol(
    stream: &mut TcpStream,
) -> Result<MessageProtocol, Box<dyn std::error::Error>> {
    let mut buffer_operation = [0u8; 3];
    stream.read_exact(&mut buffer_operation).await.unwrap();

    let operation = &buffer_operation[0];
    let bytes_amount = String::from_utf8(buffer_operation[2..3].to_vec())
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let mut bytes_lentgh = vec![0u8; bytes_amount as usize];
    stream.read_exact(&mut bytes_lentgh).await.unwrap();

    let str_concat: String = bytes_lentgh.iter().map(|b| b.to_string()).collect();
    let complete_bytes = str_concat.parse::<u32>().unwrap();

    let mut payload_bytes = vec![0u8; complete_bytes as usize];
    stream.read_exact(&mut payload_bytes).await.unwrap();

    let payload_content = String::from_utf8(payload_bytes).unwrap();

    Ok(MessageProtocol {
        operation: *operation,
        bytes: complete_bytes,
        payload: serde_json::from_str::<Value>(&payload_content).unwrap(),
    })
}

struct Broker;
impl Broker {
    fn new() -> Self {
        Broker
    }

    fn initialize_topics(&self) {}
    fn register_consumer(&self) {}
    fn register_message_from_producer(&self) {}
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8787").await.unwrap();

    let broker = Arc::new(Broker::new());
    broker.initialize_topics();

    loop {
        let broker_instace = Arc::clone(&broker);
        let (mut stream, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            loop {
                match read_protocol(&mut stream).await {
                    Ok(message) => {
                        if message.operation == 1 {
                            broker_instace.register_message_from_producer();
                        }
                        if message.operation == 2 {
                            broker_instace.register_consumer();
                        }
                    }
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
        });
    }
}
