use std::ops::{Add, Sub};
use chrono::{Datelike, DateTime, Duration, Utc, Weekday};

pub trait DateUtils {
    fn with_weekday(self, weekday: Weekday) -> Self;
}

impl DateUtils for DateTime<Utc> {
    fn with_weekday(self, weekday: Weekday) -> Self {
        if self.weekday() == weekday {
            self
        } else {
            let diff = self.weekday().num_days_from_monday() as i64
                - weekday.num_days_from_monday() as i64;
            if diff > 0 {
                self.sub(Duration::days(diff))
            } else {
                self.add(Duration::days(diff.abs()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_with_weekday() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        assert_eq!(date.weekday(), Weekday::Tue);
        let result = date.with_weekday(Weekday::Mon);
        assert_eq!(result.day(), 31);
        assert_eq!(result.month(), 12);
    }
}