use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum InvalidFrequency {
    Interval { message: String },
    Time { message: String },
    Day { message: String },
}

impl Display for InvalidFrequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidFrequency::Interval { message } => {
                write!(f, "Invalid interval: {message}")
            }
            InvalidFrequency::Time { message } => write!(f, "Invalid time: {message}"),
            InvalidFrequency::Day { message } => write!(f, "Invalid day: {message}"),
        }
    }
}
