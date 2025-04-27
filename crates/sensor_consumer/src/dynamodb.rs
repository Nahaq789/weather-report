use std::time::Duration;

use aws_config::{retry::RetryConfig, timeout::TimeoutConfig};
use aws_sdk_dynamodb::{
    types::{AttributeValue, PutRequest, WriteRequest},
    Client,
};
use sensor::sensor::Sensor;

const TABLE_NAME: &'static str = "Sensor";

pub fn build_client<'a>(
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Client>> + Send + 'a>> {
    Box::pin(async move {
        let timeout_config = TimeoutConfig::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .operation_timeout(Duration::from_secs(30))
            .operation_attempt_timeout(Duration::from_secs(30))
            .build();
        let retry_config = RetryConfig::standard().with_max_attempts(10);
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .test_credentials()
            .endpoint_url("http://localhost:8000")
            .timeout_config(timeout_config)
            .retry_config(retry_config)
            .load()
            .await;
        let dynamodb_local = aws_sdk_dynamodb::config::Builder::from(&config).build();
        let client = aws_sdk_dynamodb::Client::from_conf(dynamodb_local);

        Ok(client)
    })
}

pub fn insert_data<'a>(
    client: &'a Client,
    sensor: Sensor,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
    Box::pin(async move {
        let sensor_id = AttributeValue::S(sensor.sensor_id().to_string());
        let time_stamp = AttributeValue::S(sensor.time_stamp().to_string());
        let area = AttributeValue::S(sensor.location().area().to_string());
        let latitude = AttributeValue::S(sensor.location().latitude().to_string());
        let longitude = AttributeValue::S(sensor.location().longitude().to_string());
        let temperature = AttributeValue::S(sensor.measurements().temperature().to_string());
        let humidity = AttributeValue::S(sensor.measurements().humidity().to_string());
        let status = AttributeValue::S(sensor.status().to_string());

        let request = client
            .put_item()
            .table_name(TABLE_NAME)
            .item("sensor_id", sensor_id)
            .item("time_stamp", time_stamp)
            .item("area", area)
            .item("latitude", latitude)
            .item("longitude", longitude)
            .item("temperature", temperature)
            .item("humidity", humidity)
            .item("status", status);

        match request.send().await {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to insert data: {:?}", e),
        };
        Ok(())
    })
}

pub fn sensor_to_write_request(sensor: &Sensor) -> WriteRequest {
    let mut item = std::collections::HashMap::new();

    item.insert(
        "sensor_id".to_string(),
        AttributeValue::S(sensor.sensor_id().to_string()),
    );
    item.insert(
        "time_stamp".to_string(),
        AttributeValue::S(sensor.time_stamp().to_rfc3339()),
    );
    item.insert(
        "area".to_string(),
        AttributeValue::S(sensor.location().area().to_string()),
    );
    item.insert(
        "latitude".to_string(),
        AttributeValue::S(sensor.location().latitude().to_string()),
    );
    item.insert(
        "longitude".to_string(),
        AttributeValue::S(sensor.location().longitude().to_string()),
    );
    item.insert(
        "temperature".to_string(),
        AttributeValue::S(sensor.measurements().temperature().to_string()),
    );
    item.insert(
        "humidity".to_string(),
        AttributeValue::S(sensor.measurements().humidity().to_string()),
    );
    item.insert(
        "status".to_string(),
        AttributeValue::S(sensor.status().to_string()),
    );

    WriteRequest::builder()
        .put_request(PutRequest::builder().set_item(Some(item)).build().unwrap())
        .build()
}

pub fn batch_insert<'a>(
    client: &'a Client,
    write_requests: Vec<WriteRequest>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
    Box::pin(async move {
        if write_requests.is_empty() {
            return Ok(());
        }
        let mut requests = std::collections::HashMap::new();
        requests.insert(TABLE_NAME.to_string(), write_requests);
        match client
            .batch_write_item()
            .set_request_items(Some(requests))
            .send()
            .await
        {
            Ok(output) => {
                //println!("{:?}", output);
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("{:?}", e)),
        }
    })
}
