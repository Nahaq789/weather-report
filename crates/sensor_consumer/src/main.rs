use std::time::Duration;

use kafka::{build_consumer, receive_messages};
use rdkafka::consumer::Consumer;

pub mod cassandra;
pub mod dynamodb;
pub mod kafka;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let topic = "Sensor";
    loop {
        println!("build kafka consumer");
        match build_consumer() {
            Ok(consumer) => {
                if let Err(e) = consumer.subscribe(&[topic]) {
                    println!("failed subscribe topic: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }

                println!("stating receive messages");

                if let Err(e) = receive_messages(&consumer).await {
                    println!("failed to receive: {:?}", e);
                }

                // tokio::time::sleep(Duration::from_secs(5)).await;
            }
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }
}
