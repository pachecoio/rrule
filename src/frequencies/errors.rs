use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FrequencyErrors {
    InvalidInterval {
        message: String,
    },
    InvalidTime {
        message: String,
    },
}

impl Display for FrequencyErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FrequencyErrors::InvalidInterval { message } => write!(f, "Invalid interval: {message}"),
            FrequencyErrors::InvalidTime { message } => write!(f, "Invalid time: {message}"),
        }
    }
}
