use crate::frequencies::InvalidFrequency;
use crate::{Frequency, MonthlyDate, NthWeekday, Time};
use chrono::{Month, Weekday};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

trait WeekdayUtils {
    fn to_string(&self) -> String;
    fn from_str_short(s: &str) -> Result<Weekday, InvalidFrequency>;
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

    fn from_str_short(s: &str) -> Result<Weekday, InvalidFrequency> {
        match s {
            "MO" => Ok(Weekday::Mon),
            "TU" => Ok(Weekday::Tue),
            "WE" => Ok(Weekday::Wed),
            "TH" => Ok(Weekday::Thu),
            "FR" => Ok(Weekday::Fri),
            "SA" => Ok(Weekday::Sat),
            "SU" => Ok(Weekday::Sun),
            _ => Err(InvalidFrequency::Day {
                message: format!("Invalid day: {s}"),
            })?,
        }
    }
}

impl Display for NthWeekday {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.week_number,
            WeekdayUtils::to_string(&self.weekday)
        )
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequency::Secondly { interval } => {
                write!(f, "FREQ=SECONDLY;INTERVAL={interval}")
            }
            Frequency::Minutely { interval } => {
                write!(f, "FREQ=MINUTELY;INTERVAL={interval}")
            }
            Frequency::Hourly { interval } => {
                write!(f, "FREQ=HOURLY;INTERVAL={interval}")
            }
            Frequency::Daily { interval, by_time } => {
                let mut value = format!("FREQ=DAILY;INTERVAL={interval}");
                if by_time.is_empty() {
                    return write!(f, "{value}");
                }
                let by_time_values: Vec<String> =
                    by_time.iter().map(|time| time.to_string()).collect();
                value.push_str(&format!(";BYTIME={}", by_time_values.join(",")));
                write!(f, "{value}")
            }
            Frequency::Weekly { interval, by_day } => {
                let mut value = format!("FREQ=WEEKLY;INTERVAL={interval}");
                if by_day.is_empty() {
                    return write!(f, "{value}");
                }
                let by_day_values: Vec<String> =
                    by_day.iter().map(WeekdayUtils::to_string).collect();
                value.push_str(&format!(";BYDAY={}", by_day_values.join(",")));
                write!(f, "{value}")
            }
            Frequency::Monthly {
                interval,
                by_month_day,
                nth_weekdays,
            } => {
                let mut value = format!("FREQ=MONTHLY;INTERVAL={interval}");

                if !by_month_day.is_empty() {
                    let by_month_day_values: Vec<String> =
                        by_month_day.iter().map(|day| day.to_string()).collect();
                    value.push_str(&format!(";BYMONTHDAY={}", by_month_day_values.join(",")));
                }

                if !nth_weekdays.is_empty() {
                    let nth_weekdays_values: Vec<String> = nth_weekdays
                        .iter()
                        .map(|nth_weekday| nth_weekday.to_string())
                        .collect();
                    value.push_str(&format!(";BYDAY={}", nth_weekdays_values.join(",")));
                }

                write!(f, "{value}")
            }
            Frequency::Yearly {
                interval,
                by_monthly_date,
            } => {
                let mut value = format!("FREQ=YEARLY;INTERVAL={interval}");
                if let Some(by_monthly_date) = by_monthly_date {
                    value.push_str(&format!(
                        ";BYMONTH={};BYMONTHDAY={}",
                        by_monthly_date.month.number_from_month(),
                        by_monthly_date.day
                    ));
                }
                write!(f, "{value}")
            }
        }
    }
}

impl FromStr for Frequency {
    type Err = InvalidFrequency;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (frequency, s) = match extract_frequency(s) {
            Some(frequency) => frequency,
            None => {
                return Err(InvalidFrequency::Format {
                    message: format!("Cannot parse frequency from value {s}"),
                })
            }
        };

        match frequency.as_ref() {
            "SECONDLY" => parse_secondly(&s),
            "MINUTELY" => parse_minutely(&s),
            "HOURLY" => parse_hourly(&s),
            "DAILY" => parse_daily(&s),
            "WEEKLY" => parse_weekly(&s),
            "MONTHLY" => parse_monthly(&s),
            "YEARLY" => parse_yearly(&s),
            _ => Err(InvalidFrequency::Format {
                message: format!("Frequency {frequency} is not supported"),
            }),
        }
    }
}

impl FromStr for NthWeekday {
    type Err = InvalidFrequency;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;
        let re = Regex::new(r"^(?P<week_number>\d+)(?P<weekday>[A-Z]{2})$").unwrap();
        match re.captures(s) {
            Some(captures) => {
                let week_number = match captures
                    .name("week_number")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                {
                    Ok(week_number) => week_number,
                    Err(_) => {
                        return Err(InvalidFrequency::Format {
                            message: format!("Cannot parse week number from value {s}"),
                        })
                    }
                };
                let weekday = Weekday::from_str_short(captures.name("weekday").unwrap().as_str())?;
                Ok(NthWeekday {
                    week_number,
                    weekday,
                })
            }
            None => {
                return Err(InvalidFrequency::Format {
                    message: format!("Cannot parse nth weekday from value {s}"),
                })
            }
        }
    }
}

impl FromStr for MonthlyDate {
    type Err = InvalidFrequency;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use regex::Regex;
        let re = Regex::new(r"^(?P<month>[A-Z]{3})(?P<day>\d+)$").unwrap();
        match re.captures(s) {
            Some(captures) => {
                let month_number = match captures
                    .name("month")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                {
                    Ok(month_number) => month_number,
                    Err(_) => {
                        return Err(InvalidFrequency::Format {
                            message: format!("Cannot parse month number from value {s}"),
                        })
                    }
                };
                let month = Month::from_i32(month_number)?;
                let day = match captures.name("day").unwrap().as_str().parse::<u8>() {
                    Ok(day) => day,
                    Err(_) => {
                        return Err(InvalidFrequency::Format {
                            message: format!("Cannot parse day from value {s}"),
                        })
                    }
                };
                Ok(MonthlyDate { month, day: day.into() })
            }
            None => {
                return Err(InvalidFrequency::Format {
                    message: format!("Cannot parse monthly date from value {s}"),
                })
            }
        }
    }
}

trait MonthUtils {
    fn from_i32(month: i32) -> Result<Month, InvalidFrequency>;
}

impl MonthUtils for Month {
    fn from_i32(month: i32) -> Result<Month, InvalidFrequency> {
        match month {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(InvalidFrequency::Format {
                message: format!("Cannot parse month from value {month}"),
            }),
        }
    }
}

fn parse_secondly(s: &str) -> Result<Frequency, InvalidFrequency> {
    let (interval, _) = match extract_interval(s) {
        Some(interval) => interval,
        None => {
            return Err(InvalidFrequency::Format {
                message: format!("Cannot parse interval from value {s}"),
            })
        }
    };
    Ok(Frequency::Secondly { interval })
}

fn parse_minutely(s: &String) -> Result<Frequency, InvalidFrequency> {
    let (interval, _) = match extract_interval(s) {
        Some(interval) => interval,
        None => {
            return Err(InvalidFrequency::Format {
                message: format!("Cannot parse interval from value {s}"),
            })
        }
    };
    Ok(Frequency::Minutely { interval })
}

fn parse_hourly(s: &String) -> Result<Frequency, InvalidFrequency> {
    let (interval, _) = match extract_interval(s) {
        Some(interval) => interval,
        None => {
            return Err(InvalidFrequency::Format {
                message: format!("Cannot parse interval from value {s}"),
            })
        }
    };
    Ok(Frequency::Hourly { interval })
}

fn parse_daily(s: &String) -> Result<Frequency, InvalidFrequency> {
    let (interval, s) = match extract_interval(s) {
        Some(interval) => interval,
        None => {
            return Err(InvalidFrequency::Format {
                message: format!("Cannot parse interval from value {s}"),
            })
        }
    };

    let (by_time, _s) = extract_times(&s)?;
    Ok(Frequency::Daily { interval, by_time })
}

fn parse_weekly(s: &String) -> Result<Frequency, InvalidFrequency> {
    let (interval, s) = match extract_interval(s) {
        Some(interval) => interval,
        None => {
            return Err(InvalidFrequency::Format {
                message: format!("Cannot parse interval from value {s}"),
            })
        }
    };

    let (by_day, _) = extract_weekdays(&s)?;
    Ok(Frequency::Weekly { interval, by_day })
}

fn parse_monthly(s: &String) -> Result<Frequency, InvalidFrequency> {
    let (interval, s) = match extract_interval(s) {
        Some(interval) => interval,
        None => {
            return Err(InvalidFrequency::Format {
                message: format!("Cannot parse interval from value {s}"),
            })
        }
    };

    let (by_month_day, s) = extract_monthdays(&s)?;
    let (nth_weekdays, _) = extract_nth_weekdays(&s)?;

    Ok(Frequency::Monthly {
        interval,
        by_month_day,
        nth_weekdays,
    })
}

fn parse_yearly(s: &String) -> Result<Frequency, InvalidFrequency> {
    let (interval, s) = match extract_interval(s) {
        Some(interval) => interval,
        None => {
            return Err(InvalidFrequency::Format {
                message: format!("Cannot parse interval from value {s}"),
            })
        }
    };

    let (by_monthly_date, _) = extract_monthly_date(&s)?;

    Ok(Frequency::Yearly {
        interval,
        by_monthly_date
    })
}

fn extract_frequency(s: &str) -> Option<(String, String)> {
    use regex::Regex;
    let re = Regex::new(r"FREQ=[A-Z]*;").unwrap();
    match re.find(s) {
        Some(pair) => {
            let (key, value) = split_key_value(&pair)?;
            Some((value.clone(), s.replace(&format!("{key}={value};"), "")))
        }
        None => None,
    }
}

fn extract_interval(s: &str) -> Option<(i32, String)> {
    use regex::Regex;
    let re = Regex::new(r"INTERVAL=[0-9]*").unwrap();
    match re.find(s) {
        Some(pair) => {
            let (_, value) = split_key_value(&pair)?;
            match value.parse::<i32>() {
                Ok(v) => Some((v, s.replace(&format!("INTERVAL={value};"), ""))),
                Err(_) => None,
            }
        }
        None => None,
    }
}

fn extract_times(s: &str) -> Result<(Vec<Time>, String), InvalidFrequency> {
    use regex::Regex;
    if !s.contains("BYTIME") {
        return Ok((vec![], s.to_string()));
    }
    let re = Regex::new(r"BYTIME=[0-9|:|,]*").unwrap();
    match re.find(s) {
        Some(pair) => {
            let (_, value) = match split_key_value(&pair) {
                Some(res) => res,
                None => {
                    return Err(InvalidFrequency::Format {
                        message: format!("Cannot parse by_time from value {s}"),
                    })
                }
            };
            let mut times: Vec<Time> = vec![];
            for time in value.split(',') {
                match Time::from_str(time) {
                    Ok(t) => times.push(t),
                    Err(_) => {
                        return Err(InvalidFrequency::Format {
                            message: format!("Cannot parse time from value {time}"),
                        })
                    }
                }
            }
            Ok((times, s.replace(&format!("BYTIME={value}"), "")))
        }
        None => Err(InvalidFrequency::Format {
            message: format!("Cannot parse by_time from value {s}"),
        }),
    }
}

fn extract_weekdays(s: &str) -> Result<(Vec<Weekday>, String), InvalidFrequency> {
    use regex::Regex;
    if !s.contains("BYDAY") {
        return Ok((vec![], s.to_string()));
    }
    let re = Regex::new(r"BYDAY=[A-Z|,]*").unwrap();
    match re.find(s) {
        Some(pair) => {
            let (_, value) = match split_key_value(&pair) {
                Some(res) => res,
                None => {
                    return Err(InvalidFrequency::Format {
                        message: format!("Cannot parse by_day from value {s}"),
                    })
                }
            };
            let mut weekdays: Vec<Weekday> = vec![];
            for weekday in value.split(',') {
                match Weekday::from_str_short(weekday) {
                    Ok(w) => weekdays.push(w),
                    Err(_) => {
                        return Err(InvalidFrequency::Format {
                            message: format!("Cannot parse weekday from value {weekday}"),
                        })
                    }
                }
            }
            Ok((weekdays, s.replace(&format!("BYDAY={value}"), "")))
        }
        None => Err(InvalidFrequency::Format {
            message: format!("Cannot parse by_day from value {s}"),
        }),
    }
}

fn extract_monthdays(s: &str) -> Result<(Vec<i32>, String), InvalidFrequency> {
    use regex::Regex;
    if !s.contains("BYMONTHDAY") {
        return Ok((vec![], s.to_string()));
    }
    let re = Regex::new(r"BYMONTHDAY=[0-9|,]*").unwrap();
    match re.find(s) {
        Some(pair) => {
            let (_, value) = match split_key_value(&pair) {
                Some(res) => res,
                None => {
                    return Err(InvalidFrequency::Format {
                        message: format!("Cannot parse by_monthday from value {s}"),
                    })
                }
            };
            let mut monthdays: Vec<i32> = vec![];
            for monthday in value.split(',') {
                match monthday.parse::<i32>() {
                    Ok(m) => monthdays.push(m),
                    Err(_) => {
                        return Err(InvalidFrequency::Format {
                            message: format!("Cannot parse monthday from value {monthday}"),
                        })
                    }
                }
            }
            Ok((monthdays, s.replace(&format!("BYMONTHDAY={value}"), "")))
        }
        None => Err(InvalidFrequency::Format {
            message: format!("Cannot parse by_monthday from value {s}"),
        }),
    }
}

fn extract_nth_weekdays(s: &str) -> Result<(Vec<NthWeekday>, String), InvalidFrequency> {
    use regex::Regex;
    if !s.contains("BYDAY") {
        return Ok((vec![], s.to_string()));
    }
    let re = Regex::new(r"BYDAY=[0-9|A-Z|,]*").unwrap();
    match re.find(s) {
        Some(pair) => {
            let (_, value) = match split_key_value(&pair) {
                Some(res) => res,
                None => {
                    return Err(InvalidFrequency::Format {
                        message: format!("Cannot parse by_day from value {s}"),
                    })
                }
            };
            let mut nth_weekdays: Vec<NthWeekday> = vec![];
            for nth_weekday in value.split(',') {
                match NthWeekday::from_str(nth_weekday) {
                    Ok(n) => nth_weekdays.push(n),
                    Err(_) => {
                        return Err(InvalidFrequency::Format {
                            message: format!("Cannot parse nth_weekday from value {nth_weekday}"),
                        })
                    }
                }
            }
            Ok((nth_weekdays, s.replace(&format!("BYDAY={value}"), "")))
        }
        None => Err(InvalidFrequency::Format {
            message: format!("Cannot parse by_day from value {s}"),
        }),
    }
}

fn extract_months(s: &str) -> Result<(Vec<Month>, String), InvalidFrequency> {
    use regex::Regex;
    if !s.contains("BYMONTH") {
        return Ok((vec![], s.to_string()));
    }
    let re = Regex::new(r"BYMONTH=[0-9|,]*").unwrap();
    match re.find(s) {
        Some(pair) => {
            let (_, value) = match split_key_value(&pair) {
                Some(res) => res,
                None => {
                    return Err(InvalidFrequency::Format {
                        message: format!("Cannot parse by_month from value {s}"),
                    })
                }
            };
            let mut months: Vec<Month> = vec![];
            for month in value.split(',') {
                match month.parse::<i32>() {
                    Ok(m) => months.push(Month::from_i32(m)?),
                    Err(_) => {
                        return Err(InvalidFrequency::Format {
                            message: format!("Cannot parse month from value {month}"),
                        })
                    }
                }
            }
            Ok((months, s.replace(&format!("BYMONTH={value}"), "")))
        }
        None => Err(InvalidFrequency::Format {
            message: format!("Cannot parse by_month from value {s}"),
        }),
    }
}

fn extract_monthly_date(s: &str) -> Result<(Option<MonthlyDate>, String), InvalidFrequency> {
    if !s.contains("BYMONTHDAY") && !s.contains("BYMONTH") {
        return Ok((None, s.to_string()));
    }
    let (days, s) = extract_monthdays(s)?;
    if days.len() != 1 {
        return Err(InvalidFrequency::Format {
            message: format!("Cannot parse monthly_date from value {s}"),
        });
    }
    let (months, s) = extract_months(&s)?;
    if months.len() != 1 {
        return Err(InvalidFrequency::Format {
            message: format!("Cannot parse monthly_date from value {s}"),
        });
    }
    let monthly_date = MonthlyDate { day: days[0], month: months[0] };
    Ok((Some(monthly_date), s))
}

fn split_key_value(pair: &regex::Match) -> Option<(String, String)> {
    let res = pair.as_str().to_string();
    let key_value: Vec<&str> = res.split('=').collect();
    if key_value.len() != 2 {
        return None;
    }
    let value = key_value[1];
    if value == ";" {
        return None;
    }
    Some((key_value[0].to_string(), value.replace(';', "")))
}

#[cfg(test)]
mod test_serialize {
    use crate::{Frequency, MonthlyDate, NthWeekday, Time};
    use chrono::{Month, Weekday};
    use std::str::FromStr;

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
        let frequency = Frequency::Daily {
            interval: 1,
            by_time: vec![],
        };
        assert_eq!(frequency.to_string(), "FREQ=DAILY;INTERVAL=1");
    }

    #[test]
    fn test_serialize_daily_by_time() {
        let frequency = Frequency::Daily {
            interval: 1,
            by_time: vec![Time::from_str("09:00").unwrap()],
        };
        assert_eq!(frequency.to_string(), "FREQ=DAILY;INTERVAL=1;BYTIME=09:00");
    }

    #[test]
    fn test_serialize_daily_by_time_multiple() {
        let frequency = Frequency::Daily {
            interval: 1,
            by_time: vec![
                Time::from_str("09:00").unwrap(),
                Time::from_str("10:00").unwrap(),
            ],
        };
        assert_eq!(
            frequency.to_string(),
            "FREQ=DAILY;INTERVAL=1;BYTIME=09:00,10:00"
        );
    }

    #[test]
    fn test_serialize_weekly() {
        let frequency = Frequency::Weekly {
            interval: 1,
            by_day: vec![],
        };
        assert_eq!(frequency.to_string(), "FREQ=WEEKLY;INTERVAL=1");
    }

    #[test]
    fn test_serialize_weekly_by_day() {
        let frequency = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon],
        };
        assert_eq!(frequency.to_string(), "FREQ=WEEKLY;INTERVAL=1;BYDAY=MO");
    }

    #[test]
    fn test_serialize_weekly_by_day_multiple() {
        let frequency = Frequency::Weekly {
            interval: 1,
            by_day: vec![Weekday::Mon, Weekday::Tue],
        };
        assert_eq!(frequency.to_string(), "FREQ=WEEKLY;INTERVAL=1;BYDAY=MO,TU");
    }

    #[test]
    fn test_serialize_monthly() {
        let frequency = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![],
        };
        assert_eq!(frequency.to_string(), "FREQ=MONTHLY;INTERVAL=1");
    }

    #[test]
    fn test_serialize_monthly_by_month_day() {
        let frequency = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![1],
            nth_weekdays: vec![],
        };
        assert_eq!(
            frequency.to_string(),
            "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=1"
        );
    }

    #[test]
    fn test_serialize_monthly_by_month_day_multiple() {
        let frequency = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![1, 2],
            nth_weekdays: vec![],
        };
        assert_eq!(
            frequency.to_string(),
            "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=1,2"
        );
    }

    #[test]
    fn test_serialize_monthly_by_nth_weekday() {
        let frequency = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![NthWeekday::new(Weekday::Mon, 1)],
        };
        assert_eq!(frequency.to_string(), "FREQ=MONTHLY;INTERVAL=1;BYDAY=1MO");
    }

    #[test]
    fn test_serialize_monthly_by_nth_weekday_multiple() {
        let frequency = Frequency::Monthly {
            interval: 1,
            by_month_day: vec![],
            nth_weekdays: vec![
                NthWeekday::new(Weekday::Mon, 1),
                NthWeekday::new(Weekday::Tue, 2),
            ],
        };
        assert_eq!(
            frequency.to_string(),
            "FREQ=MONTHLY;INTERVAL=1;BYDAY=1MO,2TU"
        );
    }

    #[test]
    fn test_serialize_yearly() {
        let frequency = Frequency::Yearly {
            interval: 1,
            by_monthly_date: None,
        };
        assert_eq!(frequency.to_string(), "FREQ=YEARLY;INTERVAL=1");
    }

    #[test]
    fn test_serialize_yearly_by_monthly_date() {
        let frequency = Frequency::Yearly {
            interval: 1,
            by_monthly_date: Some(MonthlyDate {
                month: Month::January,
                day: 1,
            }),
        };
        assert_eq!(
            frequency.to_string(),
            "FREQ=YEARLY;INTERVAL=1;BYMONTH=1;BYMONTHDAY=1"
        );
    }
}

#[cfg(test)]
mod test_helpers {
    use chrono::Month;
    use crate::serializer::{extract_frequency, extract_interval, extract_monthdays, extract_monthly_date, extract_months, extract_nth_weekdays, extract_times, extract_weekdays, WeekdayUtils};

    #[test]
    fn test_extract_frequency() {
        let value = "FREQ=SECONDLY;INTERVAL=1";
        let (freq, remainder) = extract_frequency(&value).unwrap();
        assert_eq!(freq, "SECONDLY");
        assert_eq!(remainder, "INTERVAL=1");
    }

    #[test]
    fn test_extract_interval() {
        let value = "FREQ=SECONDLY;INTERVAL=1";
        let (interval, remainder) = extract_interval(&value).unwrap();
        assert_eq!(interval, 1);
    }

    #[test]
    fn test_extract_interval_invalid() {
        let value = "FREQ=SECONDLY;INTERVAL=INVALID";
        let res = extract_interval(&value);
        assert!(res.is_none());
    }

    #[test]
    fn test_extract_interval_empty() {
        let value = "FREQ=SECONDLY;INTERVAL=";
        let res = extract_interval(&value);
        assert!(res.is_none());
    }

    #[test]
    fn test_extract_interval_with_semicolon() {
        let value = "FREQ=SECONDLY;INTERVAL=1;";
        let res = extract_interval(&value);
        assert!(res.is_some());
        assert_eq!(res.unwrap().0, 1);
    }

    #[test]
    fn test_extract_times() {
        let value = "FREQ=DAILY;INTERVAL=1;BYTIME=10:00";
        let (times, remainder) = extract_times(&value).unwrap();
        assert_eq!(times.len(), 1);
        assert_eq!(times[0].to_string(), "10:00");
        assert_eq!(remainder, "FREQ=DAILY;INTERVAL=1;");
    }

    #[test]
    fn test_extract_times_multiple() {
        let value = "FREQ=DAILY;INTERVAL=1;BYTIME=10:00,11:00";
        let (times, remainder) = extract_times(&value).unwrap();
        assert_eq!(times.len(), 2);
        assert_eq!(times[0].to_string(), "10:00");
        assert_eq!(times[1].to_string(), "11:00");
        assert_eq!(remainder, "FREQ=DAILY;INTERVAL=1;");
    }

    #[test]
    fn test_extract_times_empty() {
        let value = "FREQ=DAILY;INTERVAL=1;BYTIME=";
        let res = extract_times(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_times_invalid() {
        let value = "FREQ=DAILY;INTERVAL=1;BYTIME=INVALID";
        let res = extract_times(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_times_with_semicolon() {
        let value = "FREQ=DAILY;INTERVAL=1;BYTIME=10:00;";
        let (times, remainder) = extract_times(&value).unwrap();
        assert_eq!(times.len(), 1);
        assert_eq!(times[0].to_string(), "10:00");
    }

    #[test]
    fn test_extract_weekdays() {
        let value = "FREQ=WEEKLY;INTERVAL=1;BYDAY=MO,TU";
        let (weekdays, remainder) = extract_weekdays(&value).unwrap();
        assert_eq!(weekdays.len(), 2);
        assert_eq!(WeekdayUtils::to_string(&weekdays[0]), "MO");
        assert_eq!(WeekdayUtils::to_string(&weekdays[1]), "TU");
    }

    #[test]
    fn test_extract_weekdays_empty() {
        let value = "FREQ=WEEKLY;INTERVAL=1;BYDAY=";
        let res = extract_weekdays(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_weekdays_invalid() {
        let value = "FREQ=WEEKLY;INTERVAL=1;BYDAY=INVALID";
        let res = extract_weekdays(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_weekdays_with_semicolon() {
        let value = "FREQ=WEEKLY;INTERVAL=1;BYDAY=MO,TU;";
        let (weekdays, remainder) = extract_weekdays(&value).unwrap();
        assert_eq!(weekdays.len(), 2);
        assert_eq!(WeekdayUtils::to_string(&weekdays[0]), "MO");
        assert_eq!(WeekdayUtils::to_string(&weekdays[1]), "TU");
    }

    #[test]
    fn test_extract_weekdays_not_present() {
        let value = "FREQ=WEEKLY;INTERVAL=1";
        let (weekdays, _) = extract_weekdays(&value).unwrap();
        assert_eq!(weekdays.len(), 0);
    }

    #[test]
    fn test_extract_monthdays() {
        let value = "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=1,2";
        let (monthdays, remainder) = extract_monthdays(&value).unwrap();
        assert_eq!(monthdays.len(), 2);
        assert_eq!(monthdays[0], 1);
        assert_eq!(monthdays[1], 2);
    }

    #[test]
    fn test_extract_monthdays_empty() {
        let value = "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=";
        let res = extract_monthdays(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_monthdays_invalid() {
        let value = "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=INVALID";
        let res = extract_monthdays(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_nth_weekdays() {
        let value = "FREQ=MONTHLY;INTERVAL=1;BYDAY=1MO,2TU";
        let (nth_weekdays, remainder) = extract_nth_weekdays(&value).unwrap();
        assert_eq!(nth_weekdays.len(), 2);
    }

    #[test]
    fn test_extract_nth_weekdays_empty() {
        let value = "FREQ=MONTHLY;INTERVAL=1;BYDAY=";
        let res = extract_nth_weekdays(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_nth_weekdays_invalid() {
        let value = "FREQ=MONTHLY;INTERVAL=1;BYDAY=INVALID";
        let res = extract_nth_weekdays(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_months() {
        let value = "FREQ=YEARLY;INTERVAL=1;BYMONTH=1,2";
        let (months, remainder) = extract_months(&value).unwrap();
        assert_eq!(months.len(), 2);
    }

    #[test]
    fn test_extract_months_empty() {
        let value = "FREQ=YEARLY;INTERVAL=1;BYMONTH=";
        let res = extract_months(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_months_invalid() {
        let value = "FREQ=YEARLY;INTERVAL=1;BYMONTH=INVALID";
        let res = extract_months(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_months_not_present() {
        let value = "FREQ=YEARLY;INTERVAL=1";
        let (months, _) = extract_months(&value).unwrap();
        assert_eq!(months.len(), 0);
    }

    #[test]
    fn test_extract_yearly_month_date() {
        let value = "FREQ=YEARLY;INTERVAL=1;BYMONTH=1;BYMONTHDAY=1";
        let (monthly_date, remainder) = extract_monthly_date(&value).unwrap();
        let monthly_date = monthly_date.unwrap();
        assert_eq!(monthly_date.day, 1);
        assert_eq!(monthly_date.month, Month::January);
    }

    #[test]
    fn test_extract_yearly_month_date_empty() {
        let value = "FREQ=YEARLY;INTERVAL=1;BYMONTH=1;BYMONTHDAY=";
        let res = extract_monthly_date(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_yearly_month_date_invalid() {
        let value = "FREQ=YEARLY;INTERVAL=1;BYMONTH=1;BYMONTHDAY=INVALID";
        let res = extract_monthly_date(&value);
        assert!(res.is_err());
    }

    #[test]
    fn test_extract_yearly_month_date_not_present() {
        let value = "FREQ=YEARLY;INTERVAL=1;BYMONTH=1";
        let res = extract_monthly_date(&value);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod test_deserialize_from_str {
    use crate::frequencies::InvalidFrequency;
    use crate::Frequency;
    use chrono::{DateTime, Utc};
    use std::str::FromStr;

    #[test]
    fn test_invalid_format() {
        let value = "blabla";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_err());
        let message = frequency.unwrap_err().to_string();
        assert_eq!(
            message,
            "Invalid format: Cannot parse frequency from value blabla"
        );
    }

    #[test]
    fn test_invalid_key() {
        let value = "INVALID-KEY=INVALID";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_err());
    }

    #[test]
    fn secondly_from_str() {
        let value = "FREQ=SECONDLY;INTERVAL=1";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        let now = Utc::now();
        let next = frequency.next_event(&now).unwrap();
        assert_eq!(next, now + chrono::Duration::seconds(1));
    }

    #[test]
    fn minutely_from_str() {
        let value = "FREQ=MINUTELY;INTERVAL=1";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        let now = Utc::now();
        let next = frequency.next_event(&now).unwrap();
        assert_eq!(next, now + chrono::Duration::minutes(1));
    }

    #[test]
    fn hourly_from_str() {
        let value = "FREQ=HOURLY;INTERVAL=1";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        let now = Utc::now();
        let next = frequency.next_event(&now).unwrap();
        assert_eq!(next, now + chrono::Duration::hours(1));
    }

    #[test]
    fn daily_from_str() {
        let value = "FREQ=DAILY;INTERVAL=1";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        let now = Utc::now();
        let next = frequency.next_event(&now).unwrap();
        assert_eq!(next, now + chrono::Duration::days(1));
    }

    #[test]
    fn daily_by_time_from_str() {
        let value = "FREQ=DAILY;INTERVAL=1;BYTIME=10:00";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next = frequency.next_event(&date).unwrap();
        let expected = DateTime::<Utc>::from_str("2020-01-01T10:00:00Z").unwrap();
    }

    #[test]
    fn weekly_from_str() {
        let value = "FREQ=WEEKLY;INTERVAL=1";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        let now = Utc::now();
        let next = frequency.next_event(&now).unwrap();
        assert_eq!(next, now + chrono::Duration::weeks(1));
    }

    #[test]
    fn weekly_by_day_from_str() {
        let value = "FREQ=WEEKLY;INTERVAL=1;BYDAY=MO,TU";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next = frequency.next_event(&date).unwrap();
        let expected = DateTime::<Utc>::from_str("2020-01-06T00:00:00Z").unwrap();
        assert_eq!(next, expected);
    }

    #[test]
    fn monthly_from_str() {
        let value = "FREQ=MONTHLY;INTERVAL=1";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        assert_eq!(frequency.to_string(), "FREQ=MONTHLY;INTERVAL=1");
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next = frequency.next_event(&date).unwrap();
        let expected = DateTime::<Utc>::from_str("2020-02-01T00:00:00Z").unwrap();
    }

    #[test]
    fn monthly_by_monthday_from_str() {
        let value = "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=1,2,3";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        assert_eq!(
            frequency.to_string(),
            "FREQ=MONTHLY;INTERVAL=1;BYMONTHDAY=1,2,3"
        );
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next = frequency.next_event(&date).unwrap();
        let expected = DateTime::<Utc>::from_str("2020-01-02T00:00:00Z").unwrap();
        assert_eq!(next, expected);
    }

    #[test]
    fn monthly_by_nth_weekday_from_str() {
        let value = "FREQ=MONTHLY;INTERVAL=1;BYDAY=1MO";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        assert_eq!(frequency.to_string(), "FREQ=MONTHLY;INTERVAL=1;BYDAY=1MO");
        let date = DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap();
        let next = frequency.next_event(&date).unwrap();
        let expected = DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap();
        assert_eq!(next, expected);
    }

    #[test]
    fn yearly_from_str() {
        let value = "FREQ=YEARLY;INTERVAL=1";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        assert_eq!(frequency.to_string(), "FREQ=YEARLY;INTERVAL=1");
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next = frequency.next_event(&date).unwrap();
        let expected = DateTime::<Utc>::from_str("2021-01-01T00:00:00Z").unwrap();
        assert_eq!(next, expected);
    }

    #[test]
    fn yearly_by_monthday_from_str() {
        let value = "FREQ=YEARLY;INTERVAL=1;BYMONTHDAY=15;BYMONTH=1";
        let frequency = Frequency::from_str(value);
        assert!(frequency.is_ok());
        let frequency = frequency.unwrap();
        let date = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let next = frequency.next_event(&date).unwrap();
        let expected = DateTime::<Utc>::from_str("2020-01-15T00:00:00Z").unwrap();
        assert_eq!(next, expected);
    }
}
