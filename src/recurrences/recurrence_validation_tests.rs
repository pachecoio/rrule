




#[cfg(test)]
mod secondly_validations {
    use std::str::FromStr;
    use chrono::{DateTime, Duration, Utc};
    use crate::frequencies::Frequency;
    use crate::recurrences::Recurrence;
    use super::*;

    #[test]
    fn every_second() {
        let freq = Frequency::Secondly {
            interval: 1,
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::seconds(1)
            )
        );
        assert!(recurrence.is_ok());
    }

    #[test]
    fn every_second_with_invalid_duration() {
        let freq = Frequency::Secondly {
            interval: 1,
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::seconds(2)
            )
        );
        assert!(recurrence.is_err());
    }
}

#[cfg(test)]
mod minutely_validations {
    use std::str::FromStr;
    use chrono::{DateTime, Duration, Utc};
    use crate::frequencies::Frequency;
    use crate::recurrences::Recurrence;
    use super::*;

    #[test]
    fn every_minute() {
        let freq = Frequency::Minutely {
            interval: 1,
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::minutes(1)
            )
        );
        assert!(recurrence.is_ok());
    }

    #[test]
    fn every_minute_with_invalid_duration() {
        let freq = Frequency::Minutely {
            interval: 1,
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::minutes(2)
            )
        );
        assert!(recurrence.is_err());
    }
}

#[cfg(test)]
mod hourly_validations {
    use std::str::FromStr;
    use chrono::{DateTime, Duration, Utc};
    use crate::frequencies::Frequency;
    use crate::recurrences::Recurrence;
    use super::*;

    #[test]
    fn every_hour() {
        let freq = Frequency::Hourly {
            interval: 1,
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::hours(1)
            )
        );
        assert!(recurrence.is_ok());
    }

    #[test]
    fn every_hour_with_invalid_duration() {
        let freq = Frequency::Hourly {
            interval: 1,
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::hours(2)
            )
        );
        assert!(recurrence.is_err());
    }
}

#[cfg(test)]
mod daily_validations {
    use std::str::FromStr;
    use chrono::{DateTime, Duration, Utc};
    use crate::frequencies::{Frequency, Time};
    use crate::recurrences::Recurrence;
    use super::*;

    #[test]
    fn every_day() {
        let freq = Frequency::Daily {
            interval: 1,
            by_time: vec![],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::days(1)
            )
        );
        assert!(recurrence.is_ok());
    }

    #[test]
    fn every_day_with_invalid_duration() {
        let freq = Frequency::Daily {
            interval: 1,
            by_time: vec![],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::days(2)
            )
        );
        assert!(recurrence.is_err());
    }

    #[test]
    fn every_day_by_time_with_invalid_duration() {
        let freq = Frequency::Daily {
            interval: 1,
            by_time: vec![
                Time::from_str("12:00:00").unwrap(),
                Time::from_str("14:00:00").unwrap()
            ],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::hours(6)
            )
        );
        assert!(recurrence.is_err());
    }

    #[test]
    fn every_day_by_time_with_invalid_duration_over_day() {
        let freq = Frequency::Daily {
            interval: 1,
            by_time: vec![
                Time::from_str("02:00:00").unwrap(),
                Time::from_str("22:00:00").unwrap()
            ],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::hours(6)
            )
        );
        assert!(recurrence.is_err());
    }
}

#[cfg(test)]
mod weekly_validations {
    use std::str::FromStr;
    use chrono::{DateTime, Duration, Utc, Weekday};
    use crate::frequencies::Frequency;
    use crate::recurrences::Recurrence;
    use super::*;

    #[test]
    fn every_week() {
        let freq = Frequency::Weekly {
            interval: 1,
            by_day: vec![],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::weeks(1)
            )
        );
        assert!(recurrence.is_ok());
    }

    #[test]
    fn every_week_with_invalid_duration() {
        let freq = Frequency::Weekly {
            interval: 1,
            by_day: vec![],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::weeks(2)
            )
        );
        assert!(recurrence.is_err());
    }

    #[test]
    fn every_week_by_day_with_invalid_duration() {
        let freq = Frequency::Weekly {
            interval: 1,
            by_day: vec![
                Weekday::Mon,
                Weekday::Wed,
                Weekday::Fri
            ],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::days(3)
            )
        );
        assert!(recurrence.is_err());
    }
}

#[cfg(test)]
mod monthly_validations {
    use std::str::FromStr;
    use chrono::{DateTime, Duration, Utc, Weekday};
    use crate::frequencies::{Frequency, NthWeekday};
    use crate::recurrences::Recurrence;
    use super::*;
    
    #[test]
    fn every_month() {
        let freq = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::days(30)
            )
        );
        assert!(recurrence.is_ok());
    }

    #[test]
    fn every_month_by_nth_weekday() {
        let every_mon_and_fri = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday::new(Weekday::Wed, 1),
                NthWeekday::new(Weekday::Fri, 1)
            ],
        };
        let recurrence = Recurrence::new(
            every_mon_and_fri,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::hours(1)
            )
        );
        assert!(recurrence.is_ok());
    }

    #[test]
    fn every_month_with_invalid_duration() {
        let freq = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::days(31)
            )
        );
        assert!(recurrence.is_err());
    }

    #[test]
    fn every_month_by_month_day_invalid_duration() {
        let freq = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![1, 15],
            nth_weekdays: vec![],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::days(15)
            )
        );
        assert!(recurrence.is_err());
    }

    #[test]
    fn every_month_by_weekday_invalid_duration() {
        let freq = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday::new(Weekday::Mon, 1),
                NthWeekday::new(Weekday::Wed, 1),
                NthWeekday::new(Weekday::Fri, 1)
            ],
        };
        let recurrence = Recurrence::new(
            freq,
            DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
            None,
            Some(
                Duration::days(15)
            )
        );
        assert!(recurrence.is_err());
    }
}
