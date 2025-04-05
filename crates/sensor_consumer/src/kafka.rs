use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    error::KafkaError,
    message::Headers,
    ClientConfig, Message,
};

pub fn build_consumer() -> Result<StreamConsumer, KafkaError> {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:9094")
        .set("enable.partition.eof", "false")
        .set("auto.offset.reset", "earliest")
        .set("session.timeout.ms", "30000")
        .set("max.poll.interval.ms", "300000")
        .set("message.timeout.ms", "10000")
        .set("heartbeat.interval.ms", "2000")
        .set("socket.timeout.ms", "60000")
        .set("fetch.min.bytes", "1")
        .set("fetch.wait.max.ms", "100")
        .set("group.id", "sensor_group_1")
        .create::<StreamConsumer>()
}

pub fn receive_messages<'a>(
    consumer: &'a StreamConsumer,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>> {
    Box::pin(async move {
        loop {
            match consumer.recv().await {
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            println!("Failed while deserializing message payload: {:?}", e);
                            ""
                        }
                    };
                    println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());

                    if let Some(headers) = m.headers() {
                        for header in headers.iter() {
                            println!("Header {:#?}: {:?}", header.key, header.value);
                        }
                    }

                    if let Err(e) = consumer.commit_message(&m, CommitMode::Async) {
                        println!("Failed to commit offset: {:?}", e);
                    }
                }
                Err(e) => println!("Failed to recive message: {:?}", e),
            };
        }
    })
}
