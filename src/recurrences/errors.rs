use std::fmt::{Display, Formatter};
use chrono::{DateTime, Utc};
use crate::recurrences::Recurrence;

#[derive(Debug)]
pub struct RecurrenceInvalid {
    pub(crate) message: String,
}

impl Display for RecurrenceInvalid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

