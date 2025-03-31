use temperature::Temperature;

pub mod humidity;
pub mod temperature;

#[derive(Debug)]
pub struct Measurements {
    temperature: Temperature,
}
