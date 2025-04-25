use std::collections::HashMap;

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use sensor::sensor::Sensor;

const PAGE_SIZE: i32 = 10;

#[derive(Debug)]
pub struct SensorRepositoryImpl {
    client: Client,
}

impl SensorRepositoryImpl {
    fn map(items: Vec<HashMap<String, AttributeValue>>) {}
}

impl sensor::repository::SensorRepository for SensorRepositoryImpl {
    async fn get_sensor_data(
        &self,
        sensor: &sensor::sensor::Sensor,
    ) -> anyhow::Result<Vec<Sensor>> {
        let result = self.client.query().table_name("hoge").send().await?;

        if let Some(items) = result.items {
            let sensors = items.iter().map(|v| v.into()).collect();
            Ok(sensors)
        }
        Ok(vec![])
    }
}
