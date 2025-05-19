use std::usize;

use ::sensor::sensor::Sensor;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[cfg(feature = "not_docker")]
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorDto {
    location: String,
    time_stamp: DateTime<Local>,
    aggregate: Aggregate,
    status: Option<Status>,
}

impl SensorDto {
    pub fn build(
        location: String,
        time_stamp: DateTime<Local>,
        aggregate: Aggregate,
        status: Option<Status>,
    ) -> SensorDto {
        SensorDto {
            location,
            time_stamp,
            aggregate,
            status,
        }
    }

    #[cfg(feature = "not_docker")]
    pub fn generate() -> SensorDto {
        let mut rng = rand::thread_rng();

        let location = sensor::sensor::location::area::Area::build(rng.gen_range(0..99));
        let time_stamp = Local::now();
        let aggregate = Aggregate::gen_rand_aggregate(&mut rng);

        SensorDto::build(location.to_string(), time_stamp, aggregate, None)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aggregate {
    temperature: Temperature,
    humidity: Humidity,
}

impl Aggregate {
    pub fn build(t_vec: Vec<f64>, h_vec: Vec<f64>) -> Aggregate {
        let t = Temperature::new(
            Sensor::average(&t_vec),
            Sensor::min(&t_vec),
            Sensor::max(&t_vec),
            Sensor::mid(t_vec),
        );

        let h = Humidity::new(
            Sensor::average(&h_vec),
            Sensor::min(&h_vec),
            Sensor::max(&h_vec),
            Sensor::mid(h_vec),
        );

        Aggregate {
            temperature: t,
            humidity: h,
        }
    }

    #[cfg(feature = "not_docker")]
    pub fn gen_rand_aggregate(rng: &mut ThreadRng) -> Aggregate {
        let t = Temperature::new(
            rng.gen_range(20.0..25.0),
            rng.gen_range(18.0..22.0),
            rng.gen_range(25.0..28.0),
            rng.gen_range(23.0..25.0),
        );

        let h = Humidity::new(
            rng.gen_range(40.0..60.0),
            rng.gen_range(35.0..45.0),
            rng.gen_range(55.0..65.0),
            rng.gen_range(45.0..55.0),
        );

        Aggregate {
            temperature: t,
            humidity: h,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    avg: f64,
    min: f64,
    max: f64,
    mid: f64,
}

impl Temperature {
    fn new(avg: f64, min: f64, max: f64, mid: f64) -> Temperature {
        Temperature { avg, min, max, mid }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Humidity {
    avg: f64,
    min: f64,
    max: f64,
    mid: f64,
}

impl Humidity {
    fn new(avg: f64, min: f64, max: f64, mid: f64) -> Humidity {
        Humidity { avg, min, max, mid }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    error_count: usize,
    last_error_time: DateTime<Local>,
    errors: Vec<Sensor>,
}

impl Status {
    pub fn new(count: usize, time: DateTime<Local>, errors: Vec<Sensor>) -> Status {
        Status {
            error_count: count,
            last_error_time: time,
            errors,
        }
    }
}
