use area::Area;
use latitude::Latitude;
use longitude::Longitude;

pub mod area;
pub mod latitude;
pub mod longitude;

#[derive(Debug)]
pub struct Location {
    name: Area,
    latitude: Latitude,
    longitude: Longitude,
}

impl Location {
    pub fn new(name: Area) -> Location {
        let latitude = Latitude::new(&name);
        let longitude = Longitude::new(&name);

        Location {
            name,
            latitude,
            longitude,
        }
    }

    pub fn name(&self) -> &Area {
        &self.name
    }

    pub fn latitude(&self) -> &Latitude {
        &self.latitude
    }

    pub fn longitude(&self) -> &Longitude {
        &self.longitude
    }
}
