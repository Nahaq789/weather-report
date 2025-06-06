use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use super::location::area::Area;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct SensorId {
    value: String,
}

#[derive(Error, Debug)]
pub enum SensorIdError {
    #[error("It is not in the prefix_UUID format.")]
    InvalidFormat,
}

impl SensorId {
    pub fn new(area: &Area) -> SensorId {
        let id = Uuid::new_v4();
        SensorId {
            value: format!("{}_{}", area.to_string(), id),
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

impl From<String> for SensorId {
    fn from(value: String) -> Self {
        SensorId { value }
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

impl fmt::Display for SensorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
