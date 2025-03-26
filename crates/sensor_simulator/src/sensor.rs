use location::Location;
use sensor_id::SensorId;
use status::Status;

pub mod location;
pub mod sensor_id;
pub mod status;

#[derive(Debug)]
pub struct Sensor {
    sensor_id: SensorId,
    location: Location,
    time_stamp: chrono::Utc,
    measurements: String,
    status: Status,
}
