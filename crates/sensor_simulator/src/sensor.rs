use location::Location;
use measurements::Measurements;
use sensor_id::SensorId;
use status::Status;
use crate::sensor::location::area::Area;
use crate::sensor::season::Season;

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
    pub fn new(area: &Area, season: &Season) -> Sensor {
        // let sensor_id = SensorId::new(area);
        // let measurements = Measurements::new(area, season);
        // let time_stamp = chrono::Utc::now();
        // let
        Sensor{
            sensor_id: SensorId::new(area),
            measurements: Measurements::new(area, season),
            time_stamp: chrono::Utc::now(),
            location: Location::new(area),

        }
    }
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
