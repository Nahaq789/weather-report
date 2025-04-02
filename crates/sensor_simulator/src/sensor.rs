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

impl Sensor {
    fn determine_status(&self) -> Status {
        if self.measurements.has_severe_anomaly() {
            return Status::Error;
        }

        if self.measurements.has_minor_anomaly() {
            return Status::Warning;
        }
        Status::Normal
    }
}
