use std::{net::SocketAddr};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

struct MessageProtocol {
    operation: u8,
    size: u64,
    payload: usize,
}

async fn handle_socket_connection(mut stream: TcpStream, _: SocketAddr) {
    loop {
        let mut buffer_operation = [0u8; 1024];

        match stream.read(&mut buffer_operation).await {
            Ok(0) => {
                println!("Client Disconnected");
                break;
            }
            Ok(n) => {
                println!("bytes: {}, Decimal is {:?}", n, &buffer_operation[..n])
            }
            Err(e) => {
                println!("Erro in read message: {}", e);
                break;
            } 
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8787").await.unwrap();

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_socket_connection(stream, _addr).await;
        });
    }
}
