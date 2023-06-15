//! Time module.
//!
//! Implements core time concepts via UTC Timestamps and UTC Days.
//! Provides useful time constants for transformations.

use std::time::{
    Duration,
    SystemTime
};

use anyhow::{
    Result,
    anyhow
};
use derive_more::{
    Add,
    Sub,
    Mul,
    Div,
    From,
    Into,
};

/** Hours per day       */ pub const HOURS_PER_DAY: u64 = 24;
/** Minutes per day     */ pub const MINUTES_PER_DAY: u64 = HOURS_PER_DAY * 60;
/** Seconds per day     */ pub const SECONDS_PER_DAY: u64 = MINUTES_PER_DAY * 60;
/** Milliseconds per day*/ pub const MILLIS_PER_DAY: u64 = SECONDS_PER_DAY * 1000;
/** Microseconds per day*/ pub const MICROS_PER_DAY: u64 = MILLIS_PER_DAY * 1000;
/** Nanoseconds per day */ pub const NANOS_PER_DAY: u64 = MICROS_PER_DAY * 1000;

/** Minutes per hour        */ pub const MINUTES_PER_HOUR: u64 = 60;
/** Seconds per hour        */ pub const SECONDS_PER_HOUR: u64 = MINUTES_PER_HOUR * 60;
/** Milliseconds per hour   */ pub const MILLIS_PER_HOUR: u64 = SECONDS_PER_HOUR * 1000;
/** Microseconds per hour   */ pub const MICROS_PER_HOUR: u64 = MILLIS_PER_HOUR * 1000;
/** Nanoseconds per hour    */ pub const NANOS_PER_HOUR: u64 = MICROS_PER_HOUR * 1000;

/** Seconds per minute      */ pub const SECONDS_PER_MINUTE: u64 = 60;
/** Milliseconds per minute */ pub const MILLIS_PER_MINUTE: u64 = SECONDS_PER_MINUTE * 1000;
/** Microseconds per minute */ pub const MICROS_PER_MINUTE: u64 = MILLIS_PER_MINUTE * 1000;
/** Nanoseconds per minute  */ pub const NANOS_PER_MINUTE: u64 = MICROS_PER_MINUTE * 1000;

/** Milliseconds per second */ pub const MILLIS_PER_SECOND: u64 = 1000;
/** Microseconds per second */ pub const MICROS_PER_SECOND: u64 = MILLIS_PER_SECOND * 1000;
/** Microseconds per second */ pub const NANOS_PER_SECOND: u64 = MICROS_PER_SECOND * 1000;

/** Microseconds per millisecond*/ pub const MICROS_PER_MILLI: u64 = 1000;
/** Nanoseconds per millisecond */ pub const NANOS_PER_MILLI: u64 = MICROS_PER_MILLI * 1000;

/** Nanoseconds per microsecond */ pub const NANOS_PER_MICRO: u64 = 1000;

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
            return Err(anyhow!("Nanoseconds not within a day! (time_of_day_ns: {})", time_of_day_ns));
        }
        Ok(Self::from_days_and_nanos(days, time_of_day_ns))
    }

    /// Try to create a UTC Timestamp from the local system time.
    pub fn try_from_system_time() -> Result<Self> {
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(duration.into())
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
    pub fn to_time_of_day_nanos(&self) -> u64 {
        ((self.0.as_secs() % SECONDS_PER_DAY)  * NANOS_PER_SECOND) + (self.0.subsec_nanos() as u64)
    }
}

/// Common methods for creating UTC Datetime structures.
pub trait UTCTransformations
where
    Self: Sized
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Add, Sub, Mul, Div, From, Into)]
pub struct UTCDay(u32);

impl UTCTransformations for UTCDay {
    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self {
        Self((timestamp.0.as_secs() / SECONDS_PER_DAY) as u32)
    }
}

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

#[test]
fn test_from_days_and_nanos() -> Result<()> {
    let test_cases = [
        (UTCTimestamp(Duration::from_nanos(0)), UTCDay(0), 0, 4),
        (UTCTimestamp(Duration::from_nanos(123456789)), UTCDay(0), 123456789, 4),
        (UTCTimestamp(Duration::from_millis(1686756677000)), UTCDay(19522), 55877_000_000_000, 3),
        (UTCTimestamp(Duration::from_millis(1709220677000)), UTCDay(19782), 55877_000_000_000, 4),
        (UTCTimestamp(Duration::from_millis(1677684677000)), UTCDay(19417), 55877_000_000_000, 3),
        (UTCTimestamp(Duration::new(u32::MAX as u64 * SECONDS_PER_DAY, 0)), UTCDay(u32::MAX), 0, 0),
    ];

    for (
        expected_timestamp,
        utc_days,
        time_of_day_ns,
        weekday,
    )  in test_cases {
        let timestamp = UTCTimestamp::try_from_days_and_nanos(utc_days, time_of_day_ns)?;
        assert_eq!(timestamp, expected_timestamp);
        assert_eq!(UTCDay::from_utc_timestamp(timestamp), utc_days);
        assert_eq!(timestamp.to_time_of_day_nanos(), time_of_day_ns);
        assert_eq!(utc_days.to_utc_weekday(), weekday);
    }

    Ok(())
}
