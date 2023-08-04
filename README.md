# UTC Datetime

[![crates.io](https://img.shields.io/crates/v/utc-dt?style=flat-square&logo=rust)](https://crates.io/crates/utc-dt)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![build status](https://img.shields.io/github/actions/workflow/status/uniciant/utc-datetime/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/uniciant/utc-datetime/actions)

Simple, fast and small UTC date, timestamp and datetime library for Rust.

UTC Datetime aims to be ergonomic and user friendly, focused on core features.
It prioritizes being space-optimal and efficient.

```toml
[dependencies]
utc-dt = "0.1"
```
For extended/niche features and local time-zone support see [`chrono`](https://github.com/chronotope/chrono) or [`time`](https://github.com/time-rs/time).

### `unsigned` only!
UTC Datetime will only express times and dates SINCE the Unix Epoch `(1970-01-01T00:00:00Z)`.
The library takes advantage of this assumption to simplify the API and internal logic.

## Documentation
See [docs.rs](https://docs.rs/utc-dt) for the API reference.

## Features
- Create UTC timestamps and datetimes from `Duration`s, or directly from unsigned UTC sub-second measurements, or from the system time.
- Determine the civil calendar date, time of day, weekday or the number of days since the Unix Epoch.
- Obtain information on a date or time, such as if it occurs within a leap year, or the number of days in the month.
- Convert between time representations efficiently and ergonomically.
- Compile-time const evaluation wherever possible.
- Format and parse dates according to ISO 8601 `(YYYY-MM-DD)`
- Format and parse datetimes according to ISO 8601 `(YYYY-MM-DDThh:mm:ssZ)`
- Provides constants useful for time transformations: [`utc-dt::constants`](https://docs.rs/utc-dt/latest/utc_dt/constants/index.html)
- Nanosecond resolution.
- `#![no_std]` support.

## Examples (exhaustive)
 ```rust
    use core::time::Duration;

    use utc_dt::UTCDatetime;
    use utc_dt::time::{
        UTCTimestamp,
        UTCDay,
        UTCTimeOfDay,
    };
    use utc_dt::date::UTCDate;

    // An example duration.
    // When a duration is used, it is assumed to be relative to the unix epoch.
    // Thursday, 15 June 2023 10:18:08.903
    let example_duration = Duration::from_millis(1686824288903);

    // UTC Timestamp from a duration
    let utc_timestamp = UTCTimestamp::from(example_duration); // OR
    let utc_timestamp = UTCTimestamp::from_duration(example_duration);
    // UTC timestamp from the local system time.
    // Not available for #![no_std]
    let utc_timestamp = UTCTimestamp::try_from_system_time().unwrap();
    // UTC Timestamp from a time measurement (for secs, millis, micros, nanos)
    let utc_timestamp = UTCTimestamp::from_millis(1686824288903);
    // Use UTC Timestamp to get a time measurement since the epoch (for secs, millis, micros, nanos)
    let utc_millis = utc_timestamp.as_millis();
    // Use UTC Timestamp to get time-of-day
    let utc_tod: UTCTimeOfDay = utc_timestamp.as_tod();
    // Use UTC Timestamp to get days since epoch (ie. UTC Day)
    let utc_day: UTCDay = utc_timestamp.as_day();
    // UTC Timestamp from UTC Day and time-of-day components
    let utc_timestamp = UTCTimestamp::from_day_and_tod(utc_day, utc_tod);

    // UTC Day from an integer
    let utc_day = UTCDay::from(19523); // OR
    let utc_day = UTCDay::from_u32(19523);
    // Use UTC Day to get the weekday
    let weekday = utc_day.as_weekday();

    // UTC Time of Day from a time measurement (for secs, millis, micros, nanos)
    let utc_tod = UTCTimeOfDay::try_from_millis(37088903).unwrap(); // OR
    let utc_tod = unsafe { UTCTimeOfDay::from_millis_unchecked(37088903) };
    // UTC Time of Day from hours, minutes, seconds and subseconds
    let utc_tod = UTCTimeOfDay::try_from_hhmmss(10, 18, 08, 903_000_000).unwrap(); // OR
    let utc_tod = unsafe { UTCTimeOfDay::from_hhmmss_unchecked(10, 18, 08, 903_000_000) };
    // UTC Time of Day as a time measurement (for secs, millis, micros, nanos)
    let utc_tod_us = utc_tod.as_micros();
    // UTC Time of Day as hours, minutes and seconds
    let (hrs, mins, secs) = utc_tod.as_hhmmss();
    // UTC Time of Day subsecond component (in nanoseconds)
    let subsec_ns = utc_tod.as_subsec_ns();
    // Parse a UTC Time of Day from an ISO 8601 time string `(Thh:mm:ssZ)`
    let utc_tod = UTCTimeOfDay::try_from_iso_tod("T10:18:08.903Z").unwrap();
    // Get a time of day string formatted according to ISO 8601 `(Thh:mm:ssZ)`
    // Not available for #![no_std]
    let precision = Some(6);
    let iso_tod = utc_tod.as_iso_tod(precision);
    assert_eq!(iso_tod, "T10:18:08.903000Z");

    // UTC Date directly from components
    let utc_date = UTCDate::try_from_components(2023, 6, 15).unwrap(); // OR
    let utc_date = unsafe { UTCDate::from_components_unchecked(2023, 6, 15) };
    // UTC Date from UTC Day
    let utc_date = UTCDate::from_day(utc_day);
    // Check whether date occurs within leap year
    let is_leap_year: bool = utc_date.is_leap_year();
    // Get number of days within date's month
    let days_in_month: u8 = utc_date.days_in_month();
    // Get the date in integer forms
    let (year, month, day) = utc_date.as_components();
    // UTC Day from UTC Date
    let utc_day = utc_date.as_day();
    // Parse a UTC Date from an ISO 8601 date string `(YYYY-MM-DD)`
    let utc_date = UTCDate::try_from_iso_date("2023-06-15").unwrap();
    // Get date string formatted according to ISO 8601 `(YYYY-MM-DD)`
    // Not available for #![no_std]
    let iso_date = utc_date.as_iso_date();
    assert_eq!(iso_date, "2023-06-15");

    // UTC Datetime from date and time-of-day components
    let utc_datetime = UTCDatetime::from_components(utc_date, utc_tod);
    // Get date and time-of-day components
    let (utc_date, time_of_day_ns) = (utc_datetime.as_date(), utc_datetime.as_tod()); // OR
    let (utc_date, time_of_day_ns) = utc_datetime.as_components();
    // Parse a UTC Datetime from an ISO 8601 datetime string `(YYYY-MM-DDThh:mm:ssZ)`
    let utc_datetime = UTCDatetime::try_from_iso_datetime("2023-06-15T10:18:08.903Z").unwrap();
    // Get UTC datetime string formatted according to ISO 8601 `(YYYY-MM-DDThh:mm:ssZ)`
    // Not available for #![no_std]
    let precision = None;
    let iso_datetime = utc_datetime.as_iso_datetime(precision);
    assert_eq!(iso_datetime, "2023-06-15T10:18:08Z");

    {
        // `UTCTransformations` can be used to create shortcuts to the desired type!
        use utc_dt::time::UTCTransformations;

        // Example shortcuts using `UTCTransformations`
        // UTC Day / UTC Date / UTC Datetime from a duration
        let utc_day = UTCDay::from_duration(example_duration); // OR
        let utc_day = UTCDay::from(example_duration);
        let utc_date = UTCDate::from_duration(example_duration); // OR
        let utc_date = UTCDate::from(example_duration);
        let utc_datetime = UTCDatetime::from_duration(example_duration); // OR
        let utc_datetime = UTCDatetime::from(example_duration);

        // UTC Day / UTC Date / UTC Datetime from a timestamp
        let utc_day = UTCDay::from_timestamp(utc_timestamp); // OR
        let utc_day = UTCDay::from(utc_timestamp);
        let utc_date = UTCDate::from_timestamp(utc_timestamp); // OR
        let utc_date = UTCDate::from(utc_timestamp);
        let utc_datetime = UTCDatetime::from_timestamp(utc_timestamp); // OR
        let utc_datetime = UTCDatetime::from(utc_timestamp);

        // UTC Day / UTC Date / UTC Datetime from local system time
        // Not available for #![no_std]
        let utc_day = UTCDay::try_from_system_time().unwrap();
        let utc_date = UTCDate::try_from_system_time().unwrap();
        let utc_datetime = UTCDatetime::try_from_system_time().unwrap();

        // UTC Day / UTC Date / UTC Datetime from u64 epoch measurements
        let utc_day = UTCDay::from_secs(1686824288);
        let utc_date = UTCDate::from_millis(1686824288_000);
        let utc_datetime = UTCDate::from_micros(1686824288_000_000);

        // Convert from UTC Day / UTC Date / UTC Datetime back to various types
        let utc_duration: Duration = utc_day.as_duration();
        let utc_timestamp: UTCTimestamp = utc_date.as_timestamp();
        let utc_secs: u64 = utc_date.as_secs();
        let utc_millis: u64 = utc_datetime.as_millis();
        let utc_micros: u64 = utc_day.as_micros();
        let utc_nanos: u64 = utc_date.as_nanos();
    }
```

## References
- [(Howard Hinnant, 2021) `chrono`-Compatible Low-Level Date Algorithms](http://howardhinnant.github.io/date_algorithms.html)
- [(W3C, 1997) ISO 8601 Standard for Date and Time Formats](https://www.w3.org/TR/NOTE-datetime)

## License
This project is licensed under either of
* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
* [MIT License](https://opensource.org/licenses/MIT)
at your option.