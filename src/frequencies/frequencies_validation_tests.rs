#[cfg(test)]
mod test_daily {
    use crate::frequencies::{Frequency, Time};
    use std::str::FromStr;

    #[test]
    fn validate_daily() {
        let freq = Frequency::Daily {
            interval: 1,
            by_time: vec![],
        };
        assert!(freq.is_valid().is_ok());
    }

    #[test]
    fn validate_daily_with_invalid_interval() {
        let freq = Frequency::Daily {
            interval: 0,
            by_time: vec![],
        };
        assert!(freq.is_valid().is_err());
    }

    #[test]
    fn validate_daily_with_repeated_times() {
        let freq = Frequency::Daily {
            interval: 1,
            by_time: vec![
                Time::from_str("12:00:00").unwrap(),
                Time::from_str("12:00:00").unwrap(),
            ],
        };
        assert!(freq.is_valid().is_err());
    }
}

#[cfg(test)]
mod test_weekly {
    use crate::frequencies::Frequency;
    use chrono::Weekday;

    #[test]
    fn validate_weekly() {
        let freq = Frequency::Weekly {
            interval: 1,
            by_day: vec![],
        };
        assert!(freq.is_valid().is_ok());
    }

    #[test]
    fn validate_weekly_with_invalid_interval() {
        let freq = Frequency::Weekly {
            interval: 0,
            by_day: vec![],
        };
        assert!(freq.is_valid().is_err());
    }

    #[test]
    fn validate_weekly_with_repeated_days() {
        let freq = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Mon],
        };
        assert!(freq.is_valid().is_err());
    }
}

#[cfg(test)]
mod test_monthly {
    use crate::frequencies::{Frequency, NthWeekday};
    use chrono::Weekday;

    #[test]
    fn validate_monthly() {
        let freq = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![],
        };
        assert!(freq.is_valid().is_ok());
    }

    #[test]
    fn validate_monthly_with_invalid_interval() {
        let freq = Frequency::Monthly {
            interval: 0,
            by_month_day: vec![],
            nth_weekdays: vec![],
        };
        assert!(freq.is_valid().is_err());
    }

    #[test]
    fn validate_monthly_with_repeated_month_days() {
        let freq = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![15, 15],
            nth_weekdays: vec![],
        };
        assert!(freq.is_valid().is_err());
    }

    #[test]
    fn validate_monthly_with_repeated_nth_weekdays() {
        let freq = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday::new(Weekday::Mon, 1),
                NthWeekday::new(Weekday::Mon, 1),
            ],
        };
        assert!(freq.is_valid().is_err());
    }
}
