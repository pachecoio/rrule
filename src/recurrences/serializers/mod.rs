use std::fmt::{Display, Formatter};
use crate::{Frequency, Recurrence, RecurrenceInvalid};
use chrono::{DateTime, Duration, Utc};
use std::str::FromStr;
use crate::recurrences::MAX_DATE;

impl FromStr for Recurrence {
    type Err = RecurrenceInvalid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let frequency = match Frequency::from_str(s) {
            Ok(f) => f,
            Err(e) => {
                return Err(RecurrenceInvalid {
                    message: format!("Invalid frequency: {e}"),
                })
            }
        };
        let start_date = extract_start_date(s)?;
        let end_date = extract_end_date(s)?;
        let duration = extract_duration(s)?;
        Recurrence::new(frequency, start_date, end_date, Some(duration))
    }
}

impl Display for Recurrence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = format!(
            "{};DTSTART={}",
            self.frequency,
            self.start.format("%Y-%m-%dT%H:%M:%SZ"),
        );
        let max_date = DateTime::<Utc>::from_str(MAX_DATE).unwrap();
        if self.end != max_date {
            res = format!("{};DTEND={}", res, self.end.format("%Y-%m-%dT%H:%M:%SZ"));
        }
        if self.duration > Duration::seconds(0) {
            res = format!("{};DURATION={}", res, self.duration);
        }
        write!(f, "{}", res)
    }
}

pub fn extract_start_date(s: &str) -> Result<DateTime<Utc>, RecurrenceInvalid> {
    use regex::Regex;
    let re =
        Regex::new(r"DTSTART=(?P<date>[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z)")
            .unwrap();
    let caps = re.captures(s).ok_or(RecurrenceInvalid {
        message: "No DTSTART found".to_string(),
    })?;
    let date = caps.name("date").ok_or(RecurrenceInvalid {
        message: "No date found".to_string(),
    })?;
    let date = DateTime::<Utc>::from_str(date.as_str()).map_err(|e| RecurrenceInvalid {
        message: format!("Invalid date: {e}"),
    })?;
    Ok(date)
}

pub fn extract_end_date(s: &str) -> Result<Option<DateTime<Utc>>, RecurrenceInvalid> {
    use regex::Regex;
    let re = Regex::new(r"DTEND=(?P<date>[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z)")
        .unwrap();
    let caps = match re.captures(s).ok_or(RecurrenceInvalid {
        message: "No DTEND found".to_string(),
    }) {
        Ok(caps) => caps,
        Err(e) => return Ok(None),
    };
    let date = caps.name("date").ok_or(RecurrenceInvalid {
        message: "No date found".to_string(),
    })?;
    let date = DateTime::<Utc>::from_str(date.as_str()).map_err(|e| RecurrenceInvalid {
        message: format!("Invalid date: {e}"),
    })?;
    Ok(Some(date))
}

fn extract_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
    let seconds = extract_seconds_duration(s).unwrap_or(Duration::seconds(0));
    let minutes = extract_minutes_duration(s).unwrap_or(Duration::minutes(0));
    let hours = extract_hours_duration(s).unwrap_or(Duration::hours(0));
    let days = extract_days_duration(s).unwrap_or(Duration::days(0));
    let weeks = extract_weeks_duration(s).unwrap_or(Duration::weeks(0));
    let duration = seconds + minutes + hours + days + weeks;
    Ok(duration)
}

fn extract_seconds_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
    use regex::Regex;
    let re = Regex::new(r"DURATION=PT(?P<seconds>[0-9]+)S").unwrap();
    let caps = re.captures(s).ok_or(RecurrenceInvalid {
        message: "No DURATION found".to_string(),
    })?;
    let seconds = parse_duration_pair(&caps, "seconds");
    let seconds = seconds.parse::<i64>().map_err(|e| RecurrenceInvalid {
        message: format!("Invalid seconds: {e}"),
    })?;
    let duration = Duration::seconds(seconds);
    Ok(duration)
}

fn extract_minutes_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
    use regex::Regex;
    let re = Regex::new(r"DURATION=PT(?P<minutes>[0-9]+)M").unwrap();
    let caps = re.captures(s).ok_or(RecurrenceInvalid {
        message: "No DURATION found".to_string(),
    })?;
    let minutes = parse_duration_pair(&caps, "minutes");
    let minutes = minutes.parse::<i64>().map_err(|e| RecurrenceInvalid {
        message: format!("Invalid minutes: {e}"),
    })?;
    let duration = Duration::minutes(minutes);
    Ok(duration)
}

fn extract_hours_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
    use regex::Regex;
    let re = Regex::new(r"DURATION=PT(?P<hours>[0-9]+)H").unwrap();
    let caps = re.captures(s).ok_or(RecurrenceInvalid {
        message: "No DURATION found".to_string(),
    })?;
    let hours = parse_duration_pair(&caps, "hours");
    let hours = hours.parse::<i64>().map_err(|e| RecurrenceInvalid {
        message: format!("Invalid hours: {e}"),
    })?;
    let duration = Duration::hours(hours);
    Ok(duration)
}

fn extract_days_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
    use regex::Regex;
    let re = Regex::new(r"DURATION=P(?P<days>[0-9]+)D").unwrap();
    let caps = re.captures(s).ok_or(RecurrenceInvalid {
        message: "No DURATION found".to_string(),
    })?;
    let days = parse_duration_pair(&caps, "days");
    let days = days.parse::<i64>().map_err(|e| RecurrenceInvalid {
        message: format!("Invalid days: {e}"),
    })?;
    let duration = Duration::days(days);
    Ok(duration)
}

fn extract_weeks_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
    use regex::Regex;
    let re = Regex::new(r"DURATION=P(?P<weeks>[0-9]+)W").unwrap();
    let caps = re.captures(s).ok_or(RecurrenceInvalid {
        message: "No DURATION found".to_string(),
    })?;
    let weeks = parse_duration_pair(&caps, "weeks");
    let weeks = weeks.parse::<i64>().map_err(|e| RecurrenceInvalid {
        message: format!("Invalid weeks: {e}"),
    })?;
    let duration = Duration::weeks(weeks);
    Ok(duration)
}

fn parse_duration_pair<'a>(caps: &'a regex::Captures, key: &'a str) -> &'a str {
    if let Some(d) = caps.name(key) {
        d.as_str()
    } else {
        ""
    }
}

#[cfg(test)]
mod test_helpers {
    use crate::recurrences::serializers::{extract_duration, extract_start_date};
    use chrono::{DateTime, Duration, Utc};
    use std::str::FromStr;

    #[test]
    fn test_extract_start() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=2020-01-01T00:00:00Z";
        let expected = DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap();
        let actual = extract_start_date(&value).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_start_no_dtstart() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10";
        let result = extract_start_date(&value);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_start_no_date() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=";
        let result = extract_start_date(&value);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_start_invalid_date() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=2020-01-01T00:00:00";
        let result = extract_start_date(&value);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_duration_no_duration() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=2020-01-01T00:00:00Z";
        let result = extract_duration(&value);
        assert!(result.is_ok());
        let duration = result.unwrap();
        assert_eq!(duration, Duration::seconds(0));
    }

    #[test]
    fn test_extract_duration_invalid_returns_0() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=2020-01-01T00:00:00Z;DURATION=PT1";
        let result = extract_duration(&value);
        assert!(result.is_ok());
        let duration = result.unwrap();
        assert_eq!(duration, Duration::seconds(0));
    }

    #[test]
    fn test_extract_seconds_duration() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=2020-01-01T00:00:00Z;DURATION=PT1S";
        let actual = extract_duration(&value).unwrap();
        let expected = Duration::seconds(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_minutes_duration() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=2020-01-01T00:00:00Z;DURATION=PT1M";
        let actual = extract_duration(&value).unwrap();
        let expected = Duration::minutes(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_hours_duration() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=2020-01-01T00:00:00Z;DURATION=PT1H";
        let actual = extract_duration(&value).unwrap();
        let expected = Duration::hours(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_days_duration() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=2020-01-01T00:00:00Z;DURATION=P1D";
        let actual = extract_duration(&value).unwrap();
        let expected = Duration::days(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_weeks_duration() {
        let value = "FREQ=SECONDLY;INTERVAL=1;COUNT=10;DTSTART=2020-01-01T00:00:00Z;DURATION=P1W";
        let actual = extract_duration(&value).unwrap();
        let expected = Duration::weeks(1);
        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod deserialize_tests {
    use crate::{Frequency, Recurrence};
    use chrono::{DateTime, Utc};
    use std::str::FromStr;

    #[test]
    fn secondly_recurrence_from_str() {
        let value = "FREQ=SECONDLY;INTERVAL=1;DTSTART=2020-01-01T00:00:00Z;DURATION=PT1S";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(2).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2020-01-01T00:00:01Z").unwrap(),
            ]
        );
    }

    #[test]
    fn minutely_recurrence_from_str() {
        let value = "FREQ=MINUTELY;INTERVAL=1;DTSTART=2020-01-01T00:00:00Z;DURATION=PT1M";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(2).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2020-01-01T00:01:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn hourly_recurrence_from_str() {
        let value = "FREQ=HOURLY;INTERVAL=1;DTSTART=2020-01-01T00:00:00Z;DURATION=PT1H";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(2).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2020-01-01T01:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn daily_recurrence_from_str() {
        let value = "FREQ=DAILY;INTERVAL=1;DTSTART=2020-01-01T00:00:00Z;DURATION=PT1D";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(2).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2020-01-02T00:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn weekly_recurrence_from_str() {
        let value = "FREQ=WEEKLY;INTERVAL=1;DTSTART=2020-01-01T00:00:00Z;DURATION=PT1W";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(2).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2020-01-08T00:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn monthly_recurrence_from_str() {
        let value = "FREQ=MONTHLY;INTERVAL=1;DTSTART=2020-01-01T00:00:00Z";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(2).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2020-02-01T00:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn yearly_recurrence_from_str() {
        let value = "FREQ=YEARLY;INTERVAL=1;DTSTART=2020-01-01T00:00:00Z";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(2).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2020-01-01T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2021-01-01T00:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn daily_by_time_from_str() {
        let value = "FREQ=DAILY;INTERVAL=1;DTSTART=2020-01-01T00:00:00Z;BYTIME=09:00,10:00";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(3).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2020-01-01T09:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2020-01-01T10:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2020-01-02T09:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn weekly_by_day() {
        let value = "FREQ=WEEKLY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z;BYDAY=MO,TU";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(3).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2023-01-03T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2023-01-09T00:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn monthly_by_month_day() {
        let value = "FREQ=MONTHLY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z;BYMONTHDAY=1,2";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(3).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2023-02-01T00:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn monthly_by_nth_weekday() {
        let value = "FREQ=MONTHLY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z;BYDAY=1MO";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(3).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2023-02-06T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2023-03-06T00:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn yearly_by_month_date() {
        let value =
            "FREQ=YEARLY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z;BYMONTH=1;BY;BYMONTHDAY=15";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(2).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2023-01-15T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2024-01-15T00:00:00Z").unwrap(),
            ]
        );
    }

    #[test]
    fn daily_with_dt_end() {
        let value = "FREQ=DAILY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z;DTEND=2023-01-03T00:00:00Z";
        let recurrence = Recurrence::from_str(value).unwrap();
        let events = recurrence.take(5).collect::<Vec<DateTime<Utc>>>();
        assert_eq!(
            events,
            vec![
                DateTime::<Utc>::from_str("2023-01-01T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2023-01-02T00:00:00Z").unwrap(),
                DateTime::<Utc>::from_str("2023-01-03T00:00:00Z").unwrap(),
            ]
        );
    }
}

#[cfg(test)]
mod serialize_tests {
    use std::str::FromStr;
    use crate::Recurrence;

    #[test]
    fn secondly_to_str() {
        let value = "FREQ=SECONDLY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z";
        let recurrence = Recurrence::from_str(value).unwrap();
        let serialized = recurrence.to_string();
        assert_eq!(serialized, value);
    }

    #[test]
    fn second_to_str_with_end_date() {
        let value = "FREQ=SECONDLY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z;DTEND=2023-01-02T00:00:00Z";
        let recurrence = Recurrence::from_str(value).unwrap();
        let serialized = recurrence.to_string();
        assert_eq!(serialized, value);
    }

    #[test]
    fn minutely_to_str() {
        let value = "FREQ=MINUTELY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z";
        let recurrence = Recurrence::from_str(value).unwrap();
        let serialized = recurrence.to_string();
        assert_eq!(serialized, value);
    }
}
