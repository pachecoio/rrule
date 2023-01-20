use std::fmt::{Display, Formatter};
use std::ops::Add;
use chrono::{DateTime, Utc};

enum Frequency {
    Secondly {
        interval: i32,
    },
}

impl Frequency {
    fn is_valid(&self) -> Result<(), FrequencyErrors> {
        match self {
            Frequency::Secondly { interval } => validate_secondly(interval),
        }
    }

    /// Returns the next event date for the current frequency config given the current date.
    pub(crate) fn next_event(&self, current_date: &DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            Frequency::Secondly { interval } => {
                let next_date = current_date.add(chrono::Duration::seconds(*interval as i64));
                Some(next_date)
            }
        }
    }
}

fn validate_secondly(interval: &i32) -> Result<(), FrequencyErrors> {
    if *interval > 0 {
        Ok(())
    } else {
        Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        })
    }
}

enum FrequencyErrors {
    InvalidInterval {
        message: String,
    },
}

impl Display for FrequencyErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FrequencyErrors::InvalidInterval { message } => write!(f, "Invalid interval: {}", message),
        }
    }
}

#[cfg(test)]
mod secondly_frequency {
    use std::str::FromStr;
    use chrono::{Timelike, Utc};
    use super::*;

    #[test]
    fn every_second_frequency() {
        let f = Frequency::Secondly { interval: 1 };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Secondly { interval: 0 };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Secondly { interval: -1 };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_second_collect_events() {
        let f = Frequency::Secondly { interval: 1 };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().second(), now.second() + 1);
    }

    #[test]
    fn collect_events_that_span_to_another_minute() {
        let f = Frequency::Secondly { interval: 30 };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().second(), 29);
    }
}