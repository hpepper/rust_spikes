use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// This seems to be required for 'send' to work.
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Result;


#[tokio::main]
async fn main() -> Result<()> {
    let connect_addr = "ws://localhost:9002/";

    let url = url::Url::parse(&connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut ws_write, mut ws_read) = ws_stream.split();

    // send a message to the server, to get the server started on sending messages.
    ws_write.send(Message::Text("first message".to_owned())).await?;

    loop {
        let msg = ws_read.next().await;
        match msg {
            Some(msg) => {
                let msg = msg?;
                if msg.is_text() ||msg.is_binary() {
                    println!("Received a message from the server: {:?}", msg);
                } else if msg.is_close() {
                    break;
                }
            }
            None => break,
        }
    }

    Ok(())
}

