use std::time::Duration;

use aws_config::{retry::RetryConfig, timeout::TimeoutConfig};
use aws_sdk_dynamodb::{types::AttributeValue, Client};
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

// TODO
// batch insert dynamodb
