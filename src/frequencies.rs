use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use chrono::{Datelike, DateTime, Duration, Timelike, Utc, Weekday};
use crate::utils::{DateUtils, get_next_nth_weekday_in_range, weekday_ordinal};

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
    Weekly {
        interval: i32,
        by_day: Vec<Weekday>,
    },
    Monthly {
        interval: i32,
        by_month_day: Vec<i32>,
        by_day: Vec<Weekday>,
        by_week_number: Vec<i32>,
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
            Frequency::Weekly { interval, by_day } => validate_weekly(interval, by_day),
            Frequency::Monthly { interval, by_month_day, .. } => validate_monthly(interval, by_month_day),
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
            Frequency::Weekly { interval, by_day } => next_weekly_event(
                current_date, *interval, &by_day
            ),
            Frequency::Monthly { interval, by_month_day, by_day, by_week_number } => next_monthly_event(
                current_date, *interval, &by_month_day, &by_day, &by_week_number
            ),
        }
    }

    pub(crate) fn contains(&self, date: &DateTime<Utc>) -> bool {
        match self {
            Frequency::Secondly { .. } => true,
            Frequency::Minutely { .. } => true,
            Frequency::Hourly { .. } => true,
            Frequency::Daily { by_time, .. } => {
                if by_time.is_empty() {
                    return true;
                }
                // Return 1 minute from current date to confirm if the current date could be
                // the next event date.
                let start = date.sub(Duration::minutes(1));
                match self.next_event(&start) {
                    None => false,
                    Some(next_date) => next_date == *date
                }
            }
            Frequency::Weekly { by_day, .. } => {
                by_day.is_empty() || by_day.contains(&date.weekday())
            },
            Frequency::Monthly { by_month_day, .. } => {
                let day = date.day() as i32;
                by_month_day.is_empty() || by_month_day.contains(&day)
            },
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

fn next_weekly_event(current_date: &DateTime<Utc>, interval: i32, by_day: &Vec<Weekday>) -> Option<DateTime<Utc>> {
    let mut next_date = current_date.add(chrono::Duration::weeks(interval as i64));

    if !by_day.is_empty() {
        let current_weekday_num = current_date.weekday().num_days_from_sunday() + 1;
        let d = current_date.format("%Y-%m-%d").to_string();
        for day in by_day {
            let day_num = day.num_days_from_sunday() + 1;
            if day_num > current_weekday_num {
                let diff = day_num - current_weekday_num;
                return Some(current_date.add(chrono::Duration::days(diff as i64)));
            }
        }
        // No days left in the week, so we need to add a week
        if let Some(d) = current_date.with_weekday(by_day[0]) {
            return d.shift_weeks(interval as i64);
        }
    }
    Some(next_date)
}

fn next_monthly_event(current_date: &DateTime<Utc>, interval: i32, by_month_day: &Vec<i32>, by_day: &Vec<Weekday>, by_week_number: &Vec<i32>) -> Option<DateTime<Utc>> {
    let mut next_date = current_date.shift_months(interval as i64);
    if !by_month_day.is_empty() {
        let current_month_day = current_date.day() as i32;
        for day in by_month_day {
            if *day > current_month_day {
                if let Some(d) = current_date.with_day(*day as u32) {
                    return Some(d);
                }
            }
        }
        // No days left in the month, so we need to add a month
        if let Some(d) = current_date.with_day(by_month_day[0] as u32) {
            return d.shift_months(interval as i64);
        }
    }
    if !by_day.is_empty() || !by_week_number.is_empty() {
        return get_next_nth_weekday_in_range(current_date, by_day, by_week_number)
    }
    next_date
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
    Ok(())
}

fn validate_weekly(interval: &i32, by_day: &Vec<Weekday>) -> Result<(), FrequencyErrors> {
    if *interval <= 0 {
        return Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        });
    }
    // Todo: Validate weekday
    Ok(())
}

fn validate_monthly(interval: &i32, by_month_day: &Vec<i32>) -> Result<(), FrequencyErrors> {
    if *interval <= 0 {
        return Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
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

#[cfg(test)]
mod weekly_frequency {
    use std::str::FromStr;
    use chrono::{Datelike, Duration, Timelike};
    use super::*;

    #[test]
    fn every_week_frequency() {
        let f = Frequency::Weekly {
            interval: 1,
            by_day: vec![],
        };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Weekly { interval: 0, by_day: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Weekly { interval: -1, by_day: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_week_collect_events() {
        let f = Frequency::Weekly { interval: 1, by_day: vec![] };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().day(), now.day() + 7);
    }

    #[test]
    fn collect_events_that_span_to_another_month() {
        let f = Frequency::Weekly { interval: 1, by_day: vec![] };
        let date = DateTime::<Utc>::from_str("2020-01-28T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().day(), 4);
        assert_eq!(next_event.unwrap().month(), 2);
    }
}

#[cfg(test)]
mod weekly_by_day {
    use std::str::FromStr;
    use chrono::Datelike;
    use super::*;

    #[test]
    fn every_monday() {
        let f = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon],
        };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.weekday(), Weekday::Mon);
        assert_eq!(next_event.day(), 6);

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.weekday(), Weekday::Mon);
        assert_eq!(next_event.day(), 13);
    }

    #[test]
    fn twice_a_week() {
        let f = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Wed],
        };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.weekday(), Weekday::Mon);
        assert_eq!(next_event.day(), 2);
    }
}

#[cfg(test)]
mod monthly_frequency {
    use std::str::FromStr;
    use chrono::{Datelike, Duration, Timelike};
    use super::*;

    #[test]
    fn every_month_frequency() {
        let f = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            by_day: vec![],
            by_week_number: vec![],
        };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Monthly { interval: 0, by_month_day: vec![], by_day: vec![], by_week_number: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Monthly { interval: -1, by_month_day: vec![], by_day: vec![], by_week_number: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_month_collect_events() {
        let f = Frequency::Monthly { interval: 1, by_month_day: vec![], by_day: vec![], by_week_number: vec![] };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().month(), now.month() + 1);
    }

    #[test]
    fn collect_events_that_span_to_another_year() {
        let f = Frequency::Monthly { interval: 1, by_month_day: vec![], by_day: vec![], by_week_number: vec![] };
        let date = DateTime::<Utc>::from_str("2020-12-02T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().month(), 1);
        assert_eq!(next_event.unwrap().year(), 2021);
    }
}

#[cfg(test)]
mod monthly_by_month_day {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn every_1st_of_month() {
        let f = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![1],
            by_day: vec![],
            by_week_number: vec![],
        };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 1, "next event should be the 1st of the month");
        assert_eq!(next_event.month(), 2, "next event should be in the next month");

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), 1);
        assert_eq!(next_event.month(), 3);
    }

    #[test]
    fn every_1st_and_15th_of_the_month() {
        let f = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![1, 15],
            by_day: vec![],
            by_week_number: vec![],
        };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 15, "next event should be the 15th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), 1, "next event should be the 1st of the month");
        assert_eq!(next_event.month(), 2, "next event should be in the next month");
    }

    #[test]
    fn every_31th() {
        let f = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![31],
            by_day: vec![],
            by_week_number: vec![],
        };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 31, "next event should be the 31th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");

        let next_event = f.next_event(&next_event);
        assert!(next_event.is_none(), "next event should be none because february does not have a 31th day");
    }
}

#[cfg(test)]
mod monthly_by_weekday {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn every_1st_monday_of_the_month() {
        let f = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            by_day: vec![Weekday::Mon],
            by_week_number: vec![1],
        };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 2, "next event should be the 2th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");
    }

    #[test]
    fn every_2nd_tuesday() {
        let f = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            by_day: vec![Weekday::Tue],
            by_week_number: vec![2],
        };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 10, "next event should be the 10th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");
    }

    #[test]
    fn every_1st_wednesday_and_friday() {
        let f = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            by_day: vec![Weekday::Wed, Weekday::Fri],
            by_week_number: vec![1],
        };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 4, "next event should be the 4th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), 6, "next event should be the 6th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), 1, "next event should be the 1st of the month");
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_date_within_frequency() {
        let f = Frequency::Daily { interval: 1, by_time: vec![] };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let result = f.contains(&date);
        assert!(result);
    }

    #[test]
    fn not_within_frequency() {
        let f = Frequency::Daily { interval: 1, by_time: vec![
            Time::from_str("12:00:00").unwrap(),
        ] };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let result = f.contains(&date);
        assert!(!result);
    }

    #[test]
    fn within_weekly_frequency() {
        let f = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Wed],
        };
        let date_within_frequency = DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap();
        let result = f.contains(&date_within_frequency);
        assert!(result);

        let date_not_within_frequency = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let result = f.contains(&date_not_within_frequency);
        assert!(!result);
    }

    #[test]
    fn within_monthly_frequency() {
        let f = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            by_day: vec![],
            by_week_number: vec![],
        };
        let date_within_frequency = DateTime::<Utc>::from_str("2023-01-15T00:00:00Z").unwrap();
        let result = f.contains(&date_within_frequency);
        assert!(result);
    }

    #[test]
    fn within_monthly_by_month_day() {
        let f = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![15],
            by_day: vec![],
            by_week_number: vec![],
        };
        let date_within_frequency = DateTime::<Utc>::from_str("2023-01-15T00:00:00Z").unwrap();
        let result = f.contains(&date_within_frequency);
        assert!(result);

        let date_not_within_frequency = DateTime::<Utc>::from_str("2023-01-16T00:00:00Z").unwrap();
        let result = f.contains(&date_not_within_frequency);
        assert!(!result);
    }
}
