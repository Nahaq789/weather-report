#[derive(Debug)]
pub struct Sensor {
    pub sensor_id: String,
    pub time_stamp: chrono::DateTime<chrono::Utc>,
    pub area: String,
    pub latitude: f64,
    pub longitude: f64,
    pub temperature: f64,
    pub humidity: f64,
    pub status: String,
}

impl Sensor {
    pub fn new(
        sensor_id: String,
        time_stamp: chrono::DateTime<chrono::Utc>,
        area: String,
        latitude: f64,
        longitude: f64,
        temperature: f64,
        humidity: f64,
        status: String,
    ) -> Self {
        Self {
            sensor_id,
            time_stamp,
            area,
            latitude,
            longitude,
            temperature,
            humidity,
            status,
        }
    }
}
