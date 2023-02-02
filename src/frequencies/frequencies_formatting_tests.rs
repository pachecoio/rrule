#[cfg(test)]
mod secondly_formatting {
    use crate::Frequency;

    #[test]
    fn test_secondly_to_string() {
        let frequency = Frequency::Secondly { interval: 1 };
        assert_eq!(frequency.to_string(), "Every second");
    }

    #[test]
    fn test_secondly_to_string_plural() {
        let frequency = Frequency::Secondly { interval: 30 };
        assert_eq!(frequency.to_string(), "Every 30 seconds");
    }
}

#[cfg(test)]
mod minutely_formatting {
    use crate::Frequency;

    #[test]
    fn test_minutely_to_string() {
        let frequency = Frequency::Minutely { interval: 1 };
        assert_eq!(frequency.to_string(), "Every minute");
    }

    #[test]
    fn test_minutely_to_string_plural() {
        let frequency = Frequency::Minutely { interval: 30 };
        assert_eq!(frequency.to_string(), "Every 30 minutes");
    }
}

#[cfg(test)]
mod hourly_formatting {
    use crate::Frequency;

    #[test]
    fn test_hourly_to_string() {
        let frequency = Frequency::Hourly { interval: 1 };
        assert_eq!(frequency.to_string(), "Every hour");
    }

    #[test]
    fn test_hourly_to_string_plural() {
        let frequency = Frequency::Hourly { interval: 30 };
        assert_eq!(frequency.to_string(), "Every 30 hours");
    }
}

#[cfg(test)]
mod daily_formatting {
    use crate::{Frequency, Time};

    #[test]
    fn test_daily_to_string() {
        let frequency = Frequency::Daily {
            interval: 1,
            by_time: vec![],
        };
        assert_eq!(frequency.to_string(), "Once a day");
    }

    #[test]
    fn test_twice_a_day() {
        let frequency = Frequency::Daily {
            interval: 1,
            by_time: vec![
                Time::from_str("08:00").unwrap(),
                Time::from_str("16:00").unwrap(),
            ],
        };
        assert_eq!(frequency.to_string(), "Twice a day");
    }

    #[test]
    fn test_more_than_two_times_a_day() {
        let frequency = Frequency::Daily {
            interval: 1,
            by_time: vec![
                Time::from_str("08:00").unwrap(),
                Time::from_str("12:00").unwrap(),
                Time::from_str("16:00").unwrap(),
            ],
        };
        assert_eq!(frequency.to_string(), "3 times a day");
    }
}

#[cfg(test)]
mod weekly_formatting {
    use crate::Frequency;
    use chrono::Weekday;

    #[test]
    fn test_weekly_to_string() {
        let frequency = Frequency::Weekly {
            interval: 1,
            by_day: vec![],
        };
        assert_eq!(frequency.to_string(), "Once a week");
    }

    #[test]
    fn test_weekly_to_string_plural() {
        let frequency = Frequency::Weekly {
            interval: 2,
            by_day: vec![],
        };
        assert_eq!(frequency.to_string(), "Once every 2 weeks");
    }

    #[test]
    fn test_weekly_to_string_with_days() {
        let frequency = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Tue],
        };
        assert_eq!(frequency.to_string(), "Twice a week");
    }

    #[test]
    fn test_weekly_to_string_with_days_plural() {
        let frequency = Frequency::Weekly {
            interval: 2,
            by_day: vec![Weekday::Mon, Weekday::Tue],
        };
        assert_eq!(frequency.to_string(), "Twice every 2 weeks");
    }

    #[test]
    fn test_weekly_to_string_more_than_two_times() {
        let frequency = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Tue, Weekday::Wed],
        };
        assert_eq!(frequency.to_string(), "3 times a week");
    }
}

#[cfg(test)]
mod monthly_formatting {
    use crate::{Frequency, NthWeekday};
    use chrono::Weekday;

    #[test]
    fn test_monthly_to_string() {
        let frequency = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![],
        };
        assert_eq!(frequency.to_string(), "Once a month");
    }

    #[test]
    fn test_monthly_to_string_plural() {
        let frequency = Frequency::Monthly {
            interval: 2,
            by_month_day: vec![],
            nth_weekdays: vec![],
        };
        assert_eq!(frequency.to_string(), "Once every 2 months");
    }

    #[test]
    fn test_monthly_to_string_with_days() {
        let frequency = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![1, 2],
            nth_weekdays: vec![],
        };
        assert_eq!(frequency.to_string(), "Twice a month");
    }

    #[test]
    fn test_monthly_to_string_with_days_plural() {
        let frequency = Frequency::Monthly {
            interval: 2,
            by_month_day: vec![1, 2],
            nth_weekdays: vec![],
        };
        assert_eq!(frequency.to_string(), "Twice every 2 months");
    }

    #[test]
    fn test_monthly_to_string_with_nth_weekdays() {
        let frequency = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Mon,
                },
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Tue,
                },
            ],
        };
        assert_eq!(frequency.to_string(), "Twice a month");
    }

    #[test]
    fn test_monthly_to_string_with_nth_weekdays_plural() {
        let frequency = Frequency::Monthly {
            interval: 2,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Mon,
                },
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Tue,
                },
            ],
        };
        assert_eq!(frequency.to_string(), "Twice every 2 months");
    }

    #[test]
    fn test_monthly_to_string_with_nth_weekdays_and_plural() {
        let frequency = Frequency::Monthly {
            interval: 2,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Mon,
                },
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Tue,
                },
                NthWeekday {
                    week_number: 1,
                    weekday: Weekday::Wed,
                },
            ],
        };
        assert_eq!(frequency.to_string(), "3 times every 2 months");
    }
}

#[cfg(test)]
mod yearly_formatting {
    use crate::{Frequency, MonthlyDate};
    use chrono::Month;

    #[test]
    fn test_yearly_to_string() {
        let frequency = Frequency::Yearly {
            interval: 1,
            by_monthly_date: vec![],
        };
        assert_eq!(frequency.to_string(), "Once a year");
    }

    #[test]
    fn test_yearly_to_string_plural() {
        let frequency = Frequency::Yearly {
            interval: 2,
            by_monthly_date: vec![],
        };
        assert_eq!(frequency.to_string(), "Once every 2 years");
    }

    #[test]
    fn test_yearly_to_string_with_days() {
        let frequency = Frequency::Yearly {
            interval: 1,
            by_monthly_date: vec![
                MonthlyDate {
                    month: Month::January,
                    day: 1,
                },
                MonthlyDate {
                    month: Month::January,
                    day: 2,
                },
            ],
        };
        assert_eq!(frequency.to_string(), "Twice a year");
    }

    #[test]
    fn test_yearly_to_string_with_days_plural() {
        let frequency = Frequency::Yearly {
            interval: 2,
            by_monthly_date: vec![
                MonthlyDate {
                    month: Month::January,
                    day: 1,
                },
                MonthlyDate {
                    month: Month::January,
                    day: 2,
                },
            ],
        };
        assert_eq!(frequency.to_string(), "Twice every 2 years");
    }

    #[test]
    fn test_yearly_to_string_with_days_and_plural() {
        let frequency = Frequency::Yearly {
            interval: 2,
            by_monthly_date: vec![
                MonthlyDate {
                    month: Month::January,
                    day: 1,
                },
                MonthlyDate {
                    month: Month::January,
                    day: 2,
                },
                MonthlyDate {
                    month: Month::January,
                    day: 3,
                },
            ],
        };
        assert_eq!(frequency.to_string(), "3 times every 2 years");
    }
}
