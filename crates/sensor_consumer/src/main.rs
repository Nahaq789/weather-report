use kafka::{build_consumer, receive_messages};
use rdkafka::consumer::Consumer;

pub mod kafka;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let consumer = build_consumer()?;
    consumer.subscribe(&["Sensor"])?;

    let task = tokio::spawn(async move {
        let _ = receive_messages(&consumer).await;
    });
    tokio::try_join!(task)?;

    println!("Done");
    Ok(())
}
