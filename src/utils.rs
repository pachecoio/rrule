use std::cmp::max;
use std::ops::{Add, Sub};
use chrono::{Datelike, DateTime, Duration, Utc, Weekday};
use crate::frequencies::NthWeekday;

pub trait DateUtils {
    fn with_weekday(self, weekday: Weekday) -> Option<Self> where Self: Sized;
    fn shift_days(&self, days: i64) -> Option<DateTime<Utc>>;
    fn shift_weeks(self, days: i64) -> Option<Self> where Self: Sized;
    fn shift_months(self, months: i64) -> Option<Self> where Self: Sized;
    fn shift_years(self, years: i64) -> Option<Self> where Self: Sized;
}

impl DateUtils for DateTime<Utc> {
    fn with_weekday(self, weekday: Weekday) -> Option<Self> {
        if self.weekday() == weekday {
            Some(self)
        } else {
            let diff = self.weekday().num_days_from_monday() as i64
                - weekday.num_days_from_monday() as i64;
            if diff > 0 {
                Some(self.sub(Duration::days(diff)))
            } else {
                Some(self.add(Duration::days(diff.abs())))
            }
        }
    }

    /// Shift the date by the given number of days.
    fn shift_days(&self, days: i64) -> Option<DateTime<Utc>> {
        Some(*self + Duration::days(days))
    }

    fn shift_weeks(self, days: i64) -> Option<Self> {
        Some(self.add(Duration::days(days * 7)))
    }
    fn shift_months(self, months: i64) -> Option<Self> {
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

        match self.with_month(diff as u32) {
            None => None,
            Some(d) => d.shift_years(years as i64)
        }
    }
    fn shift_years(self, years: i64) -> Option<Self> {
        self.with_year(self.year() + years as i32)
    }
}

/// Check if the given date is the first week of the month.
pub fn is_first_week(date: &DateTime<Utc>) -> bool {
    let day = date.day();
    let weekday = date.weekday();
    let weekday_num = weekday.num_days_from_sunday() + 1;
    if day <= 7 && day <= weekday_num {
        return true;
    }
    false
}

/// Return the nth weekday
///
/// E.g. if the date is 2023-01-09 (Monday), it will return 2
///   That is equivalent to the 2nd Monday of the month
pub fn weekday_ordinal(date: &DateTime<Utc>) -> i32 {
    let mut week_number = 1;
    let mut tmp = *date;
    while !is_first_week(&tmp) {
        let _d = tmp.format("%Y-%m-%d").to_string();
        tmp = tmp.shift_weeks(-1).unwrap();
        if tmp.month() != date.month() {
            break;
        }
        week_number += 1;
    }
    week_number
}

pub fn get_next_weekday(date: &DateTime<Utc>, weekdays: &Vec<Weekday>) -> Option<DateTime<Utc>> {
    if weekdays.is_empty() {
        return None;
    }
    let current_weekday_number = date.weekday();
    for weekday in weekdays {
        if current_weekday_number.num_days_from_sunday() < weekday.num_days_from_sunday() {
            return Some(date.with_weekday(*weekday).unwrap());
        }
    }
    // Get first supported weekday of next week
    let d = date.shift_weeks(1).unwrap();
    Some(
        d.with_weekday(weekdays[0]).unwrap()
    )
}

pub fn get_next_nth_weekday(date: &DateTime<Utc>, weekday: &Weekday, nth: i32) -> Option<DateTime<Utc>> {
    let weekdays = vec![*weekday];
    let next = get_next_weekday(date, &weekdays).unwrap();
    let current_week = weekday_ordinal(&next);
    let _d = next.format("%Y-%m-%d").to_string();
    if current_week == nth {
        return Some(next);
    }
    get_next_nth_weekday(&next, weekday, nth)
}

pub fn get_next_nth_weekday_in_range(date: &DateTime<Utc>, weekdays: &[Weekday], ordinals: &[i32]) -> Option<DateTime<Utc>> {
    if weekdays.is_empty() || ordinals.is_empty() {
        return None;
    }
    if weekdays.len() == 1 && ordinals.len() == 1 {
        return get_next_nth_weekday(date, &weekdays[0], ordinals[0]);
    }

    for weekday in weekdays {
        if weekday.num_days_from_sunday() > date.weekday().num_days_from_sunday() {
            return get_next_nth_weekday(date, weekday, ordinals[0]);
        }
    }

    if let Some(next) = get_next_nth_weekday_in_range(date, weekdays, &ordinals[1..]) {
        return Some(next);
    }

    get_next_nth_weekday(date, &weekdays[0], ordinals[0])
}


pub fn potato(current_date: &DateTime<Utc>, nth_weekdays: &Vec<NthWeekday>) -> Option<DateTime<Utc>> {
    let ordered_weekdays = order_nth_weekdays(nth_weekdays);

    let weekday = current_date.weekday();
    let week_number = weekday_ordinal(current_date);

    for nth_weekday in ordered_weekdays {
        if nth_weekday.week_number == week_number && nth_weekday.weekday.num_days_from_sunday() > weekday.num_days_from_sunday() {
            return Some(current_date.add(
                Duration::days((nth_weekday.weekday.num_days_from_sunday() - weekday.num_days_from_sunday()) as i64)
            ))
        } else if nth_weekday.week_number > week_number {
            let days_diff = nth_weekday.weekday.num_days_from_sunday() as i64 - weekday.num_days_from_sunday() as i64;
            let weeks_diff = nth_weekday.week_number - week_number;
            return current_date.shift_days(days_diff as i64)?.shift_weeks(weeks_diff as i64)
        }
    }
    None
}


fn order_nth_weekdays(nth_weekdays: &Vec<NthWeekday>) -> Vec<NthWeekday> {
    let mut result = nth_weekdays.clone();
    result.sort_by(|a, b| a.cmp(&b));
    Vec::from(&result[..])
}

#[cfg(test)]
mod shift_days {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_shift_days() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_days(1).unwrap();
        assert_eq!(result.day(), 2);
    }

    #[test]
    fn test_shift_days_backwards() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_days(-1).unwrap();
        assert_eq!(result.day(), 31);
    }
}

#[cfg(test)]
mod test_shift_weeks {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_shift_weeks() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_weeks(1).unwrap();
        assert_eq!(result.day(), 8);
    }

    #[test]
    fn test_shift_weeks_2() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_weeks(2).unwrap();
        assert_eq!(result.day(), 15);
    }

    #[test]
    fn test_shift_back() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_weeks(-1).unwrap();
        assert_eq!(result.day(), 25);
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
        let result = date.with_weekday(Weekday::Mon).unwrap();
        assert_eq!(result.day(), 31);
        assert_eq!(result.month(), 12);
    }

    #[test]
    fn test_shift_month() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(1).unwrap();
        assert_eq!(result.month(), 2);
    }

    #[test]
    fn test_shift_month_2() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(2).unwrap();
        assert_eq!(result.month(), 3);
    }

    #[test]
    fn test_shift_month_to_next_year() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(14).unwrap();
        assert_eq!(result.month(), 3);
    }

    #[test]
    fn test_shift_backwards() {
        let date = DateTime::<Utc>::from_str("2019-05-01T00:00:00Z").unwrap();
        let result = date.shift_months(-1).unwrap();
        assert_eq!(result.month(), 4);
    }

    #[test]
    fn test_shift_to_previous_year() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(-1).unwrap();
        assert_eq!(result.month(), 12);
    }

    #[test]
    fn test_shift_to_previous_year_2() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_months(-14).unwrap();
        assert_eq!(result.month(), 10);
    }

    #[test]
    fn test_get_ordinal_number_of_the_week() {
        let date = DateTime::<Utc>::from_str("2023-02-06T00:00:00Z").unwrap();
        let num = weekday_ordinal(&date);
        assert_eq!(num, 1);
    }

    #[test]
    fn test_get_weekday_ordinal_last() {
        let date = DateTime::<Utc>::from_str("2023-02-28T00:00:00Z").unwrap();
        let num = weekday_ordinal(&date);
        assert_eq!(num, 4);
    }

    #[test]
    fn test_get_next_weekday() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Mon, Weekday::Tue];

        let next_weekday = get_next_weekday(&date, &weekdays).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-10T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_weekday_next_week() {
        let date = DateTime::<Utc>::from_str("2023-01-03T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Mon, Weekday::Tue];

        let next_weekday = get_next_weekday(&date, &weekdays).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap());

        let next_weekday = get_next_weekday(&next_weekday, &weekdays).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-10T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_weekday_next_month() {
        let date = DateTime::<Utc>::from_str("2023-01-31T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Mon, Weekday::Tue];

        let next_weekday = get_next_weekday(&date, &weekdays).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-02-06T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_second_monday_current_month() {
        let date = DateTime::<Utc>::from_str("2023-01-04T00:00:00Z").unwrap();

        let next_weekday = get_next_nth_weekday(&date, &Weekday::Mon, 2).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_third_monday_current_month() {
        let date = DateTime::<Utc>::from_str("2023-01-04T00:00:00Z").unwrap();

        let next_weekday = get_next_nth_weekday(&date, &Weekday::Mon, 3).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-16T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_first_monday_next_month() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();

        let next_weekday = get_next_nth_weekday(&date, &Weekday::Mon, 1).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-02-06T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_nth_weekday_in_ranges_same_week() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Tue];
        let ordinals = vec![2];

        let next_weekday = get_next_nth_weekday_in_range(&date, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-10T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_nth_weekday_in_ranges() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Mon];
        let ordinals = vec![1];

        let next_weekday = get_next_nth_weekday_in_range(&date, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-02-06T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_nth_weekday_in_ranges_same_month() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Mon];
        let ordinals = vec![3];

        let next_weekday = get_next_nth_weekday_in_range(&date, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-16T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_nth_weekday_with_multiple_weekdays() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Mon, Weekday::Tue];
        let ordinals = vec![2];

        let next_weekday = get_next_nth_weekday_in_range(&date, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-10T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_nth_weekday_with_multiple_ordinals() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Fri];
        let ordinals = vec![2, 3];

        let next_weekday = get_next_nth_weekday_in_range(&date, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-13T00:00:00Z").unwrap());

        let next_weekday = get_next_nth_weekday_in_range(&next_weekday, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-20T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_nth_weekday_with_multiple_weekdays_and_ordinals() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Thu, Weekday::Fri];
        let ordinals = vec![2, 3];

        // Collect second week
        let next_weekday = get_next_nth_weekday_in_range(&date, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-12T00:00:00Z").unwrap());

        let next_weekday = get_next_nth_weekday_in_range(&next_weekday, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-13T00:00:00Z").unwrap());

        // Collect third week
        let next_weekday = get_next_nth_weekday_in_range(&next_weekday, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-19T00:00:00Z").unwrap());

        // let next_weekday = get_next_nth_weekday_in_range(&next_weekday, &weekdays, &ordinals).unwrap();
        // assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-20T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_nth_weekday_when_next_weekday_is_prev() {
        let date = DateTime::<Utc>::from_str("2023-01-13T00:00:00Z").unwrap();
        let weekdays = vec![Weekday::Tue, Weekday::Wed];
        let ordinals = vec![3];

        let next_weekday = get_next_nth_weekday_in_range(&date, &weekdays, &ordinals).unwrap();
        assert_eq!(next_weekday, DateTime::<Utc>::from_str("2023-01-17T00:00:00Z").unwrap());
    }
}

#[cfg(test)]
mod test_shift_years {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_shift_years() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_years(1).unwrap();
        assert_eq!(result.year(), 2020);
    }

    #[test]
    fn test_shift_years_backwards() {
        let date = DateTime::<Utc>::from_str("2019-01-01T00:00:00Z").unwrap();
        let result = date.shift_years(-1).unwrap();
        assert_eq!(result.year(), 2018);
    }
}

#[cfg(test)]
mod test_refactor {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_order_nth_weekdays() {
        let weekdays = vec![
            NthWeekday::new(Weekday::Tue, 3),
            NthWeekday::new(Weekday::Mon, 3),
            NthWeekday::new(Weekday::Wed, 1),
        ];
        let result = order_nth_weekdays(&weekdays);
        assert_eq!(
            result,
            vec![
                NthWeekday::new(Weekday::Wed, 1),
                NthWeekday::new(Weekday::Mon, 3),
                NthWeekday::new(Weekday::Tue, 3),
            ]
        )
    }

    #[test]
    fn test_get_next_nth_weekday_none() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();
        let result = potato(&date, &vec![]);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_next_nth_weekday_base_case() {
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let result = potato(
            &date,
            &vec![NthWeekday::new(Weekday::Mon, 1)]
        );
        assert_eq!(result.unwrap(), DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap());
    }

    #[test]
    fn test_get_next_nth_weekday_multiple_weekdays() {
        let date = DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap();
        let result = potato(
            &date,
            &vec![
                NthWeekday::new(Weekday::Mon, 3),
                NthWeekday::new(Weekday::Tue, 2),
            ]
        );
        assert_eq!(result.unwrap(), DateTime::<Utc>::from_str("2023-01-10T00:00:00Z").unwrap());

        let result = potato(
            &result.unwrap(),
            &vec![
                NthWeekday::new(Weekday::Tue, 2),
                NthWeekday::new(Weekday::Mon, 3),
            ]
        );
        assert_eq!(result.unwrap(), DateTime::<Utc>::from_str("2023-01-16T00:00:00Z").unwrap());
    }

}
