//! Time module.
//!
//! Implements core time concepts via UTC Timestamps and UTC Days.

use core::time::Duration;

#[cfg(feature = "std")]
use std::time::SystemTime;

use anyhow::{anyhow, Result};
use derive_more::{Add, Div, From, Into, Mul, Sub};

use crate::constants::*;

/// UTC Timestamp.
/// A UTC Timestamp is a Duration since the Unix Epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From, Into)]
pub struct UTCTimestamp(Duration);

impl UTCTimestamp {
    fn from_day_and_nanos(day: UTCDay, time_of_day_ns: u64) -> Self {
        let secs = (day.0 as u64 * SECONDS_PER_DAY) + (time_of_day_ns / NANOS_PER_SECOND);
        let nanos = (time_of_day_ns % NANOS_PER_SECOND) as u32;
        Duration::new(secs, nanos).into()
    }

    /// Create a UTC Timestamp from UTC day
    pub fn from_day(day: UTCDay) -> Self {
        let secs = day.0 as u64 * SECONDS_PER_DAY;
        Duration::from_secs(secs).into()
    }

    /// Try to create a UTC Timestamp from UTC day and time-of-day components.
    pub fn try_from_day_and_nanos(day: UTCDay, time_of_day_ns: u64) -> Result<Self> {
        if time_of_day_ns >= NANOS_PER_DAY {
            return Err(anyhow!(
                "Nanoseconds not within a day! (time_of_day_ns: {})",
                time_of_day_ns
            ));
        }
        Ok(Self::from_day_and_nanos(day, time_of_day_ns))
    }

    /// Try to create a UTC Timestamp from the local system time.
    #[cfg(feature = "std")]
    pub fn try_from_system_time() -> Result<Self> {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(duration.into())
    }

    /// UTC Timestamp as a Duration since the Unix Epoch.
    pub fn as_utc_duration(&self) -> Duration {
        self.0
    }

    /// Consume UTC Timestamp into a Duration since the Unix Epoch.
    pub fn to_utc_duration(self) -> Duration {
        self.0
    }

    /// Get the UTC time-of-day in nanoseconds.
    pub fn as_time_of_day_ns(&self) -> u64 {
        ((self.0.as_secs() % SECONDS_PER_DAY) * NANOS_PER_SECOND) + (self.0.subsec_nanos() as u64)
    }

    /// Get the number of UTC days since the Unix Epoch.
    pub fn as_utc_day(&self) -> UTCDay {
        ((self.0.as_secs() / SECONDS_PER_DAY) as u32).into()
    }
}

impl From<UTCDay> for UTCTimestamp {
    fn from(day: UTCDay) -> Self {
        UTCTimestamp::from_day(day)
    }
}

/// Common methods for creating and converting between UTC structures.
pub trait UTCTransformations
where
    Self: Sized,
{
    /// Create from a duration measured from the Unix Epoch.
    fn from_utc_duration(duration: Duration) -> Self {
        let timestamp = duration.into();
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to a duration measured from the Unix Epoch.
    fn as_utc_duration(&self) -> Duration {
        self.as_utc_timestamp().into()
    }

    /// Create from seconds measured from the Unix Epoch.
    fn from_utc_secs(s: u64) -> Self {
        let timestamp = Duration::from_secs(s).into();
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to seconds measured from the Unix Epoch.
    fn as_utc_secs(&self) -> u64 {
        self.as_utc_duration().as_secs()
    }

    /// Create from milliseconds measured from the Unix Epoch.
    fn from_utc_millis(ms: u64) -> Self {
        let timestamp = Duration::from_millis(ms).into();
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to milliseconds measured from the Unix Epoch.
    fn as_utc_millis(&self) -> u64 {
        self.as_utc_duration().as_millis() as u64
    }

    /// Create from milliseconds measured from the Unix Epoch.
    fn from_utc_micros(us: u64) -> Self {
        let timestamp = Duration::from_micros(us).into();
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to microseconds measured from the Unix Epoch.
    fn as_utc_micros(&self) -> u64 {
        self.as_utc_duration().as_micros() as u64
    }

    /// Create from nanoseconds measured from the Unix Epoch.
    fn from_utc_nanos(ns: u64) -> Self {
        let timestamp = Duration::from_nanos(ns).into();
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to nanoseconds measured from the Unix Epoch.
    fn as_utc_nanos(&self) -> u64 {
        self.as_utc_duration().as_nanos() as u64
    }

    /// Create from local system time
    #[cfg(feature = "std")]
    fn try_from_system_time() -> Result<Self> {
        let timestamp = UTCTimestamp::try_from_system_time()?;
        Ok(Self::from_utc_timestamp(timestamp))
    }

    /// Create from a UTC timestamp.
    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self;
    /// Convert to a UTC timestamp.
    fn as_utc_timestamp(&self) -> UTCTimestamp;
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
    /// <http://howardhinnant.github.io/date_algorithms.html#weekday_from_days>
    pub fn as_utc_weekday(&self) -> u8 {
        ((self.0 as u64 + 4) % 7) as u8
    }
}

impl UTCTransformations for UTCDay {
    fn from_utc_secs(s: u64) -> Self {
        Self((s / SECONDS_PER_DAY) as u32)
    }

    fn as_utc_secs(&self) -> u64 {
        (self.0 as u64) * SECONDS_PER_DAY
    }

    fn from_utc_millis(ms: u64) -> Self {
        Self((ms / MILLIS_PER_DAY) as u32)
    }

    fn as_utc_millis(&self) -> u64 {
        (self.0 as u64) * MILLIS_PER_DAY
    }

    fn from_utc_micros(us: u64) -> Self {
        Self((us / MICROS_PER_DAY) as u32)
    }

    fn as_utc_micros(&self) -> u64 {
        (self.0 as u64) * MICROS_PER_DAY
    }

    fn from_utc_nanos(ns: u64) -> Self {
        Self((ns / NANOS_PER_DAY) as u32)
    }

    fn as_utc_nanos(&self) -> u64 {
        (self.0 as u64) * NANOS_PER_DAY
    }

    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self {
        timestamp.as_utc_day()
    }

    fn as_utc_timestamp(&self) -> UTCTimestamp {
        UTCTimestamp::from_day(*self)
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
    use core::time::Duration;

    use crate::time::{UTCDay, UTCTimestamp, UTCTransformations, SECONDS_PER_DAY};

    #[test]
    fn test_from_days_and_nanos() -> Result<()> {
        let test_cases = [
            (Duration::from_nanos(0), UTCDay(0), 0, 4),
            (Duration::from_nanos(123456789), UTCDay(0), 123456789, 4),
            (
                Duration::from_millis(1686756677000),
                UTCDay(19522),
                55_877_000_000_000,
                3,
            ),
            (
                Duration::from_millis(1709220677000),
                UTCDay(19782),
                55_877_000_000_000,
                4,
            ),
            (
                Duration::from_millis(1677684677000),
                UTCDay(19417),
                55_877_000_000_000,
                3,
            ),
            (
                Duration::new(u32::MAX as u64 * SECONDS_PER_DAY, 0),
                UTCDay(u32::MAX),
                0,
                0,
            ),
        ];

        for (expected_timestamp, utc_days, time_of_day_ns, weekday) in test_cases {
            let timestamp = UTCTimestamp::try_from_day_and_nanos(utc_days, time_of_day_ns)?;
            assert_eq!(timestamp, expected_timestamp.into());
            assert_eq!(UTCDay::from_utc_timestamp(timestamp), utc_days);
            assert_eq!(timestamp.as_time_of_day_ns(), time_of_day_ns);
            assert_eq!(utc_days.as_utc_weekday(), weekday);
        }

        Ok(())
    }
}
