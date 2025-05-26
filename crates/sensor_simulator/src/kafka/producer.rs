use std::env;

use rdkafka::{
    error::KafkaError,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig,
};
use sensor::sensor;

pub fn build_producer() -> Result<FutureProducer, KafkaError> {
    let bootstrap_servers =
        env::var("KAFKA_SERVER").unwrap_or_else(|_| "localhost:9094".to_string());
    ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create::<FutureProducer>()
}

pub fn send_message<'a>(
    producer: &'a FutureProducer,
    message: sensor::Sensor,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
    Box::pin(async move {
        let serde = serde_json::to_string(&message)
            .map_err(|e| anyhow::anyhow!("Failed to serialize sensor: {}", e))?;
        let binary_data = serde.as_bytes();

        producer
            .send(
                FutureRecord::<(), _>::to("Sensor").payload(binary_data),
                Timeout::Never,
            )
            .await
            .map_err(|(kafka_error, _)| {
                anyhow::anyhow!("Failed to send message: {}", kafka_error)
            })?;

        Ok(())
    })
}
