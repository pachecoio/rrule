use std::fmt::{Display, Formatter};
use std::str::FromStr;
use chrono::{DateTime, Duration, Utc};
use crate::frequencies::Frequency;

const MAX_DATE: &str = "9999-12-31T23:59:59Z";

pub struct Recurrence {
    frequency: Frequency,

    /// Start date of the recurrence
    ///
    /// This won't necessarily be the first event date, as that depends on the frequency
    /// configuration defined.
    start: DateTime<Utc>,

    /// Current event date to be returned by next()
    /// Starts as None, and is set to the first event date once next() is called
    current_date: Option<DateTime<Utc>>,

    /// End date of the recurrence
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
                self.current_date = Some(self.start);
                return Some(self.start);
            },
            Some(current_date) => current_date,
        };
        // Get the next event date based on the current date and frequency
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

#[cfg(test)]
mod tests {
    use super::*;

    fn every_second_recurrence(start: DateTime<Utc>, end: Option<DateTime<Utc>>, duration: Option<Duration>) -> Result<Recurrence, RecurrenceInvalid> {
        let duration = duration.unwrap_or_else(|| Duration::seconds(1));
        Recurrence::new(
            Frequency::Secondly {
                interval: 1,
            },
            start,
            end,
            Some(duration)
        )
    }

    #[test]
    fn test_new_secondly_recurrence() {
        let recurrence = every_second_recurrence(
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            Some(DateTime::<Utc>::from_str("2023-01-01T00:00:02Z").unwrap()),
            None
        );
        assert!(recurrence.is_ok());
    }

    #[test]
    fn invalid_period() {
        let recurrence = every_second_recurrence(
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            Some(DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap()),
            None
        );
        assert!(recurrence.is_err());
    }

    #[test]
    fn invalid_duration() {
        let recurrence = every_second_recurrence(
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            Some(DateTime::<Utc>::from_str("2023-01-01T00:00:10Z").unwrap()),
            Some(Duration::hours(1))
        );
        assert!(recurrence.is_err());
    }
}

#[cfg(test)]
mod secondly_recurrences {
    use std::str::FromStr;
    use chrono::{Datelike, Timelike};
    use super::*;

    #[test]
    fn every_second() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-01-01T00:00:02Z").unwrap();
        let frequency = Frequency::Secondly {
            interval: 1,
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::seconds(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 3);
    }
}
