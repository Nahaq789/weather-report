use core::f64;

use crate::sensor::location::area::Area;
use crate::sensor::season::Season;
use chrono::{DateTime, Local};
use location::Location;
use measurements::Measurements;
use sensor_id::SensorId;
use serde::{Deserialize, Serialize};
use status::Status;

pub mod location;
pub mod measurements;
pub mod season;
pub mod sensor_id;
pub mod status;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensor {
    sensor_id: SensorId,
    location: Location,
    time_stamp: DateTime<chrono::Local>,
    measurements: Measurements,
    status: Status,
}

impl Sensor {
    pub fn new(area: &Area, season: &Season) -> Sensor {
        let measurements = Measurements::new(area, season);
        let status = Self::determine_status(&measurements);

        Sensor {
            sensor_id: SensorId::new(area),
            location: Location::new(area.clone()),
            time_stamp: chrono::Local::now(),
            measurements,
            status,
        }
    }

    fn determine_status(measurements: &Measurements) -> Status {
        if measurements.has_severe_anomaly() {
            return Status::Error;
        }

        if measurements.has_minor_anomaly() {
            return Status::Warning;
        }
        Status::Normal
    }

    pub fn from(
        sensor_id: SensorId,
        location: Location,
        time_stamp: DateTime<Local>,
        measurements: Measurements,
        status: Status,
    ) -> Sensor {
        Sensor {
            sensor_id,
            location,
            time_stamp,
            measurements,
            status,
        }
    }

    pub fn sensor_id(&self) -> &SensorId {
        &self.sensor_id
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn time_stamp(&self) -> &DateTime<chrono::Local> {
        &self.time_stamp
    }

    pub fn measurements(&self) -> &Measurements {
        &self.measurements
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn from_str(s: &str) -> Result<Sensor, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn average(values: Vec<f64>) -> f64 {
        let total: f64 = values.iter().sum();
        let result = total / values.len() as f64;
        result.round()
    }

    pub fn max(values: Vec<f64>) -> f64 {
        let mut max = 0.0;
        if values.is_empty() {
            return max;
        }

        for v in values {
            if max < v {
                max = v;
            }
        }
        max
    }

    pub fn min(values: Vec<f64>) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        let mut min: f64 = values[0];

        for v in values {
            if min > v {
                min = v
            }
        }
        min
    }
    pub fn sort(values: Vec<f64>) -> Vec<f64> {
        Self::merge_sort(values)
    }

    fn merge_sort(values: Vec<f64>) -> Vec<f64> {
        if values.len() <= 1 {
            return values;
        }

        let mid = values.len() / 2;
        let left = Self::merge_sort(values[0..mid].to_vec());
        let right = Self::merge_sort(values[mid..].to_vec());
        Self::merge(left, right)
    }

    fn merge(left: Vec<f64>, right: Vec<f64>) -> Vec<f64> {
        let mut result = Vec::new();

        let (mut li, mut ri) = (0, 0);
        while li < left.len() && ri < right.len() {
            if left[li] < right[ri] {
                result.push(left[li]);
                li += 1;
            } else {
                result.push(right[ri]);
                ri += 1;
            }
        }

        while li < left.len() {
            result.push(left[li]);
            li += 1;
        }

        while ri < right.len() {
            result.push(right[ri]);
            ri += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_success() {
        let nums = vec![5.0, 0.0, 8.0];

        let actual = Sensor::sort(nums);
        let expected = vec![0.0, 5.0, 8.0];
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_average_success() {
        let test_case = vec![(vec![7.0, 0.0, 8.0], 5.0), (vec![1.0, 2.0, 5.0], 2.6)];

        for test in test_case {
            let actual = Sensor::average(test.0);
            assert_eq!(actual, test.1)
        }
    }
}
