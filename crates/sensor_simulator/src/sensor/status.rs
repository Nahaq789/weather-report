use std::{
    fmt::Display,
    str::{self},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Normal,
    Abnormal,
    Maintenance,
}

impl Status {
    pub fn from_str(status: &str) -> Status {
        match status {
            "normal" => Status::Normal,
            "abnormal" => Status::Abnormal,
            "maintenance" => Status::Maintenance,
            _ => Status::Abnormal,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Normal => write!(f, "normal"),
            Status::Abnormal => write!(f, "abnormal"),
            Status::Maintenance => write!(f, "maintenance"),
        }
    }
}
