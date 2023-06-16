# UTC Datetime
Simple, fast and small UTC dates, timestamps and datetimes library for Rust

## NOTE
Only capable of expressing times and dates SINCE the Unix Epoch `1970/01/01 00:00:00`. This library takes advantage of this assumption to simplify the API and internal logic.

## Features
- Create UTC timestamps and datetimes from `Duration`s, or directly from unsigned UTC sub-second measurements, or from the system time.
- Determine the civil calendar date.
- Determine the time of day.
- Determine the weekday.
- Determine the number of days since the Unix Epoch.
- Obtain information on a date or time, such as if it occurs within a leap year, or the number of days in the month.
- Format dates according to ISO 8601 (`YYYY-MM-DD`)
- Format datetimes according to ISO 8601 (`YYYY-MM-DDThh:mm:ssZ`)
- Provides constants useful for time transformations (`use utc-datetime::constants::*;`)
- Nanosecond resolution

## Examples (exhaustive)
 ```Rust
    use std::time::Duration;

    use utc_datetime::UTCDatetime;
    use utc_datetime::time::{
        UTCTimestamp,
        UTCDay,
    };
    use utc_datetime::date::UTCDate;

    // An example duration.
    // When a duration is used, it is assumed to be relative to the unix epoch.
    let example_duration = Duration::from_millis(1686824288903);

    // UTC Timestamp from a duration
    let utc_timestamp = UTCTimestamp::from(example_duration);
    // UTC timestamp from the local system time.
    let utc_timestamp = UTCTimestamp::try_from_system_time().unwrap();
    // UTC Timestamp from a u64 measurement directly.
    let utc_timestamp = UTCTimestamp::from_millis(1686824288903);
    // Use UTC Timestamp to get time-of-day
    let time_of_day_ns: u64 = utc_timestamp.to_time_of_day_ns();
    // Use UTC Timestamp to get days since epoch (ie. UTC Day)
    let utc_day: UTCDay = utc_timestamp.to_utc_day();

    // UTC Day from an integer
    let utc_day = UTCDay::from(19523);
    // Use UTC Day to get the weekday
    let weekday = utc_day.to_utc_weekday();

    // UTC Date directly from components
    let utc_date = UTCDate::try_from_components(2023, 6, 15).unwrap();
    // UTC Date from UTC Day
    let utc_date = UTCDate::from_utc_day(utc_day);
    // Check whether date occurs within leap year
    let is_leap_year: bool = utc_date.is_leap_year();
    // Get number of days within date's month
    let days_in_month: u8 = utc_date.days_in_month();
    // Get the date in integer forms
    let (year, month, day) = (utc_date.year(), utc_date.month(), utc_date.day()); // OR
    let (year, month, day) = utc_date.to_components();
    // UTC Day from UTC Date
    let utc_day = utc_date.to_utc_day();
    // Get date string formatted according to ISO 8601 (`YYYY-MM-DD`)
    let iso_date = utc_date.to_iso_date();
    assert_eq!(iso_date, "2023-06-15");

    // UTC Datetime directly from raw components
    let utc_datetime = UTCDatetime::try_from_raw_components(
        year,
        month,
        day,
        time_of_day_ns
    ).unwrap();
    // UTC Datetime from date and time-of-day components
    let utc_datetime = UTCDatetime::try_from_components(utc_date, time_of_day_ns).unwrap();
    // Get date and time-of-day components
    let (utc_date, time_of_day_ns) = (utc_datetime.to_date(), utc_datetime.to_time_of_day_ns());
    let (utc_date, time_of_day_ns) = utc_datetime.to_components();
    // Get the time in hours, minutes and seconds
    let (hours, minutes, seconds) = utc_datetime.to_hours_minutes_seconds();
    // Get the sub-second component of the time of day, in nanoseconds
    let subsec_ns = utc_datetime.to_subsec_ns();
    // Get UTC datetime string formatted according to ISO 8601 (`YYYY-MM-DDThh:mm:ssZ`)
    let iso_datetime = utc_datetime.to_iso_datetime();
    assert_eq!(iso_datetime, "2023-06-15T10:18:08Z");

    {
        // `UTCTransformations` can be used to create shortcuts to the desired type!
        use utc_datetime::time::UTCTransformations;

        // Example shortcuts using `UTCTransformations`
        // UTC Day / UTC Date / UTC Datetime from a duration
        let utc_day = UTCDay::from_utc_duration(example_duration); // OR
        let utc_day = UTCDay::from(example_duration);
        let utc_date = UTCDate::from_utc_duration(example_duration); // OR
        let utc_date = UTCDate::from(example_duration);
        let utc_datetime = UTCDatetime::from_utc_duration(example_duration); // OR
        let utc_datetime = UTCDatetime::from(example_duration);

        // UTC Day / UTC Date / UTC Datetime from a timestamp
        let utc_day = UTCDay::from_utc_timestamp(utc_timestamp); // OR
        let utc_day = UTCDay::from(utc_timestamp);
        let utc_date = UTCDate::from_utc_timestamp(utc_timestamp); // OR
        let utc_date = UTCDate::from(utc_timestamp);
        let utc_datetime = UTCDatetime::from_utc_timestamp(utc_timestamp); // OR
        let utc_datetime = UTCDatetime::from(utc_timestamp);

        // UTC Day / UTC Date / UTC Datetime from local system time
        let utc_day = UTCDay::try_from_system_time().unwrap();
        let utc_date = UTCDate::try_from_system_time().unwrap();
        let utc_datetime = UTCDatetime::try_from_system_time().unwrap();

        // UTC Day / UTC Date / UTC Datetime from u64 epoch measurements
        let utc_day = UTCDay::from_utc_secs(1686824288);
        let utc_date = UTCDate::from_utc_millis(1686824288_000);
        let utc_datetime = UTCDate::from_utc_micros(1686824288_000_000);
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