# RRules

A blazing fast and memory efficient library to manage recurrence rules inspired by the standards from [RFC-5547](https://icalendar.org/iCalendar-RFC-5545/3-3-10-recurrence-rule.html).
It provides the ability to define recurrence rules for events, and then iterate over them to get the dates that match the recurrence rules.

## How to use it

The easiest way to use this library is to start by loading a Recurrence instance from a string following the [standards]():

### Loading from string

#### Required attributes:

- FREQ
  - Defines the type of frequency (E.g. DAILY, WEEKLY, MONTHLY, etc)
- INTERVAL
    - Defines the interval of the frequency (E.g. every 2 days, every 3 months, etc)
- DTSTART
    - Defines the start date of the recurrence

Examples:

```rust
// Daily recurrence example:

let recurrence = Recurrence::from_str("FREQ=DAILY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z"").unwrap();

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

### How to use it with structs definition

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

The `end` attribute of a `Recurrence` is optional, and if not specified, it will yield events until the `MAX_DATE`.
> The `MAX_DATE` is defined as `9999-12-31T23:59:59Z`

The `duration` attribute of a `Recurrence` is optional, and if not specified, it will use the default as 1 hour `Duration::seconds(0)`.

## Attribute standards

| Attribute  | Description                                                                                                    | Example                                                             |
|------------|----------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------|
| FREQ       | Defines the type of frequency (E.g. DAILY, WEEKLY, MONTHLY, etc)                                               | FREQ=DAILY                                                          |
| INTERVAL   | Defines the interval of the frequency (E.g. every 2 days, every 3 months, etc)                                 | INTERVAL=2                                                          |
| DTSTART    | Defines the start date of the recurrence                                                                       | DTSTART=2023-01-01T12:00:00Z                                        |
| DTEND      | Defines the end date of the recurrence                                                                         | DTEND=2023-01-01T12:00:00Z                                          |
| DURATION   | Defines the duration of the recurrence                                                                         | DURATION=PT1H                                                       |
| BYDAY      | Defines the days of the week that the recurrence will happen                                                   | BYDAY=MO,TU -> When FREQ=WEEKLY; BYDAY=1MO,3WE -> When FREQ=MONTHLY |
| BYMONTHDAY | Defines the days of the month that the recurrence will happen                                                  | BYMONTHDAY=1,2,3,4, etc                                             |
| BYMONTH    | Defines the months of the year that the recurrence will happen                                                 | BYMONTH=1,2,3,4,5,6,7,8,9,10,11,12                                  |


## Supported recurrence rule types + examples
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
let every_second_recurrence = Recurrence::from_str(
    "FREQ=SECONDLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z"
).unwrap();
```

<span id="minutely"></span>
### Minutely Frequencies
Represents the rules for a recurrence that happens every x minutes.

```rust
let every_5_minutes = Recurrence::from_str(
    "FREQ=MINUTELY;INTERVAL=5;DTSTART=2023-01-01T12:00:00Z"
).unwrap();
```

<span id="hourly"></span>
### Hourly Frequencies
Represents the rules for a recurrence that happens every x hours.

```rust
let every_6_hours = Recurrence::from_str(
    "FREQ=HOURLY;INTERVAL=6;DTSTART=2023-01-01T12:00:00Z"
).unwrap();
```

<span id="daily"></span>
### Daily Frequencies
Represents the rules for a recurrence that happens x times every x days.

```rust

let every_3_days = Recurrence::from_str(
    "FREQ=DAILY;INTERVAL=3;DTSTART=2023-01-01T12:00:00Z"
).unwrap();

let every_day_at_8am = Recurrence::from_str(
    "FREQ=DAILY;INTERVAL=1;DTSTART=2023-01-01T08:00:00Z"
).unwrap();

let every_other_day_at_12pm_and_16pm = Recurrence::from_str(
    "FREQ=DAILY;INTERVAL=2;DTSTART=2023-01-01T00:00:00Z;BYTIME=12:00,16:00"
).unwrap();
```

<span id="weekly"></span>
### Weekly Frequencies
Represents the rules for a recurrence that happens x times every x weeks.

```rust
let every_week = Recurrence::from_str(
    "FREQ=WEEKLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z"
).unwrap();

let every_week_mon_and_tue = Recurrence::from_str(
    "FREQ=WEEKLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z;BYDAY=MO,TU"
).unwrap();
```
<span id="monthly"></span>
### Monthly Frequencies
Represents the rules for a recurrence that happens x times every x months.

```rust
let monthly = Recurrence::from_str(
    "FREQ=MONTHLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z"
).unwrap();
```

<span id="monthly-by-month-day"></span>
#### Monthly by month day

When specifying `BYMONTHDAY`, it will only yield the dates that match the days of the month specified.

```rust
let every_15th = Recurrence::from_str(
    "FREQ=MONTHLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z;BYMONTHDAY=15"
).unwrap();

let every_15th_and_30th = Recurrence::from_str(
    "FREQ=MONTHLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z;BYMONTHDAY=15,30"
).unwrap();
```

<span id="monthly-by-day"></span>
#### Monthly by nth day

When specifying `BYDAY`, it will only yield the dates that match the nth days of the week specified.
I.g. if you want to have a recurrence every first Monday of the month, you can do:

```rust
let every_first_monday = Recurrence::from_str(
    "FREQ=MONTHLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z;BYDAY=1MO"
).unwrap();

let every_first_monday_and_wednesday = Recurrence::from_str(
    "FREQ=MONTHLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z;BYDAY=1MO,1WE"
).unwrap();
```

<span id="yearly"></span>
### Yearly Frequencies
Represents the rules for a recurrence that happens x times every x years.

```rust
let yearly = Recurrence::from_str(
    "FREQ=YEARLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z"
).unwrap();
```

<span id="yearly-by-month-day"></span>
#### Yearly by month day
    
When specifying `BYMONTH` and `BYMONTHDAY`, it will only yield the dates that match the days of the month specified.
E.g. if you want to have a recurrence every 15th January of the year, you can do:
    
```rust
let every_15th_january = Recurrence::from_str(
    "FREQ=YEARLY;INTERVAL=1;DTSTART=2023-01-01T12:00:00Z;BYMONTH=1;BYMONTHDAY=15"
).unwrap();
```