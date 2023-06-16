//! Time module.
//!
//! Implements core time concepts via UTC Timestamps and UTC Days.

use std::time::{Duration, SystemTime};

use anyhow::{anyhow, Result};
use derive_more::{Add, Div, From, Into, Mul, Sub};

use crate::constants::*;

/// UTC Timestamp.
/// A UTC Timestamp is a Duration since the Unix Epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From, Into)]
pub struct UTCTimestamp(Duration);

impl UTCTimestamp {
    fn from_days_and_nanos(days: UTCDay, time_of_day_ns: u64) -> Self {
        let secs = (days.0 as u64 * SECONDS_PER_DAY) + (time_of_day_ns / NANOS_PER_SECOND);
        let nanos = (time_of_day_ns % NANOS_PER_SECOND) as u32;
        Duration::new(secs, nanos).into()
    }

    /// Try to create a UTC Timestamp from UTC day and time-of-day components.
    pub fn try_from_days_and_nanos(days: UTCDay, time_of_day_ns: u64) -> Result<Self> {
        if time_of_day_ns >= NANOS_PER_DAY {
            return Err(anyhow!(
                "Nanoseconds not within a day! (time_of_day_ns: {})",
                time_of_day_ns
            ));
        }
        Ok(Self::from_days_and_nanos(days, time_of_day_ns))
    }

    /// Try to create a UTC Timestamp from the local system time.
    pub fn try_from_system_time() -> Result<Self> {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(duration.into())
    }

    /// Create a timestamp directly from seconds since the Unix Epoch.
    pub fn from_secs(s: u64) -> Self {
        Self(Duration::from_secs(s))
    }

    /// Create a timestamp directly from milliseconds since the Unix Epoch.
    pub fn from_millis(ms: u64) -> Self {
        Self(Duration::from_millis(ms))
    }

    /// Create a timestamp directly from microseconds since the Unix Epoch.
    pub fn from_micros(us: u64) -> Self {
        Self(Duration::from_micros(us))
    }

    /// Create a timestamp directly from nanoseconds since the Unix Epoch.
    pub fn from_nanos(ns: u64) -> Self {
        Self(Duration::from_nanos(ns))
    }

    /// Get the UTC time-of-day in nanoseconds.
    pub fn to_time_of_day_ns(&self) -> u64 {
        ((self.0.as_secs() % SECONDS_PER_DAY) * NANOS_PER_SECOND) + (self.0.subsec_nanos() as u64)
    }

    /// Get the number of UTC days since the Unix Epoch.
    pub fn to_utc_day(&self) -> UTCDay {
        ((self.0.as_secs() / SECONDS_PER_DAY) as u32).into()
    }
}

/// Common methods for creating UTC Datetime structures.
pub trait UTCTransformations
where
    Self: Sized,
{
    /// Create from a duration measured from the Unix Epoch.
    fn from_utc_duration(duration: Duration) -> Self {
        let timestamp = duration.into();
        Self::from_utc_timestamp(timestamp)
    }
    /// Create from local system time
    fn try_from_system_time() -> Result<Self> {
        let timestamp = UTCTimestamp::try_from_system_time()?;
        Ok(Self::from_utc_timestamp(timestamp))
    }
    /// Create from seconds measured from the Unix Epoch.
    fn from_utc_secs(s: u64) -> Self {
        let timestamp = UTCTimestamp::from_secs(s);
        Self::from_utc_timestamp(timestamp)
    }
    /// Create from milliseconds measured from the Unix Epoch.
    fn from_utc_millis(ms: u64) -> Self {
        let timestamp = UTCTimestamp::from_millis(ms);
        Self::from_utc_timestamp(timestamp)
    }
    /// Create from milliseconds measured from the Unix Epoch.
    fn from_utc_micros(us: u64) -> Self {
        let timestamp = UTCTimestamp::from_micros(us);
        Self::from_utc_timestamp(timestamp)
    }
    /// Create from nanoseconds measured from the Unix Epoch.
    fn from_utc_nanos(ns: u64) -> Self {
        let timestamp = UTCTimestamp::from_nanos(ns);
        Self::from_utc_timestamp(timestamp)
    }
    /// Create from a UTC timestamp.
    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self;
}

/// UTC Day count.
/// UTC Days is the number of days since the Unix Epoch.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Add,
    Sub,
    Mul,
    Div,
    From,
    Into,
)]
pub struct UTCDay(u32);

impl UTCDay {
    /// Calculate and return the day of the week in numerical form
    /// `[0, 6]` represents `[Sun, Sat]`
    ///
    /// Reference:
    /// http://howardhinnant.github.io/date_algorithms.html#weekday_from_days
    pub fn to_utc_weekday(&self) -> u8 {
        ((self.0 as u64 + 4) % 7) as u8
    }
}

impl UTCTransformations for UTCDay {
    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self {
        timestamp.to_utc_day()
    }
}

impl From<Duration> for UTCDay {
    fn from(duration: Duration) -> Self {
        Self::from_utc_duration(duration)
    }
}

impl From<UTCTimestamp> for UTCDay {
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_utc_timestamp(timestamp)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::time::Duration;

    use crate::time::{UTCDay, UTCTimestamp, UTCTransformations, SECONDS_PER_DAY};

    #[test]
    fn test_from_days_and_nanos() -> Result<()> {
        let test_cases = [
            (UTCTimestamp(Duration::from_nanos(0)), UTCDay(0), 0, 4),
            (
                UTCTimestamp(Duration::from_nanos(123456789)),
                UTCDay(0),
                123456789,
                4,
            ),
            (
                UTCTimestamp(Duration::from_millis(1686756677000)),
                UTCDay(19522),
                55_877_000_000_000,
                3,
            ),
            (
                UTCTimestamp(Duration::from_millis(1709220677000)),
                UTCDay(19782),
                55_877_000_000_000,
                4,
            ),
            (
                UTCTimestamp(Duration::from_millis(1677684677000)),
                UTCDay(19417),
                55_877_000_000_000,
                3,
            ),
            (
                UTCTimestamp(Duration::new(u32::MAX as u64 * SECONDS_PER_DAY, 0)),
                UTCDay(u32::MAX),
                0,
                0,
            ),
        ];

        for (expected_timestamp, utc_days, time_of_day_ns, weekday) in test_cases {
            let timestamp = UTCTimestamp::try_from_days_and_nanos(utc_days, time_of_day_ns)?;
            assert_eq!(timestamp, expected_timestamp);
            assert_eq!(UTCDay::from_utc_timestamp(timestamp), utc_days);
            assert_eq!(timestamp.to_time_of_day_ns(), time_of_day_ns);
            assert_eq!(utc_days.to_utc_weekday(), weekday);
        }

        Ok(())
    }
}
