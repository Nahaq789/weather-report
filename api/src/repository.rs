use std::{collections::HashMap, str::FromStr};

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::{DateTime, Duration, Local};
use sensor::sensor::{
    location::{self, area, latitude, longitude},
    measurements::{self, humidity, temperature},
    sensor_id, status, Sensor,
};

const TABLE_NAME: &'static str = "Sensor";

#[derive(Debug)]
pub struct SensorRepositoryImpl {
    client: Client,
}

impl SensorRepositoryImpl {
    pub fn new(client: Client) -> SensorRepositoryImpl {
        SensorRepositoryImpl { client }
    }

    fn map(items: &HashMap<String, AttributeValue>) -> String {
        let sensor_id = sensor_id::SensorId::from(as_string(items.get("sensor_id"), ""));
        let time_stamp =
            DateTime::<Local>::from_str(&as_string(items.get("time_stamp"), "")).unwrap();
        let area = area::Area::from_str(&as_string(items.get("area"), "")).unwrap();
        let latitude = latitude::Latitude::from(as_string(items.get("latitude"), ""));
        let longitude = longitude::Longitude::from(as_string(items.get("longitude"), ""));
        let temperature = temperature::Temperature::from(as_string(items.get("temperature"), ""));
        let humidity = humidity::Humidity::from(as_string(items.get("humidity"), ""));
        let status = status::Status::from_str(&as_string(items.get("status"), ""));

        let location = location::Location::from(area, latitude, longitude);
        let measurements = measurements::Measurements::from(temperature, humidity);

        let sensor = Sensor::from(sensor_id, location, time_stamp, measurements, status);
        let serialized = serde_json::to_string(&sensor).unwrap();

        serialized
    }
}

impl sensor::repository::SensorRepository for SensorRepositoryImpl {
    async fn get_sensor_data(&self, area: &str) -> anyhow::Result<Vec<String>> {
        let one_minute_ago = Local::now() - Duration::minutes(1);
        println!("{:?}", one_minute_ago);
        let result = self
            .client
            .query()
            .table_name(TABLE_NAME)
            .key_condition_expression("#area = :area AND #time_stamp > :time_stamp")
            .expression_attribute_names("#area", "area")
            .expression_attribute_values(":area", AttributeValue::S(area.to_string()))
            .expression_attribute_names("#time_stamp", "time_stamp")
            .expression_attribute_values(
                ":time_stamp",
                AttributeValue::S(one_minute_ago.to_rfc3339()),
            )
            .send()
            .await?;

        if let Some(items) = result.items {
            let sensors: Vec<String> = items.iter().map(|v| Self::map(v)).collect();
            return Ok(sensors);
        }
        Ok(vec![])
    }
}

fn as_string(val: Option<&AttributeValue>, default: &str) -> String {
    val.and_then(|v| v.as_s().ok())
        .map(|v| v.to_string())
        .unwrap_or_else(|| default.to_owned())
}
