use std::fmt::{Display, Formatter};
use std::ops::Add;
use chrono::{DateTime, Timelike, Utc};

pub enum Frequency {
    Secondly {
        interval: i32,
    },
    Minutely {
        interval: i32,
    },
    Hourly {
        interval: i32,
    },
    Daily {
        interval: i32,
        by_time: Vec<Time>
    },
}

pub struct Time {
    pub hour: i32,
    pub minute: i32,
}

impl Time {
    pub(crate) fn from_str(time_str: &str) -> Result<Self, FrequencyErrors> {
        let mut parts = time_str.split(':');
        let hour = match parts.next() {
            None => return Err(FrequencyErrors::InvalidTime {
                message: format!("Invalid time: {}", time_str),
            }),
            Some(hour) => hour.parse::<i32>().unwrap()
        };
        let minute = match parts.next() {
            None => return Err(FrequencyErrors::InvalidTime {
                message: format!("Invalid time: {}", time_str)
            }),
            Some(minute) => minute.parse::<i32>().unwrap()
        };
        Ok(Time {
            hour,
            minute,
        })
    }
}

impl Frequency {
    fn is_valid(&self) -> Result<(), FrequencyErrors> {
        match self {
            Frequency::Secondly { interval } => validate_secondly(interval),
            Frequency::Minutely { interval } => validate_minutely(interval),
            Frequency::Hourly { interval } => validate_hourly(interval),
            Frequency::Daily { interval, by_time } => validate_daily(interval, by_time),
        }
    }

    /// Returns the next event date for the current frequency config given the current date.
    pub(crate) fn next_event(&self, current_date: &DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            Frequency::Secondly { interval } => {
                let next_date = current_date.add(chrono::Duration::seconds(*interval as i64));
                Some(next_date)
            },
            Frequency::Minutely { interval } => {
                let next_date = current_date.add(chrono::Duration::minutes(*interval as i64));
                Some(next_date)
            },
            Frequency::Hourly { interval } => {
                let next_date = current_date.add(chrono::Duration::hours(*interval as i64));
                Some(next_date)
            },
            Frequency::Daily { interval, by_time } => next_daily_event(
                current_date, *interval, &by_time
            ),
        }
    }
}

fn next_daily_event(current_date: &DateTime<Utc>, interval: i32, by_time: &Vec<Time>) -> Option<DateTime<Utc>> {
    let mut next_date = current_date.add(chrono::Duration::days(interval as i64));

    if !by_time.is_empty() {
        for time in by_time {
            let d = current_date
                .with_hour(time.hour as u32)
                .unwrap()
                .with_minute(time.minute as u32)
                .unwrap();
            if d > *current_date {
                return Some(d);
            }
        }

        // No hours left in the day, so we need to add a day
        next_date = next_date
            .with_hour(by_time[0].hour as u32).unwrap()
            .with_minute(by_time[0].minute as u32).unwrap();
    }
    Some(next_date)
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

fn validate_minutely(interval: &i32) -> Result<(), FrequencyErrors> {
    if *interval > 0 {
        Ok(())
    } else {
        Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        })
    }
}

fn validate_hourly(interval: &i32) -> Result<(), FrequencyErrors> {
    if *interval > 0 {
        Ok(())
    } else {
        Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        })
    }
}

fn validate_daily(interval: &i32, by_time: &Vec<Time>) -> Result<(), FrequencyErrors> {
    if *interval <= 0 {
        return Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        });
    }
    let mut total_hours = by_time.iter().fold(0, |acc, time| acc + time.hour);
    let mut total_minutes = by_time.iter().fold(0, |acc, time| acc + time.minute);
    if total_minutes >= 60 {
        total_hours = total_hours + (total_minutes / 60);
        total_minutes = total_minutes % 60;
    }
    if total_hours > 24 {
        return Err(FrequencyErrors::InvalidTime {
            message: "Total hours must be less than 24".to_string(),
        });
    }
    Ok(())
}

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
            FrequencyErrors::InvalidInterval { message } => write!(f, "Invalid interval: {}", message),
            FrequencyErrors::InvalidTime { message } => write!(f, "Invalid time: {}", message),
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

        let next_event = f.next_event(&next_event.unwrap());
        assert_eq!(next_event.unwrap().second(), 59);
    }
}

#[cfg(test)]
mod minutely_frequency {
    use std::str::FromStr;
    use chrono::Timelike;
    use super::*;

    #[test]
    fn every_minute_frequency() {
        let f = Frequency::Minutely { interval: 1 };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Minutely { interval: 0 };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Minutely { interval: -1 };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_minute_collect_events() {
        let f = Frequency::Minutely { interval: 1 };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().minute(), now.minute() + 1);
    }

    #[test]
    fn collect_events_that_span_to_another_hour() {
        let f = Frequency::Minutely { interval: 30 };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().minute(), 30);

        let next_event = f.next_event(&next_event.unwrap());
        assert_eq!(next_event.unwrap().minute(), 0);
    }
}

#[cfg(test)]
mod hourly_frequency {
    use std::str::FromStr;
    use chrono::{Datelike, Timelike};
    use super::*;

    #[test]
    fn every_hour_frequency() {
        let f = Frequency::Hourly { interval: 1 };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Hourly { interval: 0 };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Hourly { interval: -1 };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_hour_collect_events() {
        let f = Frequency::Hourly { interval: 1 };
        let now = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().hour(), 1);
    }

    #[test]
    fn collect_events_that_span_to_another_day() {
        let f = Frequency::Hourly { interval: 12 };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().hour(), 12);

        let next_event = f.next_event(&next_event.unwrap());
        assert_eq!(next_event.unwrap().hour(), 0);
        assert_eq!(next_event.unwrap().day(), 2);
    }
}

#[cfg(test)]
mod daily_frequency {
    use std::str::FromStr;
    use chrono::Datelike;
    use super::*;

    #[test]
    fn every_day_frequency() {
        let f = Frequency::Daily {
            interval: 1,
            by_time: vec![]
        };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Daily { interval: 0, by_time: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Daily { interval: -1, by_time: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_day_collect_events() {
        let f = Frequency::Daily { interval: 1, by_time: vec![] };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().day(), now.day() + 1);
    }

    #[test]
    fn collect_events_that_span_to_another_month() {
        let f = Frequency::Daily { interval: 15, by_time: vec![] };
        let date = DateTime::<Utc>::from_str("2020-01-02T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().day(), 17);

        let next_event = f.next_event(&next_event.unwrap());
        assert_eq!(next_event.unwrap().day(), 1);
        assert_eq!(next_event.unwrap().month(), 2);
    }
}

#[cfg(test)]
mod daily_frequencies_by_hour {
    use std::str::FromStr;
    use chrono::{Datelike, Duration, Timelike};
    use super::*;

    #[test]
    fn twice_a_day() {
        let f = Frequency::Daily { interval: 1, by_time: vec![
            Time::from_str("00:00").unwrap(),
            Time::from_str("12:00").unwrap()
        ] };
        let now = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&now).unwrap();
        assert_eq!(next_event.day(), now.day());
        assert_eq!(next_event.hour(), 12);
        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), now.day() + 1);
        assert_eq!(next_event.hour(), 0);
    }

    #[test]
    fn twice_a_day_with_interval() {
        let f = Frequency::Daily { interval: 2, by_time: vec![
            Time::from_str("00:00").unwrap(),
            Time::from_str("12:00").unwrap()
        ] };
        let now = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&now).unwrap();
        assert_eq!(next_event.day(), now.day());
        assert_eq!(next_event.hour(), 12);
        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), now.day() + 2);
        assert_eq!(next_event.hour(), 0);
    }
}
