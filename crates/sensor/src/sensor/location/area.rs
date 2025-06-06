use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Area {
    Tokyo,
    Osaka,
    Sapporo,
    Fukuoka,
    Nagoya,
}

impl Area {
    pub fn build(value: u8) -> Area {
        match value {
            0..=19 => Area::Tokyo,
            20..=39 => Area::Osaka,
            40..=59 => Area::Sapporo,
            60..=79 => Area::Fukuoka,
            80..=99 => Area::Nagoya,
            _ => Area::Tokyo,
        }
    }
}

#[derive(Error, Debug)]
pub enum AreaError {
    #[error("Invalid Area: {0}")]
    InvalidArea(String),
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Area::Tokyo => write!(f, "Tokyo"),
            Area::Osaka => write!(f, "Osaka"),
            Area::Sapporo => write!(f, "Sapporo"),
            Area::Fukuoka => write!(f, "Fukuoka"),
            Area::Nagoya => write!(f, "Nagoya"),
        }
    }
}

impl FromStr for Area {
    type Err = AreaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tokyo" => Ok(Area::Tokyo),
            "Osaka" => Ok(Area::Osaka),
            "Sapporo" => Ok(Area::Sapporo),
            "Fukuoka" => Ok(Area::Fukuoka),
            "Nagoya" => Ok(Area::Nagoya),
            _ => Err(AreaError::InvalidArea(s.to_owned())),
        }
    }
}
