use std::collections::HashSet;
use chrono::Weekday;
use crate::frequencies::errors::FrequencyErrors;
use crate::frequencies::{MonthlyDate, NthWeekday, Time};

pub fn validate_secondly(interval: &i32) -> Result<(), FrequencyErrors> {
    if *interval > 0 {
        Ok(())
    } else {
        Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        })
    }
}

pub fn validate_minutely(interval: &i32) -> Result<(), FrequencyErrors> {
    if *interval > 0 {
        Ok(())
    } else {
        Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        })
    }
}

pub fn validate_hourly(interval: &i32) -> Result<(), FrequencyErrors> {
    if *interval > 0 {
        Ok(())
    } else {
        Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        })
    }
}

pub fn validate_daily(interval: &i32, by_time: &[Time]) -> Result<(), FrequencyErrors> {
    if *interval <= 0 {
        return Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        });
    }
    let mut unique_times: HashSet<Time> = HashSet::new();
    for time in by_time {
        let t = Time {
            hour: time.hour,
            minute: time.minute,
        };
        if !unique_times.insert(t) {
            return Err(FrequencyErrors::InvalidTime {
                message: "Repeated time".to_string(),
            });
        }
    }
    Ok(())
}

pub fn validate_weekly(interval: &i32, by_day: &[Weekday]) -> Result<(), FrequencyErrors> {
    if *interval <= 0 {
        return Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        });
    }
    let mut unique_days: HashSet<Weekday> = HashSet::new();
    for day in by_day {
        if !unique_days.insert(*day) {
            return Err(FrequencyErrors::InvalidDay {
                message: "Repeated day".to_string(),
            });
        }
    }
    Ok(())
}

pub fn validate_monthly(interval: &i32, by_month_day: &[i32], nth_weekdays: &[NthWeekday]) -> Result<(), FrequencyErrors> {
    if *interval <= 0 {
        return Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        });
    }
    let mut unique_month_days: HashSet<i32> = HashSet::new();
    for day in by_month_day {
        if !unique_month_days.insert(*day) {
            return Err(FrequencyErrors::InvalidDay {
                message: "Repeated day".to_string(),
            });
        }
    }

    let mut unique_nth_weekdays: HashSet<NthWeekday> = HashSet::new();
    for nth_weekday in nth_weekdays {
        let nth_weekday = NthWeekday {
            week_number: nth_weekday.week_number,
            weekday: nth_weekday.weekday,
        };
        if !unique_nth_weekdays.insert(nth_weekday) {
            return Err(FrequencyErrors::InvalidDay {
                message: "Repeated day".to_string(),
            });
        }
    }

    Ok(())
}

pub fn validate_yearly(_interval: &i32, _by_monthly_date: &[MonthlyDate]) -> Result<(), FrequencyErrors> {
    Ok(())
}
