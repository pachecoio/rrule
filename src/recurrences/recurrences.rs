use std::fmt::{Display, Formatter};
use std::str::FromStr;
use chrono::{Datelike, DateTime, Duration, Timelike, Utc, Weekday};
use crate::frequencies::{Frequency, NthWeekday, Time};
use crate::utils::{DateUtils, get_next_nth_weekday};

const MAX_DATE: &str = "9999-12-31T23:59:59Z";

pub struct Recurrence {
    frequency: Frequency,

    /// Start date of the recurrences
    ///
    /// This won't necessarily be the first event date, as that depends on the frequencies
    /// configuration defined.
    start: DateTime<Utc>,

    /// Current event date to be returned by next()
    /// Starts as None, and is set to the first event date once next() is called
    current_date: Option<DateTime<Utc>>,

    /// End date of the recurrences
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
        Frequency::Minutely { interval } => {
            let minutes = duration.num_minutes();
            if minutes > *interval as i64 {
                return Err(RecurrenceInvalid {
                    message: "Duration must be smaller than interval".to_string()
                });
            }
        }
        Frequency::Hourly { interval } => {
            let hours = duration.num_hours();
            if hours > *interval as i64 {
                return Err(RecurrenceInvalid {
                    message: "Duration must be smaller than interval".to_string()
                });
            }
        }
        Frequency::Daily { interval, by_time } => {
            let days = duration.num_days();

            if days > *interval as i64 {
                return Err(RecurrenceInvalid {
                    message: "Duration must be smaller than interval".to_string()
                });
            }

            if by_time.len() > 1 {
                let mut t = &by_time[0];
                for time in 1..by_time.len() {
                    validate_time_duration(
                        &t,
                    &by_time[time],
                        duration
                    )?;
                    t = &by_time[time];
                }

                // Compare last time of day with first time next day
                validate_time_duration(
                    &by_time[by_time.len() - 1],
                    &by_time[0],
                    duration
                )?;
            }
        }
        Frequency::Weekly { interval, by_day } => {
            let weeks = duration.num_weeks();
            if weeks > *interval as i64 {
                return Err(RecurrenceInvalid {
                    message: "Duration must be smaller than interval".to_string()
                });
            }

            if by_day.len() > 1 {
                let mut w = &by_day[0];
                for index in 1..by_day.len() {
                    validate_weekday_duration(
                        &w,
                    &by_day[index],
                        duration
                    )?;
                    w = &by_day[index];
                }

                // Compare last time of day with first time next day
                validate_weekday_duration(
                    &by_day[by_day.len() - 1],
                    &by_day[0],
                    duration
                )?;
            }

        }
        Frequency::Monthly { interval, by_month_day,  nth_weekdays } => {
            let months = duration.num_days() as f32 / 30.0;
            if months > *interval as f32 {
                return Err(RecurrenceInvalid {
                    message: format!(
                        "Total duration cannot be bigger than {} days", 30 * interval
                    )
                });
            }
            if !by_month_day.is_empty() {
                let mut m = &by_month_day[0];
                for index in 1..by_month_day.len() {
                    validate_monthly_by_month_day_duration(
                        &m,
                        &by_month_day[index],
                        duration
                    )?;
                    m = &by_month_day[index];
                }

                // Compare last time of day with first time next day
                validate_monthly_by_month_day_duration(
                    &by_month_day[by_month_day.len() - 1],
                    &by_month_day[0],
                    duration
                )?;
            }

            if !nth_weekdays.is_empty() {
                let mut m = &nth_weekdays[0];
                for index in 1..nth_weekdays.len() {
                    validate_monthly_nth_weekday_duration(
                        &m,
                        &nth_weekdays[index],
                        duration
                    )?;
                    m = &nth_weekdays[index];
                }

                // Compare last time of day with first time next day
                validate_monthly_nth_weekday_duration(
                    &nth_weekdays[nth_weekdays.len() - 1],
                    &nth_weekdays[0],
                    duration
                )?;
            }

        }
        Frequency::Yearly { .. } => {}
    }
    Ok(())
}

fn validate_time_duration(time: &Time, next_time: &Time, duration: &Duration) -> Result<(), RecurrenceInvalid> {
    let date = Utc::now()
        .with_hour(time.hour as u32)
        .unwrap()
        .with_minute(time.minute as u32)
        .unwrap();
    let projected_date = date + *duration;
    let mut next_date = Utc::now()
        .with_hour(next_time.hour as u32)
        .unwrap()
        .with_minute(next_time.minute as u32)
        .unwrap();
    if next_date < date {
        next_date = next_date.shift_days(1).unwrap();
    }
    if projected_date > next_date {
        return Err(RecurrenceInvalid {
            message: "There is an overlap of events with the current times and duration defined.".to_string()
        });
    }
    Ok(())
}

fn validate_weekday_duration(weekday: &Weekday, next_weekday: &Weekday, duration: &Duration) -> Result<(), RecurrenceInvalid> {
    let now = Utc::now();

    let date = now.with_weekday(*weekday).unwrap();

    let projected_date = date + *duration;
    let mut next_date = now.with_weekday(*next_weekday).unwrap();
    if next_date < date {
        next_date = next_date.shift_weeks(1).unwrap();
    }
    if projected_date > next_date {
        return Err(RecurrenceInvalid {
            message: "There is an overlap of events with the current times and duration defined.".to_string()
        });
    }
    Ok(())
}

fn validate_monthly_by_month_day_duration(monthday: &i32, next_monthday: &i32, duration: &Duration) -> Result<(), RecurrenceInvalid> {
    let now = Utc::now();

    let date = now.with_day(*monthday as u32).unwrap();

    let projected_date = date + *duration;
    let mut next_date = now.with_day(*next_monthday as u32).unwrap();
    if next_date < date {
        next_date = next_date.shift_months(1).unwrap();
    }
    if projected_date > next_date {
        return Err(RecurrenceInvalid {
            message: "There is an overlap of events with the current times and duration defined.".to_string()
        });
    }
    Ok(())
}

fn validate_monthly_nth_weekday_duration(nth_weekday: &NthWeekday, next_monthday: &NthWeekday, duration: &Duration) -> Result<(), RecurrenceInvalid> {
    // Use last day of previous month as a reference to get the next nth weekday of current month
    let last_day_prev_month = Utc::now()
        .with_day(1)
        .unwrap()
        .shift_days(-1)
        .unwrap();

    let date = get_next_nth_weekday(
        &last_day_prev_month,
        1,
        &vec![nth_weekday.clone()]
    ).unwrap();

    let projected_date = date + *duration;

    let mut next_date = get_next_nth_weekday(
        &date,
        1,
        &vec![next_monthday.clone()]
    ).unwrap();
    if next_date < projected_date {
        return Err(RecurrenceInvalid {
            message: "There is an overlap of events with the current weekdays and duration defined.".to_string()
        });
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
                if self.frequency.contains(&self.start) {
                    self.current_date = Some(self.start);
                    return Some(self.start);
                }
                self.start
            },
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
            None => None
        }
    }
}

