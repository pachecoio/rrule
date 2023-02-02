use crate::frequencies::Frequency;
use crate::recurrences::errors::RecurrenceInvalid;
use crate::recurrences::validations::{validate_duration, validate_recurrence_period};
use chrono::{DateTime, Duration, Utc};
use std::str::FromStr;

const MAX_DATE: &str = "9999-12-31T23:59:59Z";

/// An Iterator-type struct that represents a recurrence of events.
/// It can be used to collect/iterate over all the events that match the recurrence rules
/// between a start and end date.
///
/// Examples:
/// ```
/// use std::str::FromStr;
/// use chrono::{DateTime, Utc};
/// use rrules::{Recurrence, Frequency};
///
/// let once_a_day = Frequency::Daily {interval: 1, by_time: vec![]};
/// let start_date = DateTime::<Utc>::from_str("2023-01-01T12:00:00Z").unwrap();
/// let end_date = DateTime::<Utc>::from_str("2023-01-03T23:59:00Z").unwrap();
///
/// let everyday_during_first_3_days_of_january = Recurrence::new(
///     once_a_day,
///     start_date,
///     Some(end_date),
///     None
/// );
/// let events: Vec<DateTime<Utc>> = everyday_during_first_3_days_of_january
///     .unwrap()
///     .collect();
/// assert_eq!(events.len(), 3);
///
/// ```
pub struct Recurrence {
    /// Represents the frequency rules of the recurrence
    pub frequency: Frequency,

    /// Start date of the recurrences
    ///
    /// This won't necessarily be the first event date, as that depends on the frequencies
    /// configuration defined.
    pub start: DateTime<Utc>,

    /// Current event date to be returned by next()
    /// Starts as None, and is set to the first event date once next() is called
    pub current_date: Option<DateTime<Utc>>,

    /// End date of the recurrences
    pub end: DateTime<Utc>,
    pub duration: Duration,
}

impl Recurrence {
    /// Validates and creates a new Recurrence instance.
    /// Returns an error if the recurrence rules are invalid.
    /// Examples:
    /// ```
    /// use rrules::{Recurrence, Frequency};
    ///
    /// let invalid_frequency = Frequency::Daily {interval: 0, by_time: vec![]};
    /// let recurrence = Recurrence::new(invalid_frequency, chrono::Utc::now(), None, None);
    /// assert!(recurrence.is_err());
    ///
    /// let valid_frequency = Frequency::Daily {interval: 1, by_time: vec![]};
    /// let recurrence = Recurrence::new(valid_frequency, chrono::Utc::now(), None, None);
    /// assert!(recurrence.is_ok());
    /// ```
    pub fn new(
        frequency: Frequency,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
        duration: Option<Duration>,
    ) -> Result<Self, RecurrenceInvalid> {
        let end = end.unwrap_or_else(|| DateTime::<Utc>::from_str(MAX_DATE).unwrap());
        if frequency.is_valid().is_err() {
            return Err(RecurrenceInvalid {
                message: format!("{}", frequency.is_valid().unwrap_err()),
            });
        }
        validate_recurrence_period(&start, &end)?;

        let duration = duration.unwrap_or_else(|| Duration::hours(1));
        validate_duration(&frequency, &duration)?;
        Ok(Recurrence {
            frequency,
            start,
            current_date: None,
            end,
            duration,
        })
    }
}

impl Iterator for Recurrence {
    type Item = DateTime<Utc>;

    /// Returns the next event date in the recurrence.
    /// Returns None if there are no more events in the recurrence.
    /// Examples:
    /// ```
    /// use chrono::DateTime;
    /// use rrules::{Recurrence, Frequency};
    ///
    /// let once_a_day = Frequency::Daily {interval: 1, by_time: vec![]};
    /// let start_date = chrono::Utc::now();
    /// let end_date = start_date + chrono::Duration::days(2);
    ///
    /// let mut  recurrence = Recurrence::new(once_a_day, start_date, Some(end_date), None).unwrap();
    /// let first_event = recurrence.next().unwrap();
    /// assert_eq!(first_event, start_date);
    ///
    /// let second_event = recurrence.next().unwrap();
    /// assert_eq!(second_event, start_date + chrono::Duration::days(1));
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let current_date = match self.current_date {
            None => {
                // If current_date is None, this is the first call to next()
                if self.frequency.contains(&self.start) {
                    self.current_date = Some(self.start);
                    return Some(self.start);
                }
                self.start
            }
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
            None => None,
        }
    }
}
