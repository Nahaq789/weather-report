use sensor_id::SensorId;

pub mod location;
pub mod sensor_id;

#[derive(Debug)]
pub struct Sensor {
    sensor_id: SensorId,
    location: String,
    time_stamp: String,
    measurements: String,
    status: String,
}
