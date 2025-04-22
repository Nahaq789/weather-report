use std::time::Duration;

use rand::Rng;
use sensor::sensor::{location::area, season, Sensor};
use sensor_simulator::kafka::producer::{build_producer, send_message};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let producer = build_producer()?;
    let mut rng = rand::thread_rng();
    let mut count = 0;

    loop {
        let mut tasks = Vec::new();
        for i in 0..100000 {
            let producer_clone = producer.clone();

            let area_value = rng.gen_range(0..99);
            let season_value = rng.gen_range(1..12);
            let task = tokio::spawn(async move {
                let area = area::Area::build(area_value);
                let season = season::Season::build(season_value);
                let sensor = Sensor::new(&area, &season);

                if i % 1000 == 0 {
                    println!("Sending message {}", i);
                }
                let _ = send_message(&producer_clone, sensor).await;
            });
            tasks.push(task);
        }

        println!("All messages sent");
        println!("count: {}", count);
        count += 1;
        tokio::time::sleep(Duration::from_secs(600)).await;

        for task in tasks {
            let _ = task.await?;
        }
    }
}
