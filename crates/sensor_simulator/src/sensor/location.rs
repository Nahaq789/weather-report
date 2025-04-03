use area::Area;
use latitude::Latitude;
use longitude::Longitude;

pub mod area;
pub mod latitude;
pub mod longitude;

#[derive(Debug)]
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
