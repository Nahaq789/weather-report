use crate::sensor::Sensor;

pub trait SensorRepository {
    fn get_sensor_data(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<Sensor>>> + Send;
}
