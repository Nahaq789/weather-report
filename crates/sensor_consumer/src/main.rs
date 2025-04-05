use std::time::Duration;

use kafka::{build_consumer, receive_messages};
use rdkafka::consumer::Consumer;

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
                let task = tokio::spawn(async move {
                    if let Err(e) = receive_messages(&consumer).await {
                        println!("failed to receive task: {:?}", e);
                    }
                    anyhow::Ok(())
                });

                match task.await {
                    Ok(result) => {
                        if let Err(_) = result {
                            println!("panic consumer task");
                        }
                    }
                    Err(e) => {
                        println!("failed to task: {:?}", e)
                    }
                }

                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        }
    }
}
