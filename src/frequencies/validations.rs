use chrono::Weekday;
use crate::frequencies::errors::FrequencyErrors;
use crate::frequencies::{MonthlyDate, Time};

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

pub fn validate_daily(interval: &i32, _by_time: &[Time]) -> Result<(), FrequencyErrors> {
    if *interval <= 0 {
        return Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        });
    }
    // Todo: Validate time
    Ok(())
}

pub fn validate_weekly(interval: &i32, _by_day: &[Weekday]) -> Result<(), FrequencyErrors> {
    if *interval <= 0 {
        return Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        });
    }
    // Todo: Validate weekday
    Ok(())
}

pub fn validate_monthly(interval: &i32, _by_month_day: &[i32]) -> Result<(), FrequencyErrors> {
    if *interval <= 0 {
        return Err(FrequencyErrors::InvalidInterval {
            message: "Interval must be greater than 0".to_string(),
        });
    }
    // Todo: Validate day of the month
    Ok(())
}

pub fn validate_yearly(_interval: &i32, _by_monthly_date: &[MonthlyDate]) -> Result<(), FrequencyErrors> {
    Ok(())
}
