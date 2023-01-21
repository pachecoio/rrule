use std::cmp::max;
use std::ops::{Add, Sub};
use chrono::{Datelike, DateTime, Duration, Utc, Weekday};

pub trait DateUtils {
    fn with_weekday(self, weekday: Weekday) -> Self;
    fn shift_months(self, months: i64) -> Self;
    fn shift_years(self, years: i64) -> Self;
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
    fn shift_months(self, months: i64) -> Self {
        let mut diff = self.month() as i32 + months as i32;

        // If the months shift is bigger than a year we need to shift the year
        let mut years = if diff > 12 {
            diff / 12
        } else if diff < 1 {
            (diff / 12) - 1
        } else {
            0
        };

        if diff == 0 {
            diff = 12;
        } else if diff > 0 {
            diff %= 12;
        } else {
            diff = 12 + (diff % 12) - 1;
        }

        self.with_month(diff as u32).unwrap().shift_years(years as i64)
    }
    fn shift_years(self, years: i64) -> Self {
        self.with_year(self.year() + years as i32).unwrap()
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

    #[test]
    fn test_shift_month() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(1);
        assert_eq!(result.month(), 2);
    }

    #[test]
    fn test_shift_month_2() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(2);
        assert_eq!(result.month(), 3);
    }

    #[test]
    fn test_shift_month_to_next_year() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(14);
        assert_eq!(result.month(), 3);
    }

    #[test]
    fn test_shift_backwards() {
        let date = DateTime::<Utc>::from_str("2019-05-01T00:00:00Z").unwrap();
        let result = date.shift_months(-1);
        assert_eq!(result.month(), 4);
    }

    #[test]
    fn test_shift_to_previous_year() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(-1);
        assert_eq!(result.month(), 12);
    }

    #[test]
    fn test_shift_to_previous_year_2() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(-14);
        assert_eq!(result.month(), 10);
    }

}

#[cfg(test)]
mod test_shift_years {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_shift_years() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_years(1);
        assert_eq!(result.year(), 2020);
    }

    #[test]
    fn test_shift_years_backwards() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_years(-1);
        assert_eq!(result.year(), 2018);
    }
}