//! # UTC Datetime
//! Simple and fast UTC dates, timestamps and datetimes library for Rust
//!
//! ## NOTE
//! Only capable of expressing times and dates SINCE the Unix Epoch `1970/01/01 00:00:00`. This library takes advantage of this assumption to simplify the API and internal logic.
//!
//! ## References
//! Date/timestamp conversion algorithims have been based upon Howard Hinnant's paper:
//! [`chrono`-Compatible Low-Level Date Algorithms](http://howardhinnant.github.io/date_algorithms.html)
//!
//! ## License
//! This project is licensed under either of
//! * [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
//! * [MIT License](https://opensource.org/licenses/MIT)
//! at your option.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(dead_code)]

pub mod date;
pub mod time;

use anyhow::{
    Result,
    anyhow
};

use date::UTCDate;
use time::{
    UTCTimestamp,
    UTCTransformations,
    NANOS_PER_DAY,
    NANOS_PER_HOUR,
    NANOS_PER_MINUTE,
    NANOS_PER_SECOND,
};

/// UTC Datetime.
/// A UTC Datetime consists of a date component and a time-of-day component
/// with nanosecond resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UTCDatetime {
    date: UTCDate,
    time_of_day_nanos: u64,
}

impl UTCTransformations for UTCDatetime {
    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self {
        let tod_ns = timestamp.to_time_of_day_nanos();
        let date = UTCDate::from_utc_timestamp(timestamp);
        Self::from_components(date, tod_ns)
    }
}

impl UTCDatetime {
    fn from_components(date: UTCDate, time_of_day_nanos: u64) -> Self {
        Self {
            date,
            time_of_day_nanos,
        }
    }

    /// Try to create a datetime from date and time-of-day components.
    pub fn try_from_components(date: UTCDate, time_of_day_nanos: u64) -> Result<Self> {
        if time_of_day_nanos >= NANOS_PER_DAY {
            return Err(anyhow!("Nanoseconds not within a day! (time_of_day_ns: {})", time_of_day_nanos));
        }
        Ok(Self::from_components(date, time_of_day_nanos))
    }

    /// Try to create a datetime from underlying raw components.
    /// Will try to create a `UTCDate` internally.
    pub fn try_from_raw_components(year: u32, month: u8, day: u8, time_of_day_nanos: u64) -> Result<Self> {
        let date = UTCDate::try_from_components(year, month, day)?;
        Ok(Self::try_from_components(date, time_of_day_nanos)?)
    }

    /// Get copy of the internal date and time-of-day components
    ///
    /// Returns tuple: `(date: UTCDate, time_of_day_nanos: u64)`
    pub fn to_components(&self) -> (UTCDate, u64) {
        (self.date, self.time_of_day_nanos)
    }

    /// Consume self into the internal date and time-of-day components
    ///
    /// Returns tuple: `(date: UTCDate, time_of_day_nanos: u64)`
    pub fn as_components(self) -> (UTCDate, u64) {
        (self.date, self.time_of_day_nanos)
    }

    /// Get the internal date component.
    pub fn to_date(&self) -> UTCDate {
        self.date
    }

    /// Get the internal time-of-day component.
    pub fn to_time_of_day_nanos(&self) -> u64 {
        self.time_of_day_nanos
    }

    /// Get date of the datetime expressed as years, days and months
    ///
    /// Returns tuple: `(years: u32, months: u8, days: u8)`
    pub fn to_years_months_days(&self) -> (u32, u8, u8) {
        (self.date.year(), self.date.month(), self.date.day())
    }

    /// Get the subsecond component of the time-of-day
    /// expressed in nanoseconds.
    pub fn to_subseconds_nanos(&self) -> u32 {
        (self.time_of_day_nanos % NANOS_PER_SECOND) as u32
    }

    /// Get the time-of-day expressed as hours minutes and seconds.
    ///
    /// Returns tuple `(hours: u8, minutes: u8, seconds: u8)`
    pub fn to_hours_minutes_seconds(&self) -> (u8, u8, u8) {
        let hours = (self.time_of_day_nanos / NANOS_PER_HOUR) as u8;
        let minutes = ((self.time_of_day_nanos % NANOS_PER_HOUR) / NANOS_PER_MINUTE) as u8;
        let seconds = ((self.time_of_day_nanos % NANOS_PER_MINUTE) / NANOS_PER_SECOND) as u8;
        (hours, minutes, seconds)
    }

    /// Return datetime as a string in the format:
    /// `YYYY-MM-DDThh:mm:ssZ`
    ///
    /// Conforms to ISO 8601:
    /// https://www.w3.org/TR/NOTE-datetime
    pub fn to_iso_datetime(&self) -> String {
        let date = self.date.to_iso_date();
        let (hours, minutes, seconds) = self.to_hours_minutes_seconds();
        format!("{date}T{:02}:{:02}:{:02}Z", hours, minutes, seconds)
    }
}

impl From<UTCTimestamp> for UTCDatetime {
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_utc_timestamp(timestamp)
    }
}