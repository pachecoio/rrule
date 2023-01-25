use crate::frequencies::{Frequency, Time};
use std::str::FromStr;
use chrono::{DateTime, Timelike, Utc, Datelike, Weekday};

#[cfg(test)]
mod secondly_frequency {
    use super::*;

    #[test]
    fn every_second_frequency() {
        let f = Frequency::Secondly { interval: 1 };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Secondly { interval: 0 };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Secondly { interval: -1 };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_second_collect_events() {
        let f = Frequency::Secondly { interval: 1 };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().second(), now.second() + 1);
    }

    #[test]
    fn collect_events_that_span_to_another_minute() {
        let f = Frequency::Secondly { interval: 30 };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().second(), 29);

        let next_event = f.next_event(&next_event.unwrap());
        assert_eq!(next_event.unwrap().second(), 59);
    }
}

#[cfg(test)]
mod minutely_frequency {
    use std::str::FromStr;
    use chrono::Timelike;
    use super::*;

    #[test]
    fn every_minute_frequency() {
        let f = Frequency::Minutely { interval: 1 };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Minutely { interval: 0 };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Minutely { interval: -1 };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_minute_collect_events() {
        let f = Frequency::Minutely { interval: 1 };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().minute(), now.minute() + 1);
    }

    #[test]
    fn collect_events_that_span_to_another_hour() {
        let f = Frequency::Minutely { interval: 30 };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().minute(), 30);

        let next_event = f.next_event(&next_event.unwrap());
        assert_eq!(next_event.unwrap().minute(), 0);
    }
}

#[cfg(test)]
mod hourly_frequency {
    use std::str::FromStr;
    use chrono::{Datelike, Timelike};
    use super::*;

    #[test]
    fn every_hour_frequency() {
        let f = Frequency::Hourly { interval: 1 };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Hourly { interval: 0 };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Hourly { interval: -1 };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_hour_collect_events() {
        let f = Frequency::Hourly { interval: 1 };
        let now = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().hour(), 1);
    }

    #[test]
    fn collect_events_that_span_to_another_day() {
        let f = Frequency::Hourly { interval: 12 };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().hour(), 12);

        let next_event = f.next_event(&next_event.unwrap());
        assert_eq!(next_event.unwrap().hour(), 0);
        assert_eq!(next_event.unwrap().day(), 2);
    }
}

#[cfg(test)]
mod daily_frequency {
    use std::str::FromStr;
    use chrono::Datelike;
    use super::*;

    #[test]
    fn every_day_frequency() {
        let f = Frequency::Daily {
            interval: 1,
            by_time: vec![]
        };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Daily { interval: 0, by_time: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Daily { interval: -1, by_time: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_day_collect_events() {
        let f = Frequency::Daily { interval: 1, by_time: vec![] };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().day(), now.day() + 1);
    }

    #[test]
    fn collect_events_that_span_to_another_month() {
        let f = Frequency::Daily { interval: 15, by_time: vec![] };
        let date = DateTime::<Utc>::from_str("2020-01-02T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().day(), 17);

        let next_event = f.next_event(&next_event.unwrap());
        assert_eq!(next_event.unwrap().day(), 1);
        assert_eq!(next_event.unwrap().month(), 2);
    }
}

#[cfg(test)]
mod daily_frequencies_by_hour {
    use super::*;

    #[test]
    fn twice_a_day() {
        let f = Frequency::Daily { interval: 1, by_time: vec![
            Time::from_str("00:00").unwrap(),
            Time::from_str("12:00").unwrap()
        ] };
        let now = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&now).unwrap();
        assert_eq!(next_event.day(), now.day());
        assert_eq!(next_event.hour(), 12);
        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), now.day() + 1);
        assert_eq!(next_event.hour(), 0);
    }

    #[test]
    fn twice_a_day_with_interval() {
        let f = Frequency::Daily { interval: 2, by_time: vec![
            Time::from_str("00:00").unwrap(),
            Time::from_str("12:00").unwrap()
        ] };
        let now = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&now).unwrap();
        assert_eq!(next_event.day(), now.day());
        assert_eq!(next_event.hour(), 12);
        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), now.day() + 2);
        assert_eq!(next_event.hour(), 0);
    }
}

#[cfg(test)]
mod weekly_frequency {
    use std::ops::Add;
    use std::str::FromStr;
    use chrono::{Datelike, Duration, Timelike};
    use super::*;

    #[test]
    fn every_week_frequency() {
        let f = Frequency::Weekly {
            interval: 1,
            by_day: vec![],
        };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::Weekly { interval: 0, by_day: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::Weekly { interval: -1, by_day: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_week_collect_events() {
        let f = Frequency::Weekly { interval: 1, by_day: vec![] };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().day(), now.add(Duration::weeks(1)).day());
    }

    #[test]
    fn collect_events_that_span_to_another_month() {
        let f = Frequency::Weekly { interval: 1, by_day: vec![] };
        let date = DateTime::<Utc>::from_str("2020-01-28T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().day(), 4);
        assert_eq!(next_event.unwrap().month(), 2);
    }
}

#[cfg(test)]
mod weekly_by_day {
    use super::*;

    #[test]
    fn every_monday() {
        let f = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon],
        };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.weekday(), Weekday::Mon);
        assert_eq!(next_event.day(), 6);

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.weekday(), Weekday::Mon);
        assert_eq!(next_event.day(), 13);
    }

    #[test]
    fn twice_a_week() {
        let f = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Wed],
        };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.weekday(), Weekday::Mon);
        assert_eq!(next_event.day(), 2);
    }
}

#[cfg(test)]
mod monthly_frequency {
    use std::str::FromStr;
    use chrono::{Datelike, Duration, Timelike};
    use super::*;

    #[test]
    fn every_month_frequency() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![],
        };
        let result = f.is_valid();
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_interval() {
        let f = Frequency::MonthlyByDay { interval: 0, by_month_day: vec![], nth_weekdays: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());

        let f = Frequency::MonthlyByDay { interval: -1, by_month_day: vec![], nth_weekdays: vec![] };
        let result = f.is_valid();
        assert!(result.is_err());
    }

    #[test]
    fn every_month_collect_events() {
        let f = Frequency::MonthlyByDay { interval: 1, by_month_day: vec![], nth_weekdays: vec![] };
        let now = Utc::now();
        let next_event = f.next_event(&now);
        assert_eq!(next_event.unwrap().month(), now.month() + 1);
    }

    #[test]
    fn collect_events_that_span_to_another_year() {
        let f = Frequency::MonthlyByDay { interval: 1, by_month_day: vec![], nth_weekdays: vec![] };
        let date = DateTime::<Utc>::from_str("2020-12-02T00:00:59Z").unwrap();
        let next_event = f.next_event(&date);
        assert_eq!(next_event.unwrap().month(), 1);
        assert_eq!(next_event.unwrap().year(), 2021);
    }
}

#[cfg(test)]
mod monthly_by_month_day {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn every_1st_of_month() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![1],
            nth_weekdays: vec![],
        };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 1, "next event should be the 1st of the month");
        assert_eq!(next_event.month(), 2, "next event should be in the next month");

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), 1);
        assert_eq!(next_event.month(), 3);
    }

    #[test]
    fn every_1st_and_15th_of_the_month() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![1, 15],
            nth_weekdays: vec![],
        };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 15, "next event should be the 15th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), 1, "next event should be the 1st of the month");
        assert_eq!(next_event.month(), 2, "next event should be in the next month");
    }

    #[test]
    fn every_31th() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![31],
            nth_weekdays: vec![],
        };
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 31, "next event should be the 31th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");

        let next_event = f.next_event(&next_event);
        assert!(next_event.is_none(), "next event should be none because february does not have a 31th day");
    }
}

#[cfg(test)]
mod monthly_by_weekday {
    use std::str::FromStr;
    use crate::frequencies::NthWeekday;
    use super::*;

    #[test]
    fn every_1st_monday_of_the_month() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Mon,
                },
            ],
        };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 2, "next event should be the 2th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");
    }

    #[test]
    fn every_2nd_tuesday() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday {
                    week_number: 2,
                    weekday: Weekday::Tue,
                },
            ],
        };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 10, "next event should be the 10th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");
    }

    #[test]
    fn every_1st_wednesday_and_friday() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Wed,
                },
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Fri,
                },
            ],
        };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 4, "next event should be the 4th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), 6, "next event should be the 6th of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");

        let next_event = f.next_event(&next_event).unwrap();
        assert_eq!(next_event.day(), 1, "next event should be the 1st of the month");
    }
}

#[cfg(test)]
mod yearly_frequencies {
    use super::*;

    #[test]
    fn once_a_year() {
        let f = Frequency::Yearly {
            interval: 1,
            by_month: 0,
            by_day: vec![],
            by_week_number: vec![],
        };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next_event = f.next_event(&date).unwrap();
        assert_eq!(next_event.day(), 1, "next event should be the 1st of the month");
        assert_eq!(next_event.month(), 1, "next event should be in the same month");
        assert_eq!(next_event.year(), 2024, "next event should be in the next year");
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::frequencies::NthWeekday;
    use super::*;

    #[test]
    fn test_date_within_frequency() {
        let f = Frequency::Daily { interval: 1, by_time: vec![] };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let result = f.contains(&date);
        assert!(result);
    }

    #[test]
    fn not_within_frequency() {
        let f = Frequency::Daily { interval: 1, by_time: vec![
            Time::from_str("12:00:00").unwrap(),
        ] };
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let result = f.contains(&date);
        assert!(!result);
    }

    #[test]
    fn within_daily_frequency() {
        let f = Frequency::Daily { interval: 1, by_time: vec![] };
        let date_within_frequency = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let result = f.contains(&date_within_frequency);
        assert!(result);
    }

    #[test]
    fn within_twice_a_day_frequency() {
        let f = Frequency::Daily { interval: 1, by_time: vec![
            Time::from_str("12:00:00").unwrap(),
            Time::from_str("18:00:00").unwrap(),
        ] };
        let date_within_frequency = DateTime::<Utc>::from_str("2023-01-01T12:00:00Z").unwrap();
        let result = f.contains(&date_within_frequency);
        assert!(result);

        let date_not_within_frequency = DateTime::<Utc>::from_str("2023-01-01T17:00:00Z").unwrap();
        let result = f.contains(&date_not_within_frequency);
        assert!(!result);
    }

    #[test]
    fn within_weekly_frequency() {
        let f = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Wed],
        };
        let date_within_frequency = DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap();
        let result = f.contains(&date_within_frequency);
        assert!(result);

        let date_not_within_frequency = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let result = f.contains(&date_not_within_frequency);
        assert!(!result);
    }

    #[test]
    fn within_weekly_by_day_frequency() {
        let f = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Wed],
        };
        let monday = DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap();
        let result = f.contains(&monday);
        assert!(result);

        let tuesday = DateTime::<Utc>::from_str("2023-01-03T00:00:00Z").unwrap();
        let result = f.contains(&tuesday);
        assert!(!result);
    }

    #[test]
    fn within_monthly_frequency() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![],
        };
        let date_within_frequency = DateTime::<Utc>::from_str("2023-01-15T00:00:00Z").unwrap();
        let result = f.contains(&date_within_frequency);
        assert!(result);
    }

    #[test]
    fn within_monthly_by_month_day() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![15],
            nth_weekdays: vec![],
        };
        let date_within_frequency = DateTime::<Utc>::from_str("2023-01-15T00:00:00Z").unwrap();
        let result = f.contains(&date_within_frequency);
        assert!(result);

        let date_not_within_frequency = DateTime::<Utc>::from_str("2023-01-16T00:00:00Z").unwrap();
        let result = f.contains(&date_not_within_frequency);
        assert!(!result);
    }

    #[test]
    fn within_monthly_by_day() {
        let f = Frequency::MonthlyByDay {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday::new(
                    Weekday::Wed,
                    1,
                ),
                NthWeekday::new(
                    Weekday::Fri,
                    1,
                ),
            ],
        };
        let wednesday = DateTime::<Utc>::from_str("2023-01-04T00:00:00Z").unwrap();
        let result = f.contains(&wednesday);
        assert!(result);

        let thursday = DateTime::<Utc>::from_str("2023-01-05T00:00:00Z").unwrap();
        let result = f.contains(&thursday);
        assert!(!result);
    }
}
