use area::Area;
use latitude::Latitude;
use longitude::Longitude;
use serde::{Deserialize, Serialize};

pub mod area;
pub mod latitude;
pub mod longitude;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    area: Area,
    latitude: Latitude,
    longitude: Longitude,
}

impl Location {
    pub fn new(area: Area) -> Location {
        let latitude = Latitude::new(&area);
        let longitude = Longitude::new(&area);

        Location {
            area,
            latitude,
            longitude,
        }
    }

    pub fn from(area: Area, latitude: Latitude, longitude: Longitude) -> Location {
        Location {
            area,
            latitude,
            longitude,
        }
    }

    pub fn area(&self) -> &Area {
        &self.area
    }

    pub fn latitude(&self) -> &Latitude {
        &self.latitude
    }

    pub fn longitude(&self) -> &Longitude {
        &self.longitude
    }
}
