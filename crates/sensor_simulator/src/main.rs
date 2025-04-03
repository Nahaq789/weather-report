use sensor_simulator::sensor::{
    location::area::Area,
    season::{self, Season},
    Sensor,
};

fn main() {
    let area = Area::Tokyo;
    let season = Season::Spring;
    let sensor = Sensor::new(&area, &season);
    println!("{:?}", sensor);
    println!("Hello, world!");
}
