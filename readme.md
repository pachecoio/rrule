# RRule

A library to manage recurrence rules following the standards from [RFC-5547](https://icalendar.org/iCalendar-RFC-5545/3-3-10-recurrence-rule.html).


# How to use it

To define a recurrence rule, start by creating the desired `frequency` rule definition:

```rust
let daily = Frequency::Daily {
    interval: 1,
    by_time: vec![],
};
```

Then, create a `Recurrence` with the frequency defined:

```rust
...
let recurrence = Recurrence::new(
    daily,
    Utc::now(), // start date
    Some(Utc::now() + Duration::days(1)), // end date
    Some(Duration::hours(1)), // duration (optional
);
```

> The `Recurrence` struct is an iterator that will yield all the dates that match the recurrence rules defined.

You can then use the `Recurrence` as an iterator by looping over it or collecting the results into a `Vec`

```rust
...
for event in recurrence {
    ...
}
# or
let events: Vec<DateTime<Utc>> = recurrence.collect();
```

The `end` attribute of a `Recurrence` is optional, and if not specified, it will yield events until the `MAX_DATE`.
> The `MAX_DATE` is defined as `9999-12-31T23:59:59Z`


The `duration` attribute of a `Recurrence` is optional, and if not specified, it will use the default as 1 hour `Duration::hours(1)`.

## Supported frequencies
Current supporting recurrence rules:

- [Secondly](#secondly)
- [minutely](#minutely)
- [hourly](#hourly)
- [Daily](#daily)
- [Weekly](#weekly)
- [Monthly](#monthly)
    - [By month day](#monthly-by-month-day)
    - [By nth weekday](#monthly-by-day)
- [Yearly](#yearly)
    - [By day](#yearly-by-day)
    - [By month day](#yearly-by-month-day)


<span id="secondly"></span>
### Secondly Frequencies
Represents the rules for a recurrence that happens every x seconds.

```rust
use rrule::Frequency;

let every_second = Frequency::Secondly {
    interval: 1,
};

let every_5_seconds = Frequency::Secondly {
    interval: 5,
};
```

<span id="minutely"></span>
### Minutely Frequencies
Represents the rules for a recurrence that happens every x minutes.

```rust
use rrule::Frequency;

let every_minute = Frequency::Minutely {
    interval: 1,
};

let every_5_minutes = Frequency::Minutely {
    interval: 5,
};
```

<span id="hourly"></span>
### Hourly Frequencies
Represents the rules for a recurrence that happens every x hours.

```rust
use rrule::Frequency;

let every_hour = Frequency::Hourly {
    interval: 1,
};

let every_6_hours = Frequency::Hourly {
    interval: 6,
};
```

<span id="daily"></span>
### Daily Frequencies
Represents the rules for a recurrence that happens x times every x days.

```rust
use chrono::{DateTime, Duration, Utc};
use rrule::{Frequency, Time};

let daily = Frequency::Daily {
    interval: 1,
    by_time: vec![],
};

let every_3_days = Frequency::Daily {
    interval: 3,
    by_time: vec![],
};

let every_day_at_8am = Frequency::Daily {
    interval: 1,
    by_time: vec![
        Time::from_str("08:00:00").unwrap(),
    ]
};
```

<span id="weekly"></span>
### Weekly Frequencies
Represents the rules for a recurrence that happens x times every x weeks.

```rust
let weekly = Frequency::Weekly {
    interval: 1,
    by_day: vec![],
};

let twice_a_week = Frequency::Weekly {
    interval: 1,
    by_day: vec![Weekday::Mon, Weekday::Tue],
};
```
<span id="monthly"></span>
### Monthly Frequencies
Represents the rules for a recurrence that happens x times every x months.

```rust
let monthly = Frequency::Monthly {
    interval: 1,
    by_month_day: vec![],
    nth_weekdays: vec![],
};
```

<span id="monthly-by-month-day"></span>
#### Monthly by month day

When specifying `by_month_day`, it will only yield the dates that match the days of the month specified.

```rust
let every_15th = Frequency::Monthly {
    interval: 1,
    by_month_day: vec![15],
    nth_weekdays: vec![],
};
```

<span id="monthly-by-day"></span>
#### Monthly by nth day

When specifying `nth_weekdays`, it will only yield the dates that match the nth days of the week specified.
I.g. if you want to have a recurrence every first Monday of the month, you can do:

```rust
let every_first_monday = Frequency::Monthly {
    interval: 1,
    by_month_day: vec![],
    nth_weekdays: vec![
        NthWeekday::new(Weekday::Mon, 1),
    ]
};
```

<span id="yearly"></span>
### Yearly Frequencies
Represents the rules for a recurrence that happens x times every x years.

```rust
let yearly = Frequency::Yearly {
    interval: 1,
    by_month_date: vec![],
};
```

<span id="yearly-by-month-day"></span>
#### Yearly by month day
    
When specifying `by_month_date`, it will only yield the dates that match the days of the month specified.
E.g. if you want to have a recurrence every 15th January of the year, you can do:
    
```rust
let every_15th_january = Frequency::Yearly {
    interval: 1,
    by_month_date: vec![
        MonthlyDate::new(Month::January, 15),
    ]
};
```
