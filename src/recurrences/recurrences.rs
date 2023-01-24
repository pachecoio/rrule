use std::fmt::{Display, Formatter};
use std::str::FromStr;
use chrono::{DateTime, Duration, Utc};
use crate::frequencies::{Frequency, Time};

const MAX_DATE: &str = "9999-12-31T23:59:59Z";

pub struct Recurrence {
    frequency: Frequency,

    /// Start date of the recurrences
    ///
    /// This won't necessarily be the first event date, as that depends on the frequencies
    /// configuration defined.
    start: DateTime<Utc>,

    /// Current event date to be returned by next()
    /// Starts as None, and is set to the first event date once next() is called
    current_date: Option<DateTime<Utc>>,

    /// End date of the recurrences
    end: DateTime<Utc>,
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

fn validate_recurrence_period(start: &DateTime<Utc>, end: &DateTime<Utc>) -> Result<(), RecurrenceInvalid> {
    if start >= end {
        return Err(RecurrenceInvalid {
            message: "Start date must be before end date".to_string()
        });
    }
    Ok(())
}

fn validate_duration(frequency: &Frequency, duration: &Duration) -> Result<(), RecurrenceInvalid> {
    match frequency {
        Frequency::Secondly { interval } => {
            let seconds = duration.num_seconds();
            if seconds > *interval as i64 {
                return Err(RecurrenceInvalid {
                    message: "Duration must be smaller than interval".to_string()
                });
            }
        }
        Frequency::Minutely { .. } => {}
        Frequency::Hourly { .. } => {}
        Frequency::Daily { .. } => {}
        Frequency::Weekly { .. } => {}
        Frequency::Monthly { .. } => {}
    }
    Ok(())
}

#[derive(Debug)]
pub struct RecurrenceInvalid {
    message: String,
}

impl Display for RecurrenceInvalid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
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

