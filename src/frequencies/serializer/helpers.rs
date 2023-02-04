use std::str::FromStr;
use chrono::{Month, Weekday};
use crate::frequencies::InvalidFrequency;
use crate::frequencies::serializer::{MonthUtils, WeekdayUtils};
use crate::{MonthlyDate, NthWeekday, Time};

pub fn extract_frequency(s: &str) -> Option<(String, String)> {
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

pub fn extract_interval(s: &str) -> Option<(i32, String)> {
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

pub fn extract_times(s: &str) -> Result<(Vec<Time>, String), InvalidFrequency> {
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

pub fn extract_weekdays(s: &str) -> Result<(Vec<Weekday>, String), InvalidFrequency> {
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

pub fn extract_monthdays(s: &str) -> Result<(Vec<i32>, String), InvalidFrequency> {
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

pub fn extract_nth_weekdays(s: &str) -> Result<(Vec<NthWeekday>, String), InvalidFrequency> {
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

pub fn extract_months(s: &str) -> Result<(Vec<Month>, String), InvalidFrequency> {
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

pub fn extract_monthly_date(s: &str) -> Result<(Option<MonthlyDate>, String), InvalidFrequency> {
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
    let monthly_date = MonthlyDate {
        day: days[0],
        month: months[0],
    };
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
mod test_helpers {
    use chrono::Month;
    use crate::frequencies::serializer::helpers::{extract_frequency, extract_interval, extract_monthdays, extract_monthly_date, extract_months, extract_nth_weekdays, extract_times, extract_weekdays};
    use crate::frequencies::serializer::WeekdayUtils;

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

