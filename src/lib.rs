//! # UTC Datetime
//! Simple, fast and small UTC date, timestamp and datetime library for Rust.
//!
//! UTC Datetime aims to be ergonomic and user friendly, focused on core features.
//! It prioritizes being space-optimal and efficient.
//!
//! ```rust,ignore
//! [dependencies]
//! utc-dt = "0.1"
//! ```
//! For extended/niche features and local time-zone support see [`chrono`](https://github.com/chronotope/chrono) or [`time`](https://github.com/time-rs/time).
//!
//! ### NOTE: `unsigned` only!
//! UTC Datetime will only express times and dates SINCE the Unix Epoch `(1970-01-01T00:00:00Z)`.
//! The library takes advantage of this assumption to simplify the API and internal logic.
//!
//! ## Documentation
//! See [docs.rs](https://docs.rs/utc-dt) for the API reference.
//!
//! ## Features
//! - Create UTC timestamps and datetimes from `Duration`s, or directly from unsigned UTC sub-second measurements, or from the system time.
//! - Determine the civil calendar date, time of day, weekday or the number of days since the Unix Epoch.
//! - Obtain information on a date or time, such as if it occurs within a leap year, or the number of days in the month.
//! - Convert between time representations efficiently and ergonomically.
//! - Compile-time const evaluation wherever possible.
//! - Format and parse dates according to ISO 8601 `(YYYY-MM-DD)`
//! - Format and parse datetimes according to ISO 8601 `(YYYY-MM-DDThh:mm:ssZ)`
//! - Provides constants useful for time transformations: [`utc-dt::constants`](https://docs.rs/utc-dt/latest/utc_dt/constants/index.html)
//! - Nanosecond resolution.
//! - `#![no_std]` support.
//!
//! ## Examples (exhaustive)
#![cfg_attr(not(feature = "std"), doc = "```rust,ignore")]
#![cfg_attr(feature = "std", doc = "```rust")]
//!     use core::time::Duration;
//!
//!     use utc_dt::UTCDatetime;
//!     use utc_dt::time::{
//!         UTCTimestamp,
//!         UTCDay,
//!         UTCTimeOfDay,
//!     };
//!     use utc_dt::date::UTCDate;
//!
//!     // An example duration.
//!     // When a duration is used, it is assumed to be relative to the unix epoch.
//!     // Thursday, 15 June 2023 10:18:08.903
//!     let example_duration = Duration::from_millis(1686824288903);
//!
//!     // UTC Timestamp from a duration
//!     let utc_timestamp = UTCTimestamp::from(example_duration); // OR
//!     let utc_timestamp = UTCTimestamp::from_duration(example_duration);
//!     // UTC timestamp from the local system time.
//!     // Not available for #![no_std]
//!     let utc_timestamp = UTCTimestamp::try_from_system_time().unwrap();
//!     // UTC Timestamp from a time measurement (for secs, millis, micros, nanos)
//!     let utc_timestamp = UTCTimestamp::from_millis(1686824288903);
//!     // Use UTC Timestamp to get a time measurement since the epoch (for secs, millis, micros, nanos)
//!     let utc_millis = utc_timestamp.as_millis();
//!     // Use UTC Timestamp to get time-of-day
//!     let utc_tod: UTCTimeOfDay = utc_timestamp.as_tod();
//!     // Use UTC Timestamp to get days since epoch (ie. UTC Day)
//!     let utc_day: UTCDay = utc_timestamp.as_day();
//!     // UTC Timestamp from UTC Day and time-of-day components
//!     let utc_timestamp = UTCTimestamp::from_day_and_tod(utc_day, utc_tod);
//!
//!     // UTC Day from an integer
//!     let utc_day = UTCDay::from(19523); // OR
//!     let utc_day = UTCDay::from_u32(19523);
//!     // Use UTC Day to get the weekday
//!     let weekday = utc_day.as_weekday();
//!
//!     // UTC Time of Day from a time measurement (for secs, millis, micros, nanos)
//!     let utc_tod = UTCTimeOfDay::try_from_millis(37088903).unwrap(); // OR
//!     let utc_tod = unsafe { UTCTimeOfDay::from_millis_unchecked(37088903) };
//!     // UTC Time of Day from hours, minutes, seconds and subseconds
//!     let utc_tod = UTCTimeOfDay::try_from_hhmmss(10, 18, 08, 903_000_000).unwrap(); // OR
//!     let utc_tod = unsafe { UTCTimeOfDay::from_hhmmss_unchecked(10, 18, 08, 903_000_000) };
//!     // UTC Time of Day as a time measurement (for secs, millis, micros, nanos)
//!     let utc_tod_us = utc_tod.as_micros();
//!     // UTC Time of Day as hours, minutes and seconds
//!     let (hrs, mins, secs) = utc_tod.as_hhmmss();
//!     // UTC Time of Day subsecond component (in nanoseconds)
//!     let subsec_ns = utc_tod.as_subsec_ns();
//!     // Parse a UTC Time of Day from an ISO 8601 time string `(Thh:mm:ssZ)`
//!     let utc_tod = UTCTimeOfDay::try_from_iso_tod("T10:18:08.903Z").unwrap();
//!     // Get a time of day string formatted according to ISO 8601 `(Thh:mm:ssZ)`
//!     // Not available for #![no_std]
//!     let precision = Some(6);
//!     let iso_tod = utc_tod.as_iso_tod(precision);
//!     assert_eq!(iso_tod, "T10:18:08.903000Z");
//!
//!     // UTC Date directly from components
//!     let utc_date = UTCDate::try_from_components(2023, 6, 15).unwrap(); // OR
//!     let utc_date = unsafe { UTCDate::from_components_unchecked(2023, 6, 15) };
//!     // UTC Date from UTC Day
//!     let utc_date = UTCDate::from_day(utc_day);
//!     // Check whether date occurs within leap year
//!     let is_leap_year: bool = utc_date.is_leap_year();
//!     // Get number of days within date's month
//!     let days_in_month: u8 = utc_date.days_in_month();
//!     // Get the date in integer forms
//!     let (year, month, day) = utc_date.as_components();
//!     // UTC Day from UTC Date
//!     let utc_day = utc_date.as_day();
//!     // Parse a UTC Date from an ISO 8601 date string `(YYYY-MM-DD)`
//!     let utc_date = UTCDate::try_from_iso_date("2023-06-15").unwrap();
//!     // Get date string formatted according to ISO 8601 `(YYYY-MM-DD)`
//!     // Not available for #![no_std]
//!     let iso_date = utc_date.as_iso_date();
//!     assert_eq!(iso_date, "2023-06-15");
//!
//!     // UTC Datetime from date and time-of-day components
//!     let utc_datetime = UTCDatetime::from_components(utc_date, utc_tod);
//!     // Get date and time-of-day components
//!     let (utc_date, time_of_day_ns) = (utc_datetime.as_date(), utc_datetime.as_tod()); // OR
//!     let (utc_date, time_of_day_ns) = utc_datetime.as_components();
//!     // Parse a UTC Datetime from an ISO 8601 datetime string `(YYYY-MM-DDThh:mm:ssZ)`
//!     let utc_datetime = UTCDatetime::try_from_iso_datetime("2023-06-15T10:18:08.903Z").unwrap();
//!     // Get UTC datetime string formatted according to ISO 8601 `(YYYY-MM-DDThh:mm:ssZ)`
//!     // Not available for #![no_std]
//!     let precision = None;
//!     let iso_datetime = utc_datetime.as_iso_datetime(precision);
//!     assert_eq!(iso_datetime, "2023-06-15T10:18:08Z");
//!
//!     {
//!         // `UTCTransformations` can be used to create shortcuts to the desired type!
//!         use utc_dt::time::UTCTransformations;
//!
//!         // Example shortcuts using `UTCTransformations`
//!         // UTC Day / UTC Date / UTC Datetime from a duration
//!         let utc_day = UTCDay::from_duration(example_duration); // OR
//!         let utc_day = UTCDay::from(example_duration);
//!         let utc_date = UTCDate::from_duration(example_duration); // OR
//!         let utc_date = UTCDate::from(example_duration);
//!         let utc_datetime = UTCDatetime::from_duration(example_duration); // OR
//!         let utc_datetime = UTCDatetime::from(example_duration);
//!
//!         // UTC Day / UTC Date / UTC Datetime from a timestamp
//!         let utc_day = UTCDay::from_timestamp(utc_timestamp); // OR
//!         let utc_day = UTCDay::from(utc_timestamp);
//!         let utc_date = UTCDate::from_timestamp(utc_timestamp); // OR
//!         let utc_date = UTCDate::from(utc_timestamp);
//!         let utc_datetime = UTCDatetime::from_timestamp(utc_timestamp); // OR
//!         let utc_datetime = UTCDatetime::from(utc_timestamp);
//!
//!         // UTC Day / UTC Date / UTC Datetime from local system time
//!         // Not available for #![no_std]
//!         let utc_day = UTCDay::try_from_system_time().unwrap();
//!         let utc_date = UTCDate::try_from_system_time().unwrap();
//!         let utc_datetime = UTCDatetime::try_from_system_time().unwrap();
//!
//!         // UTC Day / UTC Date / UTC Datetime from u64 epoch measurements
//!         let utc_day = UTCDay::from_secs(1686824288);
//!         let utc_date = UTCDate::from_millis(1686824288_000);
//!         let utc_datetime = UTCDate::from_micros(1686824288_000_000);
//!
//!         // Convert from UTC Day / UTC Date / UTC Datetime back to various types
//!         let utc_duration: Duration = utc_day.as_duration();
//!         let utc_timestamp: UTCTimestamp = utc_date.as_timestamp();
//!         let utc_secs: u64 = utc_date.as_secs();
//!         let utc_millis: u64 = utc_datetime.as_millis();
//!         let utc_micros: u64 = utc_day.as_micros();
//!         let utc_nanos: u64 = utc_date.as_nanos();
//!     }
//! ```
//!
//! ## References
//! - [(Howard Hinnant, 2021) `chrono`-Compatible Low-Level Date Algorithms](http://howardhinnant.github.io/date_algorithms.html)
//! - [(W3C, 1997) ISO 8601 Standard for Date and Time Formats](https://www.w3.org/TR/NOTE-datetime)
//!
//! ## License
//! This project is licensed under either of
//! * [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
//! * [MIT License](https://opensource.org/licenses/MIT)
//! at your option.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod date;
pub mod time;
#[rustfmt::skip]
pub mod constants;

use core::time::Duration;

use anyhow::Result;

use date::UTCDate;
use time::{UTCTimestamp, UTCTransformations, UTCTimeOfDay};

/// UTC Datetime.
///
/// A UTC Datetime consists of a date component and a time-of-day component
/// with nanosecond resolution.
///
/// ## Examples
/// ```rust,ignore
/// // UTC Datetime from date and time-of-day components
/// let utc_datetime = UTCDatetime::from_components(utc_date, utc_tod);
/// // Get date and time-of-day components
/// let (utc_date, time_of_day_ns) = (utc_datetime.as_date(), utc_datetime.as_tod()); // OR
/// let (utc_date, time_of_day_ns) = utc_datetime.as_components();
/// // Parse a UTC Datetime from an ISO 8601 datetime string `(YYYY-MM-DDThh:mm:ssZ)`
/// let utc_datetime = UTCDatetime::try_from_iso_datetime("2023-06-15T10:18:08.903Z").unwrap();
/// // Get UTC datetime string formatted according to ISO 8601 `(YYYY-MM-DDThh:mm:ssZ)`
/// // Not available for #![no_std]
/// let precision = None;
/// let iso_datetime = utc_datetime.as_iso_datetime(precision);
/// assert_eq!(iso_datetime, "2023-06-15T10:18:08Z");
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UTCDatetime {
    date: UTCDate,
    tod: UTCTimeOfDay,
}

impl UTCDatetime {
    /// Create a datetime frome date and time-of-day components.
    #[inline]
    pub const fn from_components(date: UTCDate, tod: UTCTimeOfDay) -> Self {
        Self {
            date,
            tod,
        }
    }

    /// Get copy of the internal date and time-of-day components
    ///
    /// Returns tuple: `(date: UTCDate, tod: UTCTimeOfDay)`
    #[inline]
    pub const fn as_components(&self) -> (UTCDate, UTCTimeOfDay) {
        (self.date, self.tod)
    }

    /// Consume self into the internal date and time-of-day components
    ///
    /// Returns tuple: `(date: UTCDate, tod: UTCTimeOfDay)`
    #[inline]
    pub const fn to_components(self) -> (UTCDate, UTCTimeOfDay) {
        (self.date, self.tod)
    }

    /// Get the internal date component.
    #[inline]
    pub const fn as_date(&self) -> UTCDate {
        self.date
    }

    /// Get the internal time-of-day component.
    #[inline]
    pub const fn as_tod(&self) -> UTCTimeOfDay {
        self.tod
    }

    /// Try parse datetime from string in the format:
    ///
    /// * `YYYY-MM-DDThh:mm:ssZ` or
    /// * `YYYY-MM-DDThh:mm:ss.nnnZ`
    ///
    /// Decimal precision of up to 9 places (inclusive) supported.
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    pub fn try_from_iso_datetime(iso: &str) -> Result<Self> {
        let (date_str, tod_str) = iso.split_at(10);
        let date = UTCDate::try_from_iso_date(date_str)?;
        let tod = UTCTimeOfDay::try_from_iso_tod(tod_str)?;
        Ok(Self::from_components(date, tod))
    }

    /// Return datetime as a string in the format:
    /// * Precision = `None`: `YYYY-MM-DDThh:mm:ssZ`
    /// * Precision = `Some(3)`: `YYYY-MM-DDThh:mm:ss.nnnZ`
    ///
    /// If specified, `precision` denotes the number decimal places included after the
    /// seconds, limited to 9 decimal places (nanosecond precision).
    /// If `None`, no decimal component is included.
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    #[cfg(feature = "std")]
    pub fn as_iso_datetime(&self, precision: Option<usize>) -> String {
        self.date.as_iso_date() + &self.tod.as_iso_tod(precision)
    }
}

impl UTCTransformations for UTCDatetime {
    fn from_timestamp(timestamp: UTCTimestamp) -> Self {
        let tod = timestamp.as_tod();
        let date = UTCDate::from_timestamp(timestamp);
        Self::from_components(date, tod)
    }

    fn as_timestamp(&self) -> UTCTimestamp {
        let (date, tod) = self.as_components();
        let day = date.as_day();
        UTCTimestamp::from_day_and_tod(day, tod)
    }
}

impl From<UTCTimestamp> for UTCDatetime {
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_timestamp(timestamp)
    }
}

impl From<Duration> for UTCDatetime {
    fn from(duration: Duration) -> Self {
        Self::from_duration(duration)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{UTCDatetime, date::UTCDate, time::{UTCTimeOfDay, UTCDay}};

    #[test]
    fn test_datetime_from_raw_components() -> Result<()> {
        let test_cases = [
            (1970, 1, 1, 0, 0, 0, 0, 0, UTCDay::ZERO), // thu, 00:00:00.000
            (2023, 6, 14, 09, 20, 09, 648_000_000, 33_609_648_000_000, UTCDay::try_from_u64(19522)?), // wed, 09:20:09.648
        ];

        for (year, month, day, hrs, mins, secs, subsec_ns, expected_tod_ns, expected_day) in test_cases {
            let date = UTCDate::try_from_components(year, month, day)?;
            let tod = UTCTimeOfDay::try_from_hhmmss(hrs, mins, secs, subsec_ns)?;
            let datetime = UTCDatetime::from_components(date, tod);
            assert_eq!(datetime.as_date().as_day(), expected_day);
            assert_eq!(datetime.as_tod().as_nanos(), expected_tod_ns);
        }

        Ok(())
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_datetime_iso_conversions() -> Result<()> {
        use crate::{date::UTCDate, time::UTCTimeOfDay};

        let test_cases = [
            (1970, 1, 1, 0, None, "1970-01-01T00:00:00Z"), // thu, 00:00:00
            (1970, 1, 1, 0, Some(0), "1970-01-01T00:00:00.Z"), // thu, 00:00:00.
            (1970, 1, 1, 0, Some(3), "1970-01-01T00:00:00.000Z"), // thu, 00:00:00.000
            (1970, 1, 1, 0, Some(9), "1970-01-01T00:00:00.000000000Z"), // thu, 00:00:00.000000000
            (1970, 1, 1, 0, Some(11), "1970-01-01T00:00:00.000000000Z"), // thu, 00:00:00.000000000
            (2023, 6, 14, 33_609_648_000_000, Some(3), "2023-06-14T09:20:09.648Z"), // wed, 09:20:09.648
        ];

        for (year, month, day, tod_ns, precision, iso_datetime) in test_cases {
            let date = UTCDate::try_from_components(year, month, day)?;
            let tod = UTCTimeOfDay::try_from_nanos(tod_ns)?;
            let datetime_from_components = UTCDatetime::from_components(date, tod);
            let datetime_from_iso = UTCDatetime::try_from_iso_datetime(iso_datetime)?;
            assert_eq!(datetime_from_components.as_iso_datetime(precision), iso_datetime);
            assert_eq!(datetime_from_iso, datetime_from_components)
        }

        Ok(())
    }
}
