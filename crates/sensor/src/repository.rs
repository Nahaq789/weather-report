use crate::sensor::Sensor;

pub trait SensorRepository {
    fn get_sensor_data() -> impl std::future::Future<Output = anyhow::Result<Sensor>> + Send;
}
