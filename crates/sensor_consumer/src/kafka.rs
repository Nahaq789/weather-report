use std::sync::Arc;

use futures::StreamExt;
use rdkafka::{consumer::StreamConsumer, message::Headers, ClientConfig, Message};
use sensor::sensor::Sensor;

#[cfg(feature = "with_cassandra")]
use crate::cassandra::{connect_cluster, save_sensor};
use crate::dynamodb::{build_client, insert_data};

pub fn build_consumer() -> anyhow::Result<Arc<StreamConsumer>> {
    let consumer = ClientConfig::new()
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
        .create::<StreamConsumer>()?;
    Ok(Arc::new(consumer))
}

pub fn receive_messages<'a>(
    consumer: &'a Arc<StreamConsumer>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
    Box::pin(async move {
        let mut message_stream = consumer.stream();
        let client = build_client().await?;
        while let Some(message) = message_stream.next().await {
            match message {
                Ok(m) => {
                    let tailored = match m.payload_view::<str>() {
                        Some(Ok(payload)) => {
                            if let Some(headers) = m.headers() {
                                for header in headers.iter() {
                                    println!("Header: {:?}: {:?}", header.key, header.value)
                                }
                            }
                            payload.to_string()
                        }
                        Some(Err(_)) => "Message payload is not string".to_owned(),
                        None => "No payload".to_owned(),
                    };
                    let client_clone = client.clone();
                    tokio::spawn(async move {
                        match Sensor::from_str(&tailored) {
                            Ok(s) => match insert_data(&client_clone, s).await {
                                Ok(_) => (),
                                Err(e) => println!("{:?}", e.to_string()),
                            },
                            Err(e) => println!("serde error: {:?}", e.to_string()),
                        }

                        #[cfg(feature = "with_cassandra")]
                        match connect_cluster().await {
                            Ok(session) => {
                                println!("start cassandra");
                                let _ = save_sensor(&session, &tailored).await;
                                anyhow::Ok(())
                            }
                            Err(e) => Err(anyhow::anyhow!("{}", e.to_string())),
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Failed to receiving message: {:?}", e)
                }
            }
        }
        Ok(())
    })
}
