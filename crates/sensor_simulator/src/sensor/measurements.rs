use humidity::Humidity;
use temperature::Temperature;

use super::{location::area::Area, season::Season};

pub mod humidity;
pub mod temperature;

#[derive(Debug)]
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
}
