
#[cfg(test)]
mod test_daily {
    use crate::frequencies::{Frequency, Time};

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
                Time::from_str("12:00:00").unwrap()
            ],
        };
        assert!(freq.is_valid().is_err());
    }
}