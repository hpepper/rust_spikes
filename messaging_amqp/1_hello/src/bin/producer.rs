use lapin::{
    options::*,
    types::FieldTable,
    BasicProperties,
    Connection,
    ConnectionProperties,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!(" [*] producer v0.1.0");

    let amqp_uri = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://localhost:5672//".into());

    let conn = Connection::connect(&amqp_uri, ConnectionProperties::default()).await?;

    println!(" [*] Connected to {}", amqp_uri);

    let producer_channel = conn.create_channel().await?;

    let queue = producer_channel.queue_declare(
        "hello",
        QueueDeclareOptions::default(),
        FieldTable::default()
    ).await?;
    println!(" [*] Declared queue {:?}", queue);

    let payload = b"Hello world!";

    let producer_handle = tokio::spawn(async move {
        println!(" [x] publishing");
        let confirm = producer_channel.basic_publish(
            "",
            "hello",
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default()
        );
        let result = confirm.await.unwrap();
        println!(" [*] publishing got {:?}", result);
    });

    let result = producer_handle.await;
    println!(" [*] Finished {:?}", result);

    // TODO fix this
    Ok(())
}
