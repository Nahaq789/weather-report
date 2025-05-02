use chrono::{DateTime, Local};
use sensor::sensor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorDto {
    location: String,
    time_stamp: DateTime<Local>,
    aggregate: Aggregate,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aggregate {
    temperature: Temperature,
    humidity: Humidity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    avg: f64,
    min: f64,
    max: f64,
    mid: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Humidity {
    avg: f64,
    min: f64,
    max: f64,
    mid: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    error_count: i32,
    last_error_time: DateTime<Local>,
    errors: Vec<sensor::Sensor>,
}
