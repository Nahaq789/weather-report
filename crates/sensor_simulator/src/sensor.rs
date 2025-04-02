use location::Location;
use measurements::Measurements;
use sensor_id::SensorId;
use status::Status;

pub mod location;
pub mod measurements;
pub mod season;
pub mod sensor_id;
pub mod status;

#[derive(Debug)]
pub struct Sensor {
    sensor_id: SensorId,
    location: Location,
    time_stamp: chrono::Utc,
    measurements: Measurements,
    status: Status,
}
