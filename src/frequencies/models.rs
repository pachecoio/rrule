use std::cmp::Ordering;

use std::ops::{Add, Sub};
use chrono::{Datelike, DateTime, Duration, Month, Timelike, Utc, Weekday};
use crate::frequencies::errors::FrequencyErrors;
use crate::frequencies::validations::{validate_daily, validate_hourly, validate_minutely, validate_monthly, validate_secondly, validate_weekly, validate_yearly};
use crate::utils::{DateUtils, get_next_nth_weekday, weekday_ordinal};

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
        nth_weekdays: Vec<NthWeekday>,
    },
    Yearly {
        interval: i32,
        by_monthly_date: Vec<MonthlyDate>,
    }
}

/// Representation of the nth day of the week
/// E.g. 2nd Monday, 3rd Tuesday, etc.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NthWeekday {
    pub week_number: i32,
    pub weekday: Weekday,
}

impl NthWeekday {
    pub(crate) fn new(weekday: Weekday, week_number: i32) -> NthWeekday {
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
            self.weekday.num_days_from_sunday().cmp(&other.weekday.num_days_from_sunday())
        } else {
            week_number_cmp
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Time {
    pub hour: i32,
    pub minute: i32,
}

impl Time {
    pub(crate) fn from_str(time_str: &str) -> Result<Self, FrequencyErrors> {
        let mut parts = time_str.split(':');
        let hour = match parts.next() {
            None => return Err(FrequencyErrors::InvalidTime {
                message: format!("Invalid time: {time_str}"),
            }),
            Some(hour) => hour.parse::<i32>().unwrap()
        };
        let minute = match parts.next() {
            None => return Err(FrequencyErrors::InvalidTime {
                message: format!("Invalid time: {time_str}")
            }),
            Some(minute) => minute.parse::<i32>().unwrap()
        };
        Ok(Time {
            hour,
            minute,
        })
    }
}

/// Representation of a monthly date (e.g. 1st of the month of January)
pub struct MonthlyDate {
    pub month: Month,
    pub day: i32,
}

impl Frequency {
    pub(crate) fn is_valid(&self) -> Result<(), FrequencyErrors> {
        match self {
            Frequency::Secondly { interval } => validate_secondly(interval),
            Frequency::Minutely { interval } => validate_minutely(interval),
            Frequency::Hourly { interval } => validate_hourly(interval),
            Frequency::Daily { interval, by_time } => validate_daily(interval, by_time),
            Frequency::Weekly { interval, by_day } => validate_weekly(interval, by_day),
            Frequency::Monthly { interval, by_month_day: _, nth_weekdays: _ } => validate_monthly(interval, &[]),
            Frequency::Yearly { interval, by_monthly_date} => validate_yearly(
                interval, by_monthly_date
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
                current_date, *interval, by_time
            ),
            Frequency::Weekly { interval, by_day } => next_weekly_event(
                current_date, *interval, by_day
            ),
            Frequency::Monthly { interval, by_month_day, nth_weekdays } => _next_monthly_event(
                current_date, *interval, by_month_day, nth_weekdays
            ),
            Frequency::Yearly { interval, by_monthly_date } => next_yearly_event(
                current_date, *interval, by_monthly_date
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
            Frequency::Monthly { nth_weekdays, by_month_day, .. } => {
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
            },
            Frequency::Yearly { by_monthly_date, .. } => {
                if by_monthly_date.is_empty() {
                    return true;
                }
                let month = date.month();
                let day = date.day() as i32;
                for monthly_date in by_monthly_date {
                    if monthly_date.month.number_from_month() == month && monthly_date.day == day {
                        return true;
                    }
                }
                false
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

fn _next_monthly_event(current_date: &DateTime<Utc>, interval: i32, by_month_day: &Vec<i32>, nth_weekdays: &Vec<NthWeekday>) -> Option<DateTime<Utc>> {
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
        return get_next_nth_weekday(
            current_date,
            interval as i64,
            nth_weekdays,
        )
    }
    next_date
}

fn next_yearly_event(current_date: &DateTime<Utc>, interval: i32, by_monthly_date: &Vec<MonthlyDate>) -> Option<DateTime<Utc>> {
    if !by_monthly_date.is_empty() {
        for date in by_monthly_date {
            let month_number = date.month.number_from_month();
            let d = current_date
                .with_month(month_number)
                .unwrap()
                .with_day(date.day as u32);
            match d {
                Some(d) => {
                    if d > *current_date {
                        return Some(d);
                    }
                }
                None => return None
            }
        }

        // No dates left in the year, so we need to add a year
        let month_number = by_monthly_date[0].month.number_from_month();
        let result = current_date
            .with_month(month_number)
            .unwrap()
            .with_day(by_monthly_date[0].day as u32);
        return match result {
            Some(d) => d.shift_years(interval as i64),
            None => None
        }
    }

    current_date.shift_years(interval as i64)
}
