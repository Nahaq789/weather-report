use std::{collections::HashMap, str::FromStr};

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use sensor::sensor::Sensor;

const PAGE_SIZE: i32 = 10;

#[derive(Debug)]
pub struct SensorRepositoryImpl {
    client: Client,
}

impl SensorRepositoryImpl {
    fn map(items: HashMap<String, AttributeValue>) {
        let sensor_id = sensor::sensor::sensor_id::SensorId::from_str("");
    }
}

impl sensor::repository::SensorRepository for SensorRepositoryImpl {
    async fn get_sensor_data(
        &self,
        sensor: &sensor::sensor::Sensor,
    ) -> anyhow::Result<Vec<Sensor>> {
        let result = self.client.query().table_name("hoge").send().await?;

        if let Some(items) = result.items {
            // let sensors = items.iter().map(|v| v.into()).collect();
            return Ok(vec![]);
        }
        Ok(vec![])
    }
}

fn as_string<'a>(val: Option<&AttributeValue>, default: &'a str) -> &'a str {
    val.and_then(|v| v.as_s().ok()).unwrap_or_else(|| default)
}
