use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

async fn server(mut rx: mpsc::Receiver<String>) {
    while let Some(msg) = rx.recv().await {
        println!("Got: {}", msg);
    }
}

async fn client(tx: mpsc::Sender<String>) {
    loop {
        tx.send("Hello".to_string()).await.unwrap();
        sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("MPSC example");

    let (tx, rx) = mpsc::channel(32);

    let srv = tokio::spawn(server(rx));

    let tx2 = tx.clone();
    
    let t1 = tokio::spawn(client(tx));
    let t2 = tokio::spawn(client(tx2));

    srv.await.unwrap();
    t1.await.unwrap();
    t2.await.unwrap();
}
