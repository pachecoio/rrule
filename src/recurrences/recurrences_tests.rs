use std::str::FromStr;
use chrono::{DateTime, Duration, Utc, Weekday};
use crate::frequencies::{Frequency, Time};
use crate::recurrences::recurrences::{Recurrence, RecurrenceInvalid};

#[cfg(test)]
mod tests {
    use super::*;

    fn every_second_recurrence(start: DateTime<Utc>, end: Option<DateTime<Utc>>, duration: Option<Duration>) -> Result<Recurrence, RecurrenceInvalid> {
        let duration = duration.unwrap_or_else(|| Duration::seconds(1));
        Recurrence::new(
            Frequency::Secondly {
                interval: 1,
            },
            start,
            end,
            Some(duration)
        )
    }

    #[test]
    fn test_new_secondly_recurrence() {
        let recurrence = every_second_recurrence(
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            Some(DateTime::<Utc>::from_str("2023-01-01T00:00:02Z").unwrap()),
            None
        );
        assert!(recurrence.is_ok());
    }

    #[test]
    fn invalid_period() {
        let recurrence = every_second_recurrence(
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            Some(DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap()),
            None
        );
        assert!(recurrence.is_err());
    }

    #[test]
    fn invalid_duration() {
        let recurrence = every_second_recurrence(
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            Some(DateTime::<Utc>::from_str("2023-01-01T00:00:10Z").unwrap()),
            Some(Duration::hours(1))
        );
        assert!(recurrence.is_err());
    }
}

#[cfg(test)]
mod secondly_recurrences {
    use super::*;

    #[test]
    fn every_second() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-01-01T00:00:02Z").unwrap();
        let frequency = Frequency::Secondly {
            interval: 1,
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::seconds(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-01T00:00:01Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-01T00:00:02Z").unwrap(),
        ]);
    }
}

#[cfg(test)]
mod minutely_recurrences {
    use super::*;

    #[test]
    fn every_minute() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-01-01T00:02:00Z").unwrap();
        let frequency = Frequency::Minutely {
            interval: 1,
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::minutes(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-01T00:01:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-01T00:02:00Z").unwrap(),
        ]);
    }
}

#[cfg(test)]
mod hourly_recurrences {
    use super::*;

    #[test]
    fn every_hour() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-01-01T02:00:00Z").unwrap();
        let frequency = Frequency::Hourly {
            interval: 1,
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::hours(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-01T01:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-01T02:00:00Z").unwrap(),
        ]);
    }
}

#[cfg(test)]
mod daily_recurrences {
    use super::*;

    #[test]
    fn every_day() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-01-03T00:00:00Z").unwrap();
        let frequency = Frequency::Daily {
            interval: 1,
            by_time: vec![],
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::days(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-03T00:00:00Z").unwrap(),
        ]);
    }

    #[test]
    fn every_day_twice_a_day() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-01-03T00:00:00Z").unwrap();
        let frequency = Frequency::Daily {
            interval: 1,
            by_time: vec![
                Time::from_str("09:00:00").unwrap(),
                Time::from_str("18:00:00").unwrap(),
            ],
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::days(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 4);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-01T09:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-01T18:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-02T09:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-02T18:00:00Z").unwrap(),
        ]);
    }
}

#[cfg(test)]
mod weekly_recurrences {
    use super::*;

    #[test]
    fn weekly_recurrence() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-01-15T00:00:00Z").unwrap();
        let frequency = Frequency::Weekly {
            interval: 1,
            by_day: vec![],
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::weeks(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-08T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-15T00:00:00Z").unwrap(),
        ]);
    }

    #[test]
    fn weekly_by_day_recurrence() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-01-15T00:00:00Z").unwrap();
        let frequency = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Wed, Weekday::Fri],
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::weeks(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 6);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-04T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-6T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-11T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-13T00:00:00Z").unwrap(),
        ]);
    }
}

#[cfg(test)]
mod monthly_recurrences {
    use super::*;

    #[test]
    fn monthly_recurrence() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-03-01T00:00:00Z").unwrap();
        let frequency = Frequency::Monthly {
            interval: 1,
            by_day: vec![],
            by_month_day: vec![],
            by_week_number: vec![],
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::weeks(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-02-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-03-01T00:00:00Z").unwrap(),
        ]);
    }

    #[test]
    fn monthly_recurrence_by_month_day() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-02-20T00:00:00Z").unwrap();
        let frequency = Frequency::Monthly {
            interval: 1,
            by_day: vec![],
            by_month_day: vec![1, 15],
            by_week_number: vec![],
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::weeks(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 4);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-15T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-02-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-02-15T00:00:00Z").unwrap(),
        ]);
    }

    #[test]
    fn monthly_recurrence_by_week_number() {
        let start = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let end = DateTime::<Utc>::from_str("2023-02-20T00:00:00Z").unwrap();
        let frequency = Frequency::Monthly {
            interval: 1,
            by_day: vec![Weekday::Wed, Weekday::Fri],
            by_month_day: vec![],
            by_week_number: vec![1],
        };
        let recurrence = Recurrence::new(frequency, start, Some(end), Some(
            Duration::weeks(1)
        )).unwrap();
        let dates: Vec<DateTime<Utc>> = recurrence.collect();
        assert_eq!(dates.len(), 4);
        assert_eq!(dates, vec![
            DateTime::<Utc>::from_str("2023-01-04T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-01-06T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-02-01T00:00:00Z").unwrap(),
            DateTime::<Utc>::from_str("2023-02-03T00:00:00Z").unwrap(),
        ]);
    }
}