pub mod sensor_id;

#[derive(Debug)]
pub struct Sensor {
    sensor_id: String,
    location: String,
    time_stamp: String,
    measurements: String,
    status: String,
}
