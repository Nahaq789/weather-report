pub trait SensorRepository {
    fn get_sensor_data(
        &self,
        area: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<String>>> + Send;
}
