use rdkafka::{
    error::KafkaError,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig,
};

use crate::sensor::Sensor;

pub fn build_producer() -> Result<FutureProducer, KafkaError> {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:9094")
        .set("message.timeout.ms", "5000")
        .create::<FutureProducer>()
}

pub fn send_message<'a>(
    producer: &'a FutureProducer,
    message: Sensor,
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
