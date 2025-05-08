use core::f64;
use std::fmt;

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::area::Area;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Longitude {
    value: f64,
}

impl Longitude {
    pub fn new(area: &Area) -> Longitude {
        let mut rng = rand::thread_rng();

        let x = match area {
            Area::Tokyo => rng.gen_range(139.6..139.9),
            Area::Osaka => rng.gen_range(135.4..135.6),
            Area::Sapporo => rng.gen_range(141.3..141.4),
            Area::Fukuoka => rng.gen_range(130.3..130.5),
            Area::Nagoya => rng.gen_range(136.9..137.0),
        };

        Longitude { value: x }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for Longitude {
    fn from(value: String) -> Self {
        let v = value.parse::<f64>().unwrap();
        Longitude { value: v }
    }
}
