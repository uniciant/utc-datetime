use std::time::Duration;

use anyhow::{
    Result,
    anyhow
};

use crate::date::UTCDate;

const SECONDS_PER_DAY: u64 = 86400;
const MILLIS_PER_DAY: u64 = SECONDS_PER_DAY * 1000;
const MICROS_PER_DAY: u64 = MILLIS_PER_DAY * 1000;
const NANOS_PER_DAY: u64 = MICROS_PER_DAY * 1000;
const MILLIS_PER_SECOND: u64 = 1000;
const MICROS_PER_SECOND: u64 = MILLIS_PER_SECOND * 1000;
const NANOS_PER_SECOND: u64 = MICROS_PER_SECOND * 1000;

/// UTC Timestamp.
/// A UTC Timestamp the duration since the Unix Epoch (1970).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UTCTimestamp(Duration);

impl UTCTimestamp {
    pub fn to_time_of_day_nanos(&self) -> u64 {
        ((self.0.as_secs() % SECONDS_PER_DAY)  * NANOS_PER_SECOND) + (self.0.subsec_nanos() as u64)
    }

    pub fn to_utc_days(&self) -> u32 {
        (self.0.as_secs() / SECONDS_PER_DAY) as u32
    }

    /// Calculate and return the day of the week, in numerical form
    ///
    /// Reference:
    /// http://howardhinnant.github.io/date_algorithms.html#weekday_from_days
    pub fn to_utc_weekday(&self) -> u8 {
        ((self.to_utc_days() + 4) % 7) as u8
    }

    pub fn try_from_components(secs: u64, nanos: u32) -> Result<Self> {
        Ok(Self(Duration::new(secs, nanos)))
    }

    pub fn try_from_days_and_nanos(days: u32, time_of_day_ns: u64) -> Result<Self> {
        if time_of_day_ns >= NANOS_PER_DAY {
            return Err(anyhow!("Nanoseconds not within a day! (time_of_day_ns: {})", time_of_day_ns));
        }
        let secs = (days as u64 * SECONDS_PER_DAY) + (time_of_day_ns / NANOS_PER_SECOND);
        let nanos = (time_of_day_ns % NANOS_PER_SECOND) as u32;
        Self::try_from_components(secs, nanos)
    }

    pub fn from_millis(ms: u64) -> Self {
        Self(Duration::from_millis(ms))
    }

    pub fn from_micros(us: u64) -> Self {
        Self(Duration::from_micros(us))
    }

    pub fn from_nanos(ns: u64) -> Self {
        Self(Duration::from_nanos(ns))
    }
}

impl From<UTCDate> for UTCTimestamp {
    fn from(date: UTCDate) -> Self {
        Self::try_from_days_and_nanos(date.to_utc_days(), 0).unwrap()
    }
}

#[test]
fn test_from_days_and_nanos() -> Result<()> {
    let test_cases = [
        (UTCTimestamp(Duration::from_nanos(0)), 0, 0),
        (UTCTimestamp(Duration::from_nanos(123456789)), 0, 123456789),
        (UTCTimestamp(Duration::from_millis(1686756677000)), 19522, 55877_000_000_000),
        (UTCTimestamp(Duration::from_millis(1709220677000)), 19782, 55877_000_000_000),
        (UTCTimestamp(Duration::from_millis(1677684677000)), 19417, 55877_000_000_000),
        (UTCTimestamp(Duration::new(u32::MAX as u64 * SECONDS_PER_DAY, 0)), u32::MAX, 0),
    ];

    for (expected, utc_day, time_of_day_ns)  in test_cases {
        let timestamp = UTCTimestamp::try_from_days_and_nanos(utc_day, time_of_day_ns)?;
        assert_eq!(timestamp, expected);
    }

    Ok(())
}
