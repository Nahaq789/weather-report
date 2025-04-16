use aws_sdk_dynamodb::{types::AttributeValue, Client};

use crate::sensor::Sensor;

const TABLE_NAME: &'static str = "sensor_data";

pub fn build_client<'a>(
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Client>> + Send + 'a>> {
    Box::pin(async move {
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .test_credentials()
            .endpoint_url("http://localhost:8000")
            .load()
            .await;
        let dynamodb_local = aws_sdk_dynamodb::config::Builder::from(&config).build();
        let client = aws_sdk_dynamodb::Client::from_conf(dynamodb_local);

        // test code
        // let list_resp = client.list_tables().send().await;
        // match list_resp {
        //     Ok(res) => {
        //         println!("Found {} tables", res.table_names().len());
        //         for name in res.table_names() {
        //             println!("{}", name)
        //         }
        //     }
        //     Err(err) => eprintln!("Failed to list local tables {:?}", err.to_string()),
        // }
        Ok(client)
    })
}

pub fn insert_data<'a>(
    client: &'a Client,
    sensor: Sensor,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
    Box::pin(async move {
        let sensor_id = AttributeValue::S(sensor.sensor_id);
        let time_stamp = AttributeValue::S(sensor.time_stamp.to_string());
        let area = AttributeValue::S(sensor.area);
        let latitude = AttributeValue::S(sensor.latitude.to_string());
        let longitude = AttributeValue::S(sensor.longitude.to_string());
        let temperature = AttributeValue::S(sensor.temperature.to_string());
        let humidity = AttributeValue::S(sensor.humidity.to_string());
        let status = AttributeValue::S(sensor.status);

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

        request.send().await?;
        Ok(())
    })
}
