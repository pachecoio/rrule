use std::str::FromStr;
use chrono::{DateTime, Duration, Utc};
use crate::RecurrenceInvalid;

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
        Err(_) => return Ok(None),
    };
    let date = caps.name("date").ok_or(RecurrenceInvalid {
        message: "No date found".to_string(),
    })?;
    let date = DateTime::<Utc>::from_str(date.as_str()).map_err(|e| RecurrenceInvalid {
        message: format!("Invalid date: {e}"),
    })?;
    Ok(Some(date))
}

pub fn extract_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
    let seconds = extract_seconds_duration(s).unwrap_or(Duration::seconds(0));
    let minutes = extract_minutes_duration(s).unwrap_or(Duration::minutes(0));
    let hours = extract_hours_duration(s).unwrap_or(Duration::hours(0));
    let days = extract_days_duration(s).unwrap_or(Duration::days(0));
    let weeks = extract_weeks_duration(s).unwrap_or(Duration::weeks(0));
    let duration = seconds + minutes + hours + days + weeks;
    Ok(duration)
}

pub fn extract_seconds_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
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

pub fn extract_minutes_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
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

pub fn extract_hours_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
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

pub fn extract_days_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
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

pub fn extract_weeks_duration(s: &str) -> Result<Duration, RecurrenceInvalid> {
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
    use chrono::{DateTime, Duration, Utc};
    use std::str::FromStr;
    use crate::recurrences::serializers::helpers::{extract_duration, extract_start_date};

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
