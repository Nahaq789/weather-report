use rand::Rng;

use super::area::Area;

#[derive(Debug)]
pub struct Latitude {
    value: f64,
}

impl Latitude {
    pub fn new(area: &Area) -> Latitude {
        let mut rng = rand::thread_rng();

        let v = match area {
            Area::Tokyo => rng.gen_range(35.6..35.8),
            Area::Osaka => rng.gen_range(34.6..34.7),
            Area::Sapporo => rng.gen_range(43.0..43.1),
            Area::Fukuoka => rng.gen_range(33.5..33.6),
            Area::Nagoya => rng.gen_range(35.1..35.2),
        };

        Latitude { value: v }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
