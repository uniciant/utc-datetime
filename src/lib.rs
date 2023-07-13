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
//! - Format dates according to ISO 8601 `(YYYY-MM-DD)`
//! - Format datetimes according to ISO 8601 `(YYYY-MM-DDThh:mm:ssZ)`
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
//!     };
//!     use utc_dt::date::UTCDate;
//!
//!     // An example duration.
//!     // When a duration is used, it is assumed to be relative to the unix epoch.
//!     let example_duration = Duration::from_millis(1686824288903);
//!
//!     // UTC Timestamp from a duration
//!     let utc_timestamp = UTCTimestamp::from(example_duration); // OR
//!     let utc_timestamp = UTCTimestamp::from_utc_duration(example_duration);
//!     // UTC timestamp from the local system time.
//!     // Not available for #![no_std]
//!     let utc_timestamp = UTCTimestamp::try_from_system_time().unwrap();
//!     // UTC Timestamp from a time measurement (for secs, millis, micros, nanos)
//!     let utc_timestamp = UTCTimestamp::from_utc_millis(1686824288903);
//!     // Use UTC Timestamp to get a time measurement since the epoch (for secs, millis, micros, nanos)
//!     let utc_millis = utc_timestamp.as_utc_millis();
//!     // Use UTC Timestamp to get time-of-day
//!     let time_of_day_ns: u64 = utc_timestamp.as_time_of_day_ns();
//!     // Use UTC Timestamp to get days since epoch (ie. UTC Day)
//!     let utc_day: UTCDay = utc_timestamp.as_utc_day();
//!     // UTC Timestamp from a UTC Day and time-of-day components
//!     let utc_timestamp = UTCTimestamp::try_from_days_and_nanos(utc_day, time_of_day_ns).unwrap(); // OR
//!     let utc_timestamp = unsafe { UTCTimestamp::from_days_and_nanos(utc_day, time_of_day_ns) };
//!
//!     // UTC Day from an integer
//!     let utc_day = UTCDay::from(19523); // OR
//!     let utc_day = UTCDay::from_u32(19523);
//!     // Use UTC Day to get the weekday
//!     let weekday = utc_day.as_utc_weekday();
//!
//!     // UTC Date directly from components
//!     let utc_date = UTCDate::try_from_components(2023, 6, 15).unwrap(); // OR
//!     let utc_date = unsafe { UTCDate::from_components(2023, 6, 15) };
//!     // UTC Date from UTC Day
//!     let utc_date = UTCDate::from_utc_day(utc_day);
//!     // Check whether date occurs within leap year
//!     let is_leap_year: bool = utc_date.is_leap_year();
//!     // Get number of days within date's month
//!     let days_in_month: u8 = utc_date.days_in_month();
//!     // Get the date in integer forms
//!     let (year, month, day) = (utc_date.as_year(), utc_date.as_month(), utc_date.as_day()); // OR
//!     let (year, month, day) = utc_date.as_components();
//!     // UTC Day from UTC Date
//!     let utc_day = utc_date.as_utc_day();
//!     // Get date string formatted according to ISO 8601 `(YYYY-MM-DD)`
//!     // Not available for #![no_std]
//!     let iso_date = utc_date.as_iso_date();
//!     assert_eq!(iso_date, "2023-06-15");
//!
//!     // UTC Datetime directly from raw components
//!     let utc_datetime = UTCDatetime::try_from_raw_components(
//!         year,
//!         month,
//!         day,
//!         time_of_day_ns
//!     ).unwrap(); // OR
//!     let utc_datetime = unsafe { UTCDatetime::from_raw_components(
//!         year,
//!         month,
//!         day,
//!         time_of_day_ns
//!     )};
//!     // UTC Datetime from date and time-of-day components
//!     let utc_datetime = UTCDatetime::try_from_components(utc_date, time_of_day_ns).unwrap(); // OR
//!     let utc_datetime = unsafe { UTCDatetime::from_components(utc_date, time_of_day_ns) };
//!     // Get date and time-of-day components
//!     let (utc_date, time_of_day_ns) = (utc_datetime.as_date(), utc_datetime.as_time_of_day_ns());
//!     let (utc_date, time_of_day_ns) = utc_datetime.as_components();
//!     // Get the time in hours, minutes and seconds
//!     let (hours, minutes, seconds) = utc_datetime.as_hours_minutes_seconds();
//!     // Get the sub-second component of the time of day, in nanoseconds
//!     let subsec_ns = utc_datetime.as_subsec_ns();
//!     // Get UTC datetime string formatted according to ISO 8601 `(YYYY-MM-DDThh:mm:ssZ)`
//!     // Not available with `no_std`
//!     let iso_datetime = utc_datetime.as_iso_datetime();
//!     assert_eq!(iso_datetime, "2023-06-15T10:18:08Z");
//!
//!     {
//!         // `UTCTransformations` can be used to create shortcuts to the desired type!
//!         use utc_dt::time::UTCTransformations;
//!
//!         // Example shortcuts using `UTCTransformations`
//!         // UTC Day / UTC Date / UTC Datetime from a duration
//!         let utc_day = UTCDay::from_utc_duration(example_duration); // OR
//!         let utc_day = UTCDay::from(example_duration);
//!         let utc_date = UTCDate::from_utc_duration(example_duration); // OR
//!         let utc_date = UTCDate::from(example_duration);
//!         let utc_datetime = UTCDatetime::from_utc_duration(example_duration); // OR
//!         let utc_datetime = UTCDatetime::from(example_duration);
//!
//!         // UTC Day / UTC Date / UTC Datetime from a timestamp
//!         let utc_day = UTCDay::from_utc_timestamp(utc_timestamp); // OR
//!         let utc_day = UTCDay::from(utc_timestamp);
//!         let utc_date = UTCDate::from_utc_timestamp(utc_timestamp); // OR
//!         let utc_date = UTCDate::from(utc_timestamp);
//!         let utc_datetime = UTCDatetime::from_utc_timestamp(utc_timestamp); // OR
//!         let utc_datetime = UTCDatetime::from(utc_timestamp);
//!
//!         // UTC Day / UTC Date / UTC Datetime from local system time
//!         // Not available for #![no_std]
//!         let utc_day = UTCDay::try_from_system_time().unwrap();
//!         let utc_date = UTCDate::try_from_system_time().unwrap();
//!         let utc_datetime = UTCDatetime::try_from_system_time().unwrap();
//!
//!         // UTC Day / UTC Date / UTC Datetime from u64 epoch measurements
//!         let utc_day = UTCDay::from_utc_secs(1686824288);
//!         let utc_date = UTCDate::from_utc_millis(1686824288_000);
//!         let utc_datetime = UTCDate::from_utc_micros(1686824288_000_000);
//!
//!         // Convert from UTC Day / UTC Date / UTC Datetime back to various types
//!         let utc_duration: Duration = utc_day.as_utc_duration();
//!         let utc_timestamp: UTCTimestamp = utc_date.as_utc_timestamp();
//!         let utc_secs: u64 = utc_date.as_utc_secs();
//!         let utc_millis: u64 = utc_datetime.as_utc_millis();
//!         let utc_micros: u64 = utc_day.as_utc_micros();
//!         let utc_nanos: u64 = utc_date.as_utc_nanos();
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

use anyhow::{anyhow, Result};

use constants::*;
use date::UTCDate;
use time::{UTCTimestamp, UTCTransformations};

/// UTC Datetime.
/// A UTC Datetime consists of a date component and a time-of-day component
/// with nanosecond resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UTCDatetime {
    date: UTCDate,
    time_of_day_ns: u64,
}

impl UTCDatetime {
    const fn _from_components(date: UTCDate, time_of_day_ns: u64) -> Self {
        Self {
            date,
            time_of_day_ns,
        }
    }

    /// Unchecked method to create a datetime frome date and time-of-day components.
    ///
    /// # Safety
    /// Unsafe if the user passes an invalid time-of-day nanoseconds component (exceeding NANOS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_components(date: UTCDate, time_of_day_ns: u64) -> Self {
        Self::_from_components(date, time_of_day_ns)
    }

    /// Try to create a datetime from date and time-of-day components.
    pub fn try_from_components(date: UTCDate, time_of_day_ns: u64) -> Result<Self> {
        if time_of_day_ns >= NANOS_PER_DAY {
            return Err(anyhow!(
                "Nanoseconds not within a day! (time_of_day_ns: {})",
                time_of_day_ns
            ));
        }
        Ok(Self::_from_components(date, time_of_day_ns))
    }

    /// Unchecked method to create datetime from underlying raw components
    /// Will force create a `UTCDate` internally.
    ///
    /// # Safety
    /// Unsafe if the user passes an invalid year-month-day combination or
    /// time-of-day nanoseconds component (exceeding NANOS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_raw_components(
        year: u32,
        month: u8,
        day: u8,
        time_of_day_ns: u64,
    ) -> Self {
        unsafe {
            let date = UTCDate::from_components(year, month, day);
            Self::from_components(date, time_of_day_ns)
        }
    }

    /// Try to create a datetime from underlying raw components.
    /// Will try to create a `UTCDate` internally.
    pub fn try_from_raw_components(
        year: u32,
        month: u8,
        day: u8,
        time_of_day_ns: u64,
    ) -> Result<Self> {
        let date = UTCDate::try_from_components(year, month, day)?;
        Self::try_from_components(date, time_of_day_ns)
    }

    /// Get copy of the internal date and time-of-day components
    ///
    /// Returns tuple: `(date: UTCDate, time_of_day_ns: u64)`
    #[inline]
    pub const fn as_components(&self) -> (UTCDate, u64) {
        (self.date, self.time_of_day_ns)
    }

    /// Consume self into the internal date and time-of-day components
    ///
    /// Returns tuple: `(date: UTCDate, time_of_day_ns: u64)`
    #[inline]
    pub const fn to_components(self) -> (UTCDate, u64) {
        (self.date, self.time_of_day_ns)
    }

    /// Get the internal date component.
    #[inline]
    pub const fn as_date(&self) -> UTCDate {
        self.date
    }

    /// Get the internal time-of-day component.
    #[inline]
    pub const fn as_time_of_day_ns(&self) -> u64 {
        self.time_of_day_ns
    }

    /// Get the time-of-day expressed as hours minutes and seconds.
    ///
    /// Returns tuple `(hours: u8, minutes: u8, seconds: u8)`
    #[inline]
    pub const fn as_hours_minutes_seconds(&self) -> (u8, u8, u8) {
        let hours = (self.time_of_day_ns / NANOS_PER_HOUR) as u8;
        let minutes = ((self.time_of_day_ns % NANOS_PER_HOUR) / NANOS_PER_MINUTE) as u8;
        let seconds = ((self.time_of_day_ns % NANOS_PER_MINUTE) / NANOS_PER_SECOND) as u8;
        (hours, minutes, seconds)
    }

    /// Get the sub-second component of the time-of-day
    /// expressed in nanoseconds.
    #[inline]
    pub const fn as_subsec_ns(&self) -> u32 {
        (self.time_of_day_ns % NANOS_PER_SECOND) as u32
    }

    /// Return datetime as a string in the format:
    /// `YYYY-MM-DDThh:mm:ssZ`
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    #[cfg(feature = "std")]
    pub fn as_iso_datetime(&self) -> String {
        let date = self.date.as_iso_date();
        let (hours, minutes, seconds) = self.as_hours_minutes_seconds();
        format!("{date}T{:02}:{:02}:{:02}Z", hours, minutes, seconds)
    }
}

impl UTCTransformations for UTCDatetime {
    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self {
        let tod_ns = timestamp.as_time_of_day_ns();
        let date = UTCDate::from_utc_timestamp(timestamp);
        unsafe { Self::from_components(date, tod_ns) }
    }

    fn as_utc_timestamp(&self) -> UTCTimestamp {
        let (date, tod_ns) = self.as_components();
        let day = date.as_utc_day();
        unsafe { UTCTimestamp::from_days_and_nanos(day, tod_ns) }
    }
}

impl From<UTCTimestamp> for UTCDatetime {
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_utc_timestamp(timestamp)
    }
}

impl From<Duration> for UTCDatetime {
    fn from(duration: Duration) -> Self {
        Self::from_utc_duration(duration)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::UTCDatetime;

    #[test]
    fn test_datetime_from_raw_components() -> Result<()> {
        let test_cases = [
            (1970, 1, 1, 0, 0, 0),                                 // thu, 00:00:00.000
            (2023, 6, 14, 33_609_648_000_000, 19522, 648_000_000), // wed, 09:20:09.648
        ];

        for (year, month, day, time_of_day_ns, expected_utc_day, expected_subsec_ns) in test_cases {
            let datetime = UTCDatetime::try_from_raw_components(year, month, day, time_of_day_ns)?;
            assert_eq!(datetime.as_date().as_utc_day(), expected_utc_day.into());
            assert_eq!(datetime.as_subsec_ns(), expected_subsec_ns);
        }

        Ok(())
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_datetime_to_iso_time() -> Result<()> {
        let test_cases = [
            (1970, 1, 1, 0, "1970-01-01T00:00:00Z"), // thu, 00:00:00.000
            (2023, 6, 14, 33_609_648_000_000, "2023-06-14T09:20:09Z"), // wed, 09:20:09.648
        ];

        for (year, month, day, time_of_day_ns, expected_iso_datetime) in test_cases {
            let datetime = UTCDatetime::try_from_raw_components(year, month, day, time_of_day_ns)?;
            assert_eq!(datetime.as_iso_datetime(), expected_iso_datetime);
        }

        Ok(())
    }
}
