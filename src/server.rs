use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};

pub async fn handle(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    write.send(msg).await.expect("Error sending message");
                }
            }
            Err(e) => {
                eprintln!("Error processing message: {:?}", e);
                break;
            }
        }
    }
}

pub async fn start() {
    let addr = "127.0.0.1:8081";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle(stream));
    }
}