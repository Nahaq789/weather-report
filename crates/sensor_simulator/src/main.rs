use sensor::sensor::{location::area, season, Sensor};
use sensor_simulator::kafka::producer::{build_producer, send_message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let producer = build_producer()?;
    let mut tasks = Vec::new();
    for i in 0..10 {
        let producer_clone = producer.clone();
        let task = tokio::spawn(async move {
            let area = area::Area::Tokyo;
            let season = season::Season::Spring;
            let sensor = Sensor::new(&area, &season);

            if i % 1000 == 0 {
                println!("Sending message {}", i);
            }
            let _ = send_message(&producer_clone, sensor).await;
        });
        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await?;
    }

    println!("All messages sent");
    Ok(())
}
