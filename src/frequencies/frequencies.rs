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
    Yearly {
        interval: i32,
        dates: Vec<DateTime<Utc>>,
    }
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
    pub(crate) fn is_valid(&self) -> Result<(), FrequencyErrors> {
        match self {
            Frequency::Secondly { interval } => validate_secondly(interval),
            Frequency::Minutely { interval } => validate_minutely(interval),
            Frequency::Hourly { interval } => validate_hourly(interval),
            Frequency::Daily { interval, by_time } => validate_daily(interval, by_time),
            Frequency::Weekly { interval, by_day } => validate_weekly(interval, by_day),
            Frequency::Monthly { interval, by_month_day, .. } => validate_monthly(interval, by_month_day),
            Frequency::Yearly { interval, dates, .. } => validate_yearly(
                interval, dates
            ),
        }
    }

    /// Returns the next event date for the current frequencies config given the current date.
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
            Frequency::Yearly { interval, dates} => next_yearly_event(
                current_date, *interval, dates
            )
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
            Frequency::Monthly { by_month_day, by_day, by_week_number, .. } => {
                let day = date.day() as i32;

                if !by_month_day.is_empty() {
                    return by_month_day.contains(&day);
                }

                if !by_day.is_empty() {
                    let weekday = date.weekday();
                    let week_number = weekday_ordinal(date);
                    return by_day.contains(&weekday) && by_week_number.contains(&week_number);
                }

                true
            },
            Frequency::Yearly { dates, .. } => {
                true
            }
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

fn next_yearly_event(current_date: &DateTime<Utc>, interval: i32, dates: &Vec<DateTime<Utc>>) -> Option<DateTime<Utc>> {
    let next_date = current_date.shift_years(interval as i64);
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
    // Todo: Validate time
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
    // Todo: Validate day of the month
    Ok(())
}

fn validate_yearly(interval: &i32, dates: &Vec<DateTime<Utc>>) -> Result<(), FrequencyErrors> {
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
