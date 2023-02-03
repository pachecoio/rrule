use std::fmt::{Display, Formatter};
use chrono::Weekday;
use crate::{Frequency, MonthlyDate, NthWeekday, Time};

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

trait WeekdayUtils {
    fn to_string(&self) -> String;
}

impl WeekdayUtils for Weekday {
    fn to_string(&self) -> String {
        match self {
            Weekday::Mon => "MO".to_string(),
            Weekday::Tue => "TU".to_string(),
            Weekday::Wed => "WE".to_string(),
            Weekday::Thu => "TH".to_string(),
            Weekday::Fri => "FR".to_string(),
            Weekday::Sat => "SA".to_string(),
            Weekday::Sun => "SU".to_string(),
        }
    }
}

impl Display for NthWeekday {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.week_number, WeekdayUtils::to_string(&self.weekday))
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequency::Secondly { interval } => {
                write!(f, "FREQ=SECONDLY;INTERVAL={}", interval)
            }
            Frequency::Minutely { interval } => {
                write!(f, "FREQ=MINUTELY;INTERVAL={}", interval)
            }
            Frequency::Hourly { interval } => {
                write!(f, "FREQ=HOURLY;INTERVAL={}", interval)
            }
            Frequency::Daily { interval, by_time } => {
                let mut value = format!("FREQ=DAILY;INTERVAL={}", interval);
                if by_time.is_empty() {
                    return write!(f, "{value}");
                }
                let by_time_values: Vec<String> = by_time.iter().map(|time| time.to_string()).collect();
                value.push_str(&format!(";BYTIME={}", by_time_values.join(",")));
                write!(f, "{value}")
            }
            Frequency::Weekly { interval, by_day } => {
                let mut value = format!("FREQ=WEEKLY;INTERVAL={}", interval);
                if by_day.is_empty() {
                    return write!(f, "{value}");
                }
                let by_day_values: Vec<String> = by_day.iter().map(|day| WeekdayUtils::to_string(day)).collect();
                value.push_str(&format!(";BYDAY={}", by_day_values.join(",")));
                write!(f, "{value}")
            }
            Frequency::Monthly { interval, by_month_day, nth_weekdays } => {
                let mut value = format!("FREQ=MONTHLY;INTERVAL={}", interval);

                if !by_month_day.is_empty() {
                    let by_month_day_values: Vec<String> = by_month_day.iter().map(|day| day.to_string()).collect();
                    value.push_str(&format!(";BYMONTHDAY={}", by_month_day_values.join(",")));
                }

                if !nth_weekdays.is_empty() {
                    let nth_weekdays_values: Vec<String> = nth_weekdays.iter().map(|nth_weekday| nth_weekday.to_string()).collect();
                    value.push_str(&format!(";BYDAY={}", nth_weekdays_values.join(",")));
                }

                write!(f, "{value}")
            }
            Frequency::Yearly { interval, by_monthly_date } => {
                let mut value = format!("FREQ=YEARLY;INTERVAL={}", interval);
                write!(f, "{value}")
            }
        }
    }
}

#[cfg(test)]
mod test_serialize {
    use std::str::FromStr;
    use chrono::{Month, Weekday};
    use crate::{Frequency, MonthlyDate, NthWeekday, Time};

    #[test]
    fn test_serialize_secondly() {
        let frequency = Frequency::Secondly { interval: 1 };
        assert_eq!(frequency.to_string(), "FREQ=SECONDLY;INTERVAL=1");
    }

    #[test]
    fn test_serialize_minutely() {
        let frequency = Frequency::Minutely { interval: 1 };
        assert_eq!(frequency.to_string(), "FREQ=MINUTELY;INTERVAL=1");
    }
    
    #[test]
    fn test_serialize_hourly() {
        let frequency = Frequency::Hourly { interval: 1 };
        assert_eq!(frequency.to_string(), "FREQ=HOURLY;INTERVAL=1");
    }

    #[test]
    fn test_serialize_daily() {
        let frequency = Frequency::Daily { interval: 1, by_time: vec![] };
        assert_eq!(frequency.to_string(), "FREQ=DAILY;INTERVAL=1");
    }

    #[test]
    fn test_serialize_daily_by_time() {
        let frequency = Frequency::Daily { interval: 1, by_time: vec![Time::from_str("09:00").unwrap()] };
        assert_eq!(frequency.to_string(), "FREQ=DAILY;INTERVAL=1;BYTIME=09:00");
    }

    #[test]
    fn test_serialize_daily_by_time_multiple() {
        let frequency = Frequency::Daily { interval: 1, by_time: vec![Time::from_str("09:00").unwrap(), Time::from_str("10:00").unwrap()] };
        assert_eq!(frequency.to_string(), "FREQ=DAILY;INTERVAL=1;BYTIME=09:00,10:00");
    }

    #[test]
    fn test_serialize_weekly() {
        let frequency = Frequency::Weekly { interval: 1, by_day: vec![] };
        assert_eq!(frequency.to_string(), "FREQ=WEEKLY;INTERVAL=1");
    }

    #[test]
    fn test_serialize_weekly_by_day() {
        let frequency = Frequency::Weekly { interval: 1, by_day: vec![Weekday::Mon] };
        assert_eq!(frequency.to_string(), "FREQ=WEEKLY;INTERVAL=1;BYDAY=MO");
    }

    #[test]
    fn test_serialize_weekly_by_day_multiple() {
        let frequency = Frequency::Weekly { interval: 1, by_day: vec![Weekday::Mon, Weekday::Tue] };
        assert_eq!(frequency.to_string(), "FREQ=WEEKLY;INTERVAL=1;BYDAY=MO,TU");
    }

    #[test]
    fn test_serialize_monthly() {
        let frequency = Frequency::Monthly { interval: 1, by_month_day: vec![], nth_weekdays: vec![] };
        assert_eq!(frequency.to_string(), "FREQ=MONTHLY;INTERVAL=1");
    }

    #[test]
    fn test_serialize_monthly_by_month_day() {
        let frequency = Frequency::Monthly { interval: 1, by_month_day: vec![1], nth_weekdays: vec![] };
        assert_eq!(frequency.to_string(), "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=1");
    }

    #[test]
    fn test_serialize_monthly_by_month_day_multiple() {
        let frequency = Frequency::Monthly { interval: 1, by_month_day: vec![1, 2], nth_weekdays: vec![] };
        assert_eq!(frequency.to_string(), "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=1,2");
    }

    #[test]
    fn test_serialize_monthly_by_nth_weekday() {
        let frequency = Frequency::Monthly { interval: 1, by_month_day: vec![], nth_weekdays: vec![
            NthWeekday::new(Weekday::Mon, 1),
        ] };
        assert_eq!(frequency.to_string(), "FREQ=MONTHLY;INTERVAL=1;BYDAY=1MO");
    }

    #[test]
    fn test_serialize_monthly_by_nth_weekday_multiple() {
        let frequency = Frequency::Monthly { interval: 1, by_month_day: vec![], nth_weekdays: vec![
            NthWeekday::new(Weekday::Mon, 1),
            NthWeekday::new(Weekday::Tue, 2),
        ] };
        assert_eq!(frequency.to_string(), "FREQ=MONTHLY;INTERVAL=1;BYDAY=1MO,2TU");
    }

    #[test]
    fn test_serialize_yearly() {
        let frequency = Frequency::Yearly { interval: 1, by_monthly_date: None };
        assert_eq!(frequency.to_string(), "FREQ=YEARLY;INTERVAL=1");
    }

}