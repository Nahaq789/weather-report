use humidity::Humidity;
use serde::Serialize;
use temperature::Temperature;

use super::{location::area::Area, season::Season};

pub mod humidity;
pub mod temperature;

#[derive(Debug, Serialize)]
pub struct Measurements {
    temperature: Temperature,
    humidity: Humidity,
}

impl Measurements {
    pub fn new(area: &Area, season: &Season) -> Measurements {
        let temperature = Temperature::new(season, area);
        let humidity = Humidity::new(season, area);

        Measurements {
            temperature,
            humidity,
        }
    }

    pub fn temperature(&self) -> &Temperature {
        &self.temperature
    }

    pub fn humidity(&self) -> &Humidity {
        &self.humidity
    }

    pub fn has_severe_anomaly(&self) -> bool {
        self.temperature.value() < -60.0
            || self.temperature.value() > 60.0
            || self.humidity.value() < 0.0
            || self.humidity.value() > 100.0
    }

    pub fn has_minor_anomaly(&self) -> bool {
        self.temperature.value() < -45.0 || self.temperature.value() > 45.0
    }
}
