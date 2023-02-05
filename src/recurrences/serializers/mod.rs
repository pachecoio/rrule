mod helpers;

use crate::recurrences::serializers::helpers::{
    extract_duration, extract_end_date, extract_start_date,
};
use crate::recurrences::MAX_DATE;
use crate::{Frequency, Recurrence, RecurrenceInvalid};
use chrono::{DateTime, Duration, Utc};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

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
        write!(f, "{res}")
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
    use crate::Recurrence;
    use std::str::FromStr;

    #[test]
    fn secondly_to_str() {
        let value = "FREQ=SECONDLY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z";
        let recurrence = Recurrence::from_str(value).unwrap();
        let serialized = recurrence.to_string();
        assert_eq!(serialized, value);
    }

    #[test]
    fn second_to_str_with_end_date() {
        let value =
            "FREQ=SECONDLY;INTERVAL=1;DTSTART=2023-01-01T00:00:00Z;DTEND=2023-01-02T00:00:00Z";
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
