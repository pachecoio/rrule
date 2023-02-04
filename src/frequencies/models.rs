use std::cmp::Ordering;

use crate::frequencies::errors::InvalidFrequency;
use crate::frequencies::validations::{
    validate_daily, validate_hourly, validate_minutely, validate_monthly, validate_secondly,
    validate_weekly, validate_yearly,
};
use crate::utils::{get_next_nth_weekday, weekday_ordinal, DateUtils};
use chrono::{DateTime, Datelike, Duration, Month, Timelike, Utc, Weekday};
use std::ops::{Add, Sub};
use std::str::FromStr;

/// Representation of the frequency of a recurrence.
/// E.g. Once a day, Twice a week, etc.
///
/// Examples:
/// ```
/// use rrules::{Frequency};
///
/// let once_a_day = Frequency::Daily {interval: 1, by_time: vec![]};
/// assert_eq!(once_a_day.to_string(), "FREQ=DAILY;INTERVAL=1");
///
/// let three_times_a_month = Frequency::Monthly {
///     interval: 1,
///     by_month_day: vec![1, 10, 20],
///     nth_weekdays: vec![]
/// };
/// assert_eq!(three_times_a_month.to_string(), "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=1,10,20");
/// ```
#[derive(Debug)]
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
        by_time: Vec<Time>,
    },
    Weekly {
        interval: i32,
        by_day: Vec<Weekday>,
    },
    Monthly {
        interval: i32,
        by_month_day: Vec<i32>,
        nth_weekdays: Vec<NthWeekday>,
    },
    Yearly {
        interval: i32,
        by_monthly_date: Option<MonthlyDate>,
    },
}

/// Representation of the nth day of the week
/// E.g. 2nd Monday, 3rd Tuesday, etc.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct NthWeekday {
    pub week_number: i32,
    pub weekday: Weekday,
}

impl NthWeekday {
    pub fn new(weekday: Weekday, week_number: i32) -> NthWeekday {
        NthWeekday {
            week_number,
            weekday,
        }
    }
}

impl PartialOrd<Self> for NthWeekday {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NthWeekday {
    fn cmp(&self, other: &Self) -> Ordering {
        let week_number_cmp = self.week_number.cmp(&other.week_number);
        if week_number_cmp == Ordering::Equal {
            self.weekday
                .num_days_from_sunday()
                .cmp(&other.weekday.num_days_from_sunday())
        } else {
            week_number_cmp
        }
    }
}

/// Representation of a time containing hour:minute
/// E.g. 12:00, 23:59, etc.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Time {
    pub hour: i32,
    pub minute: i32,
}

impl FromStr for Time {
    type Err = InvalidFrequency;

    fn from_str(time_str: &str) -> Result<Self, InvalidFrequency> {
        let mut parts = time_str.split(':');
        let hour = match parts.next() {
            None => {
                return Err(InvalidFrequency::Time {
                    message: format!("Invalid time: {time_str}"),
                })
            }
            Some(hour) => match hour.parse::<i32>() {
                Ok(hour) => hour,
                Err(_) => {
                    return Err(InvalidFrequency::Time {
                        message: format!("Invalid time: {time_str}"),
                    })
                }
            },
        };
        let minute = match parts.next() {
            None => {
                return Err(InvalidFrequency::Time {
                    message: format!("Invalid time: {time_str}"),
                })
            }
            Some(minute) => match minute.parse::<i32>() {
                Ok(minute) => minute,
                Err(_) => {
                    return Err(InvalidFrequency::Time {
                        message: format!("Invalid time: {time_str}"),
                    })
                }
            },
        };
        Ok(Time { hour, minute })
    }
}

/// Representation of a monthly date
/// E.g. 1st of January, 2nd of February, etc.
#[derive(Debug)]
pub struct MonthlyDate {
    pub month: Month,
    pub day: i32,
}

impl Frequency {
    /// Verifies if the frequency is valid.
    pub fn is_valid(&self) -> Result<(), InvalidFrequency> {
        match self {
            Frequency::Secondly { interval } => validate_secondly(interval),
            Frequency::Minutely { interval } => validate_minutely(interval),
            Frequency::Hourly { interval } => validate_hourly(interval),
            Frequency::Daily { interval, by_time } => validate_daily(interval, by_time),
            Frequency::Weekly { interval, by_day } => validate_weekly(interval, by_day),
            Frequency::Monthly {
                interval,
                by_month_day,
                nth_weekdays,
            } => validate_monthly(interval, by_month_day, nth_weekdays),
            Frequency::Yearly {
                interval,
                by_monthly_date,
            } => validate_yearly(interval, by_monthly_date),
        }
    }

    /// Returns the next event date for the current frequencies config given the current date.
    /// Returns None if there is no next event.
    /// E.g. If the frequency is once a day and the current date is 2020-01-01, the next event date will be 2020-01-02.
    pub fn next_event(&self, current_date: &DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            Frequency::Secondly { interval } => {
                let next_date = current_date.add(chrono::Duration::seconds(*interval as i64));
                Some(next_date)
            }
            Frequency::Minutely { interval } => {
                let next_date = current_date.add(chrono::Duration::minutes(*interval as i64));
                Some(next_date)
            }
            Frequency::Hourly { interval } => {
                let next_date = current_date.add(chrono::Duration::hours(*interval as i64));
                Some(next_date)
            }
            Frequency::Daily { interval, by_time } => {
                next_daily_event(current_date, *interval, by_time)
            }
            Frequency::Weekly { interval, by_day } => {
                next_weekly_event(current_date, *interval, by_day)
            }
            Frequency::Monthly {
                interval,
                by_month_day,
                nth_weekdays,
            } => _next_monthly_event(current_date, *interval, by_month_day, nth_weekdays),
            Frequency::Yearly {
                interval,
                by_monthly_date,
            } => next_yearly_event(current_date, *interval, by_monthly_date),
        }
    }

    /// Verifies if the specified date is a valid event date for the current frequency.
    /// E.g. If the frequency is once a day and the date is 2023-01-01, the method will return true.
    /// If the frequency is Once a week on Monday and the date is 2023-01-01, the method will return false
    /// because the date is not a Monday.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use rrules::Frequency;
    /// use chrono::{Utc, DateTime, Duration, Weekday};
    ///
    /// let once_a_day = Frequency::Daily {interval: 1,by_time: vec![]};
    /// let sunday = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
    /// assert!(once_a_day.contains(&sunday));
    ///
    /// let every_monday = Frequency::Weekly {interval: 1, by_day: vec![Weekday::Mon]};
    /// assert!(!every_monday.contains(&sunday));
    /// ```
    pub fn contains(&self, date: &DateTime<Utc>) -> bool {
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
                    Some(next_date) => next_date == *date,
                }
            }
            Frequency::Weekly { by_day, .. } => {
                by_day.is_empty() || by_day.contains(&date.weekday())
            }
            Frequency::Monthly {
                nth_weekdays,
                by_month_day,
                ..
            } => {
                if by_month_day.is_empty() && nth_weekdays.is_empty() {
                    return true;
                }
                let day = date.day() as i32;

                if !by_month_day.is_empty() {
                    return by_month_day.contains(&day);
                }
                let weekday = date.weekday();
                let week_number = weekday_ordinal(date);
                for nth in nth_weekdays {
                    if nth.weekday == weekday && nth.week_number == week_number {
                        return true;
                    }
                }
                false
            }
            Frequency::Yearly {
                by_monthly_date, ..
            } => {
                if let Some(by_monthly_date) = by_monthly_date {
                    let month = date.month() as i32;
                    let day = date.day() as i32;
                    return by_monthly_date.month.number_from_month() == month as u32
                        && by_monthly_date.day == day;
                }
                true
            }
        }
    }
}

fn next_daily_event(
    current_date: &DateTime<Utc>,
    interval: i32,
    by_time: &Vec<Time>,
) -> Option<DateTime<Utc>> {
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
            .with_hour(by_time[0].hour as u32)
            .unwrap()
            .with_minute(by_time[0].minute as u32)
            .unwrap();
    }
    Some(next_date)
}

fn next_weekly_event(
    current_date: &DateTime<Utc>,
    interval: i32,
    by_day: &Vec<Weekday>,
) -> Option<DateTime<Utc>> {
    let next_date = current_date.add(chrono::Duration::weeks(interval as i64));

    if !by_day.is_empty() {
        let current_weekday_num = current_date.weekday().num_days_from_sunday() + 1;
        let _d = current_date.format("%Y-%m-%d").to_string();
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

fn _next_monthly_event(
    current_date: &DateTime<Utc>,
    interval: i32,
    by_month_day: &Vec<i32>,
    nth_weekdays: &Vec<NthWeekday>,
) -> Option<DateTime<Utc>> {
    let next_date = current_date.shift_months(interval as i64);
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
    if !nth_weekdays.is_empty() {
        return get_next_nth_weekday(current_date, interval as i64, nth_weekdays);
    }
    next_date
}

fn next_yearly_event(
    current_date: &DateTime<Utc>,
    interval: i32,
    by_monthly_date: &Option<MonthlyDate>,
) -> Option<DateTime<Utc>> {
    if let Some(by_monthly_date) = by_monthly_date {
        let month_number = by_monthly_date.month.number_from_month();
        let result = current_date
            .with_month(month_number)
            .unwrap()
            .with_day(by_monthly_date.day as u32)?;
        return if result > *current_date {
            Some(result)
        } else {
            result.shift_years(interval as i64)
        };
    }
    current_date.shift_years(interval as i64)
}
