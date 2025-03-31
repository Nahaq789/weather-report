use rand::Rng;

use crate::sensor::{location::area::Area, season::Season};

#[derive(Debug)]
pub struct Temperature {
    value: f64,
}

impl Temperature {
    pub fn new(season: Season, location: Area) -> Temperature {
        let mut rng = rand::thread_rng();
        let base_range = match (location, season) {
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

        // noise
        let noise = rng.gen_range(0.95..=1.05);
        let value_with_noise = value * noise;

        // Generation of anomalies
        let final_value = if rng.gen_bool(0.05) {
            let anomaly_factor = if rng.gen_bool(0.5) {
                rng.gen_range(1.15..=1.2)
            } else {
                rng.gen_range(0.8..=0.85)
            };
            value_with_noise * anomaly_factor
        } else {
            value_with_noise
        };

        Temperature { value: final_value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
