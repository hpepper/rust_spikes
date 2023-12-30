use lapin::{
    BasicProperties,
    Connection,
    ConnectionProperties,
    ExchangeKind,
    options::*,
    Result,
    types::FieldTable,
};
use std::env;

// TODO where is lapin getting the definition of ShortShortUInt from?
type ShortShortUInt = u8;
const DELIVERY_MODE_PERSISTENT: ShortShortUInt = 2;
const EXHANGE_NAME: &str = "topic_logs";


#[tokio::main]
async fn main() -> Result<()> {
    println!(" [*] producer v0.1.0");

    let amqp_uri = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://localhost:5672//".into());

    let conn = Connection::connect(&amqp_uri, ConnectionProperties::default()).await?;

    println!(" [*] Connected to {}", amqp_uri);

    let producer_channel = conn.create_channel().await?;

    /*
      exchange_declare - https://docs.rs/lapin/latest/lapin/struct.Channel.html#method.exchange_declare
      kind - https://docs.rs/lapin/latest/lapin/enum.ExchangeKind.html
      options - https://docs.rs/lapin/latest/lapin/options/struct.ExchangeDeclareOptions.html
      arguments - https://docs.rs/amq-protocol-types/7.1.2/amq_protocol_types/struct.FieldTable.html
     */
    producer_channel.exchange_declare(EXHANGE_NAME, ExchangeKind::Topic, ExchangeDeclareOptions::default(), FieldTable::default()).await?;

    println!(" [*] Declared exchange {:?}", EXHANGE_NAME);

    let args: Vec<String> = env::args().collect();

    let routing_key = if args.len() > 1 {
        args[1].clone()
    } else {
        "anonymous.info".to_string()
    };

    let message = if args.len() > 2 {
        args[2].clone()
    } else {
        "Hello World!".to_string()
    };

    // TODO possibly remove the persistense since this is a log?
    let producer_handle = tokio::spawn(async move {
        println!(" [x] publishing");
        let confirm = producer_channel.basic_publish(
            EXHANGE_NAME,
            routing_key.as_str(),
            BasicPublishOptions::default(),
            message.as_bytes(),
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
