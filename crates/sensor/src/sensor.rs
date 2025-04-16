use crate::sensor::location::area::Area;
use crate::sensor::season::Season;
use chrono::DateTime;
use location::Location;
use measurements::Measurements;
use sensor_id::SensorId;
use serde::{Deserialize, Serialize};
use status::Status;

pub mod location;
pub mod measurements;
pub mod season;
pub mod sensor_id;
pub mod status;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensor {
    sensor_id: SensorId,
    location: Location,
    time_stamp: DateTime<chrono::Local>,
    measurements: Measurements,
    status: Status,
}

impl Sensor {
    pub fn new(area: &Area, season: &Season) -> Sensor {
        let measurements = Measurements::new(area, season);
        let status = Self::determine_status(&measurements);

        Sensor {
            sensor_id: SensorId::new(area),
            location: Location::new(area.clone()),
            time_stamp: chrono::Local::now(),
            measurements,
            status,
        }
    }
    fn determine_status(measurements: &Measurements) -> Status {
        if measurements.has_severe_anomaly() {
            return Status::Error;
        }

        if measurements.has_minor_anomaly() {
            return Status::Warning;
        }
        Status::Normal
    }

    pub fn sensor_id(&self) -> &SensorId {
        &self.sensor_id
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn time_stamp(&self) -> &DateTime<chrono::Local> {
        &self.time_stamp
    }

    pub fn measurements(&self) -> &Measurements {
        &self.measurements
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn from_str(s: &str) -> Result<Sensor, serde_json::Error> {
        serde_json::from_str(s)
    }
}
