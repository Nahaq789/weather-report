use core::f64;
use std::fmt;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    generate_anomalies,
    sensor::{location::area::Area, season::Season},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    value: f64,
}

impl Temperature {
    pub fn new(season: &Season, area: &Area) -> Temperature {
        let mut rng = rand::thread_rng();
        let base_range = match (area, season) {
            (Area::Tokyo, Season::Spring) => (10.0, 22.0),
            (Area::Tokyo, Season::Summer) => (25.0, 35.0),
            (Area::Tokyo, Season::Autumn) => (15.0, 25.0),
            (Area::Tokyo, Season::Winter) => (2.0, 12.0),

            (Area::Osaka, Season::Spring) => (12.0, 23.0),
            (Area::Osaka, Season::Summer) => (26.0, 36.0),
            (Area::Osaka, Season::Autumn) => (16.0, 26.0),
            (Area::Osaka, Season::Winter) => (3.0, 13.0),

            (Area::Nagoya, Season::Spring) => (11.0, 23.0),
            (Area::Nagoya, Season::Summer) => (26.0, 36.0),
            (Area::Nagoya, Season::Autumn) => (16.0, 26.0),
            (Area::Nagoya, Season::Winter) => (3.0, 12.0),

            (Area::Sapporo, Season::Spring) => (3.0, 15.0),
            (Area::Sapporo, Season::Summer) => (17.0, 28.0),
            (Area::Sapporo, Season::Autumn) => (8.0, 18.0),
            (Area::Sapporo, Season::Winter) => (-8.0, 2.0),

            (Area::Fukuoka, Season::Spring) => (13.0, 23.0),
            (Area::Fukuoka, Season::Summer) => (25.0, 34.0),
            (Area::Fukuoka, Season::Autumn) => (17.0, 27.0),
            (Area::Fukuoka, Season::Winter) => (5.0, 15.0),
        };

        let value = rng.gen_range(base_range.0..=base_range.1);

        let final_value = generate_anomalies(value, &mut rng);

        Temperature { value: final_value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for Temperature {
    fn from(value: String) -> Self {
        let v = value.parse::<f64>().unwrap();
        Temperature { value: v }
    }
}
