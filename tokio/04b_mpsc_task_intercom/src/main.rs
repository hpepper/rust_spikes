use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

/**
 * Used to send messages from a client to the server.
 * 
 * data: the text message to send to the server.
 * resp: the channel to send the response back to the client.
 */
struct ReturnMessage {
    data: String,
    resp: mpsc::Sender<String>,
}

/**
 * The server receives messages from the client and sends a response back.
 * 
 * rx: the channel to receive messages from the client.
 */
async fn server(mut rx: mpsc::Receiver<ReturnMessage>) {
    while let Some(msg) = rx.recv().await {
        println!("Server: {}", msg.data);
        msg.resp.send("World".to_string()).await.unwrap();

    }
}

/**
 * The client sends messages to the server and receives a response back.
 * 
 * tx: the channel to send messages to the server.
 */
async fn client(tx: mpsc::Sender<ReturnMessage>) {

    // create a channel to receive messages from the server.
    let (tx_to_client, mut rx_from_server) = mpsc::channel::<String>(32);

    loop {
        tx.send(ReturnMessage {
            data: "Hello".to_string(),
            resp: tx_to_client.clone(),
        }).await.unwrap();
        // TODO write a clearer way to receive and show the message.
        rx_from_server.try_recv().map(|msg| {
            println!("Client: {}", msg);
        }).unwrap_or(());
    
        sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("MPSC intercom");

    // create a channel to send messages to the server.
    let (tx, rx) = mpsc::channel::<ReturnMessage>(32);

    // spawn the server.
    let srv = tokio::spawn(server(rx));

    // clone the transmit point for use by the second client.
    let tx2 = tx.clone();
    
    // spawn the first client.
    let t1 = tokio::spawn(client(tx));

    // spawn the second client.
    let t2 = tokio::spawn(client(tx2));

    srv.await.unwrap();
    t1.await.unwrap();
    t2.await.unwrap();
}
