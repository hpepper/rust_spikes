use lapin::{
    BasicProperties,
    Connection,
    ConnectionProperties,
    options::*,
    Result,
    types::FieldTable,
};
use std::env;

// TODO where is lapin getting the definition of ShortShortUInt from?
type ShortShortUInt = u8;
const DELIVERY_MODE_PERSISTENT: ShortShortUInt = 2;
const QUEUE_NAME: &str = "task_queue";


#[tokio::main]
async fn main() -> Result<()> {
    println!(" [*] producer v0.1.0");

    let amqp_uri = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://localhost:5672//".into());

    let conn = Connection::connect(&amqp_uri, ConnectionProperties::default()).await?;

    println!(" [*] Connected to {}", amqp_uri);

    let producer_channel = conn.create_channel().await?;

    let queue = producer_channel.queue_declare(
        QUEUE_NAME,
        QueueDeclareOptions {
            durable: true, // Set the queue as durable.
            ..Default::default()
        },
        FieldTable::default()
    ).await?;

    println!(" [*] Declared queue {:?}", queue);

    let args: Vec<String> = env::args().collect();

    let payload_prep = if args.len() > 1 {
        args[1].clone()
    } else {
        "Hello...".to_string()
    };
    //let payload: &[u8] = &payload_prep;
    //let payload: &[u8] = payload_prep.as_bytes();

    let producer_handle = tokio::spawn(async move {
        println!(" [x] publishing");
        let confirm = producer_channel.basic_publish(
            "",
            QUEUE_NAME,
            BasicPublishOptions::default(),
            payload_prep.as_bytes(),
            BasicProperties::default()
        .with_delivery_mode(DELIVERY_MODE_PERSISTENT)
        );
        let result = confirm.await.unwrap();
        println!(" [*] publishing got {:?}", result);
    });

    let result = producer_handle.await;
    println!(" [*] Finished {:?}", result);

    // TODO fix this
    Ok(())
}
