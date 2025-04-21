use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    usize,
};

use aws_sdk_dynamodb::types::WriteRequest;
use futures::{lock::Mutex, StreamExt};
use rdkafka::{consumer::StreamConsumer, message::Headers, ClientConfig, Message};
use sensor::sensor::Sensor;

#[cfg(feature = "with_cassandra")]
use crate::cassandra::{connect_cluster, save_sensor};
use crate::dynamodb::{batch_insert, build_client, sensor_to_write_request};

const BATCH_SIZE: usize = 25;

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
        let batch_buffer = Arc::new(Mutex::new(Vec::<WriteRequest>::with_capacity(BATCH_SIZE)));

        let mut counter = Arc::new(AtomicUsize::new(0));

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
                    let buffer_clone = batch_buffer.clone();
                    let counter_clone = counter.clone();

                    tokio::spawn(async move {
                        match Sensor::from_str(&tailored) {
                            Ok(s) => {
                                let write_request = sensor_to_write_request(&s);

                                {
                                    let mut buffer = buffer_clone.lock().await;
                                    buffer.push(write_request);

                                    if buffer.len() >= BATCH_SIZE {
                                        let batch = std::mem::take(&mut *buffer);
                                        if let Err(e) = batch_insert(&client_clone, batch).await {
                                            eprintln!("Failed to insert batch: {:?}", e)
                                        };
                                    }
                                }

                                counter_clone.fetch_add(1, Ordering::Relaxed);
                                let count = counter_clone.load(Ordering::Relaxed);

                                if count % 1000 == 0 {
                                    println!("process: {:?}", counter_clone)
                                }
                            }
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
