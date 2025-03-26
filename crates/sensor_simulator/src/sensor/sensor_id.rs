use std::str::FromStr;

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub struct SensorId {
    value: String,
}

#[derive(Error, Debug)]
pub enum SensorIdError {
    #[error("It is not in the prefix_UUID format.")]
    InvalidFormat,
}

impl SensorId {
    pub fn new() -> SensorId {
        let id = Uuid::new_v4();
        SensorId {
            value: id.to_string(),
        }
    }
}

impl From<Uuid> for SensorId {
    fn from(value: Uuid) -> Self {
        SensorId {
            value: value.to_string(),
        }
    }
}

impl FromStr for SensorId {
    type Err = SensorIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s);
        match uuid {
            Ok(u) => Ok(SensorId::from(u)),
            Err(_) => return Err(SensorIdError::InvalidFormat),
        }
    }
}
