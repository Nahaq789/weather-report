use std::f32;

use area::Area;
use latitude::Latitude;

pub mod area;
pub mod latitude;
pub mod longitude;

#[derive(Debug)]
pub struct Location {
    name: Area,
    latitude: Latitude,
    longitude: f32,
}

impl Location {
    pub fn new(name: Area, latitude: Latitude, longitude: f32) -> Location {
        Location {
            name,
            latitude,
            longitude,
        }
    }
}
