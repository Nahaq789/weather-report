use sensor_simulator::{
    kafka::producer::{build_producer, send_message},
    sensor::{location::area::Area, season::Season, Sensor},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let producer = build_producer()?;
    let mut tasks = Vec::new();
    for i in 0..10000 {
        let producer_clone = producer.clone();
        let task = tokio::spawn(async move {
            let area = Area::Tokyo;
            let season = Season::Spring;
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
