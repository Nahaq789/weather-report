use std::{
    fmt::Display,
    str::{self},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Normal,
    Warning,
    Error,
    Maintenance,
}

impl Status {
    pub fn from_str(status: &str) -> Status {
        match status {
            "normal" => Status::Normal,
            "warning" => Status::Warning,
            "error" => Status::Error,
            "maintenance" => Status::Maintenance,
            _ => Status::Error,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Normal => write!(f, "normal"),
            Status::Warning => write!(f, "warning"),
            Status::Error => write!(f, "error"),
            Status::Maintenance => write!(f, "maintenance"),
        }
    }
}
