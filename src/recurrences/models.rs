use std::fmt::{Display, Formatter};
use std::str::FromStr;
use chrono::{Datelike, DateTime, Duration, Timelike, Utc, Weekday};
use crate::frequencies::{Frequency, NthWeekday, Time};
use crate::recurrences::errors::RecurrenceInvalid;
use crate::recurrences::validations::{validate_duration, validate_recurrence_period};
use crate::utils::{DateUtils, get_next_nth_weekday};

const MAX_DATE: &str = "9999-12-31T23:59:59Z";

pub struct Recurrence {
    pub(crate) frequency: Frequency,

    /// Start date of the recurrences
    ///
    /// This won't necessarily be the first event date, as that depends on the frequencies
    /// configuration defined.
    pub(crate) start: DateTime<Utc>,

    /// Current event date to be returned by next()
    /// Starts as None, and is set to the first event date once next() is called
    pub(crate) current_date: Option<DateTime<Utc>>,

    /// End date of the recurrences
    pub(crate) end: DateTime<Utc>,
    duration: Duration
}

impl Recurrence {
    pub fn new(frequency: Frequency, start: DateTime<Utc>, end: Option<DateTime<Utc>>, duration: Option<Duration>) -> Result<Self, RecurrenceInvalid> {
        let end = end.unwrap_or_else(|| DateTime::<Utc>::from_str(MAX_DATE).unwrap());
        validate_recurrence_period(&start, &end)?;

        let duration = duration.unwrap_or_else(|| Duration::hours(1));
        validate_duration(&frequency, &duration)?;
        Ok(Recurrence {
            frequency,
            start,
            current_date: None,
            end,
            duration
        })
    }
}

impl Iterator for Recurrence {
    type Item = DateTime<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_date = match self.current_date {
            None => {
                // If current_date is None, this is the first call to next()
                if self.frequency.contains(&self.start) {
                    self.current_date = Some(self.start);
                    return Some(self.start);
                }
                self.start
            },
            Some(current_date) => current_date,
        };
        // Get the next event date based on the current date and frequencies
        match self.frequency.next_event(&current_date) {
            Some(next_event) => {
                if next_event > self.end {
                    return None;
                }
                self.current_date = Some(next_event);
                Some(next_event)
            }
            None => None
        }
    }
}
