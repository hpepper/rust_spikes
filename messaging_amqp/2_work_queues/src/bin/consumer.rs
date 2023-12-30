use futures::StreamExt;
use lapin::{ options::*, types::FieldTable, Connection, ConnectionProperties, Result };
//use std::time::Duration;
use tokio::time::{sleep, Duration};

const QUEUE_NAME: &str = "task_queue";

#[tokio::main]
async fn main() -> Result<()> {
    println!(" [*] consumer v0.1.0");

    let amqp_uri = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://localhost:5672//".into());

    let conn = Connection::connect(&amqp_uri, ConnectionProperties::default()).await?;

    println!(" [*] Connected to {}", amqp_uri);

    let consumer_channel = conn.create_channel().await?;

    // https://www.rabbitmq.com/confirms.html#channel-qos-prefetch
    // Only one message at a time.
    // global: https://www.rabbitmq.com/consumer-prefetch.html
    consumer_channel.basic_qos(1, BasicQosOptions{global:true}).await?;

    let queue = consumer_channel.queue_declare(
        QUEUE_NAME,
        QueueDeclareOptions {
            durable: true, // Set the queue as durable.
            ..Default::default()
        },
        FieldTable::default()
    ).await?;

    println!(" [*] Declared queue {:?}", queue);

    let mut consumer = consumer_channel.basic_consume(
        QUEUE_NAME,
        "my_unused_consumer_tag",
        BasicConsumeOptions {
            no_ack: false, // do not automatically send an ack
            ..Default::default()
        },
        FieldTable::default()
    ).await?;

    let consumer_handle = tokio::spawn(async move {
        println!(" [*] will consume");
        loop {
            println!(" [*] wait for next message");
            match consumer.next().await {
                Some(delivery_option) => {
                    match delivery_option {
                        Ok(delivery) => {
                            let message_body = delivery.data.clone();
                            let message_text = String::from_utf8_lossy(&message_body);

                            println!(" [x] received message {}", message_text);
                            // count the number of dots '.'
                            let number_of_dots: u64 = message_text.matches('.').count() as u64;
                            sleep(Duration::from_secs(number_of_dots)).await;
                            // let delivery_something = delivery_option.expect("error in consumer");
                            delivery.ack(BasicAckOptions::default()).await.expect("ack");
                        }
                        Err(err) => {
                            eprintln!(" [*] Error while receiving message: {:?}", err);
                        }
                    }
                }
                None => {
                    // No more messages in the queue, you can break the loop or take other action.
                    break;
                }
            }
        }
    });

    let result = consumer_handle.await;
    println!(" [*] Finished {:?}", result);

    // TODO fix this
    Ok(())
}
