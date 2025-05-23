use core::f64;
use std::fmt;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    generate_anomalies,
    sensor::{location::area::Area, season::Season},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Humidity {
    value: f64,
}

impl Humidity {
    pub fn new(season: &Season, area: &Area) -> Humidity {
        let mut rng = rand::thread_rng();
        let base_range = match (season, area) {
            (Season::Spring, Area::Tokyo) => (50.0, 65.0),
            (Season::Summer, Area::Tokyo) => (70.0, 85.0),
            (Season::Autumn, Area::Tokyo) => (60.0, 75.0),
            (Season::Winter, Area::Tokyo) => (40.0, 60.0),

            (Season::Spring, Area::Osaka) => (55.0, 65.0),
            (Season::Summer, Area::Osaka) => (65.0, 80.0),
            (Season::Autumn, Area::Osaka) => (60.0, 70.0),
            (Season::Winter, Area::Osaka) => (50.0, 65.0),

            (Season::Spring, Area::Nagoya) => (55.0, 70.0),
            (Season::Summer, Area::Nagoya) => (70.0, 85.0),
            (Season::Autumn, Area::Nagoya) => (65.0, 75.0),
            (Season::Winter, Area::Nagoya) => (55.0, 65.0),

            (Season::Spring, Area::Sapporo) => (60.0, 70.0),
            (Season::Summer, Area::Sapporo) => (65.0, 80.0),
            (Season::Autumn, Area::Sapporo) => (65.0, 75.0),
            (Season::Winter, Area::Sapporo) => (60.0, 75.0),

            (Season::Spring, Area::Fukuoka) => (60.0, 70.0),
            (Season::Summer, Area::Fukuoka) => (70.0, 85.0),
            (Season::Autumn, Area::Fukuoka) => (65.0, 75.0),
            (Season::Winter, Area::Fukuoka) => (55.0, 70.0),
        };

        let value = rng.gen_range(base_range.0..=base_range.1);

        let final_value = generate_anomalies(value, &mut rng);
        Humidity { value: final_value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Humidity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for Humidity {
    fn from(value: String) -> Self {
        let v = value.parse::<f64>().unwrap();
        Humidity { value: v }
    }
}
