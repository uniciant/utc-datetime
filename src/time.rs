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
    /// Create a UTC Timestamp from UTC day
    #[inline]
    pub const fn from_day(day: UTCDay) -> Self {
        let secs = day.0 as u64 * SECONDS_PER_DAY;
        Self(Duration::from_secs(secs))
    }

    /// Create a UTC Timestamp from UTC day and time-of-day components
    #[inline]
    pub const fn from_day_and_tod(day: UTCDay, tod: UTCTimeOfDay) -> Self {
        let secs = day.0 as u64 * SECONDS_PER_DAY + tod.as_secs() as u64;
        let subsec_ns = tod.as_subsec_ns();
        Self(Duration::new(secs, subsec_ns))
    }

    /// Try to create a UTC Timestamp from the local system time.
    #[cfg(feature = "std")]
    pub fn try_from_system_time() -> Result<Self> {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(UTCTimestamp(duration))
    }

    /// Create UTC Timestamp from a duration.
    /// Constant evaluation alternative to `From<Duration>`.
    #[inline]
    pub const fn from_utc_duration(d: Duration) -> Self {
        Self(d)
    }

    /// UTC Timestamp as internal Duration since the Unix Epoch.
    #[inline]
    pub const fn as_utc_duration(&self) -> Duration {
        self.0
    }

    /// Consume UTC Timestamp into the internal Duration since the Unix Epoch.
    /// Constant evaluation alternative to `Into<Duration>`.
    #[inline]
    pub const fn to_utc_duration(self) -> Duration {
        self.0
    }

    /// Get the UTC time-of-day in nanoseconds.
    #[inline]
    pub const fn as_tod(&self) -> UTCTimeOfDay {
        let ns = ((self.0.as_secs() % SECONDS_PER_DAY) * NANOS_PER_SECOND) + (self.0.subsec_nanos() as u64);
        unsafe { UTCTimeOfDay::from_nanos_unchecked(ns) }
    }

    /// Get the number of UTC days since the Unix Epoch.
    #[inline]
    pub const fn as_utc_day(&self) -> UTCDay {
        UTCDay((self.0.as_secs() / SECONDS_PER_DAY) as u32)
    }

    /// Create UTC Timestamp from seconds since the Unix Epoch.
    #[inline]
    pub const fn from_utc_secs(s: u64) -> Self {
        UTCTimestamp(Duration::from_secs(s))
    }

    /// Convert to seconds measured from the Unix Epoch.
    #[inline]
    pub const fn as_utc_secs(&self) -> u64 {
        self.0.as_secs()
    }

    /// Create UTC Timestamp from milliseconds since the Unix Epoch.
    #[inline]
    pub const fn from_utc_millis(ms: u64) -> Self {
        UTCTimestamp(Duration::from_millis(ms))
    }

    /// Convert to milliseconds measured from the Unix Epoch.
    #[inline]
    pub const fn as_utc_millis(&self) -> u64 {
        self.0.as_millis() as u64
    }

    /// Create UTC Timestamp from microseconds since the Unix Epoch.
    #[inline]
    pub const fn from_utc_micros(us: u64) -> Self {
        UTCTimestamp(Duration::from_micros(us))
    }

    /// Convert to microseconds measured from the Unix Epoch.
    #[inline]
    pub const fn as_utc_micros(&self) -> u64 {
        self.0.as_micros() as u64
    }

    /// Create UTC Timestamp from nanoseconds since the Unix Epoch.
    #[inline]
    pub const fn from_utc_nanos(ns: u64) -> Self {
        UTCTimestamp(Duration::from_nanos(ns))
    }

    /// Convert to seconds measured from the Unix Epoch.
    #[inline]
    pub const fn as_utc_nanos(&self) -> u64 {
        self.0.as_nanos() as u64
    }
}

impl From<UTCDay> for UTCTimestamp {
    #[inline]
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
    #[inline]
    fn from_utc_duration(duration: Duration) -> Self {
        let timestamp = UTCTimestamp(duration);
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to a duration measured from the Unix Epoch.
    #[inline]
    fn as_utc_duration(&self) -> Duration {
        self.as_utc_timestamp().as_utc_duration()
    }

    /// Create from seconds measured from the Unix Epoch.
    #[inline]
    fn from_utc_secs(s: u64) -> Self {
        let timestamp = UTCTimestamp::from_utc_secs(s);
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to seconds measured from the Unix Epoch.
    #[inline]
    fn as_utc_secs(&self) -> u64 {
        self.as_utc_timestamp().as_utc_secs()
    }

    /// Create from milliseconds measured from the Unix Epoch.
    #[inline]
    fn from_utc_millis(ms: u64) -> Self {
        let timestamp = UTCTimestamp::from_utc_millis(ms);
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to milliseconds measured from the Unix Epoch.
    #[inline]
    fn as_utc_millis(&self) -> u64 {
        self.as_utc_timestamp().as_utc_millis()
    }

    /// Create from microseconds measured from the Unix Epoch.
    #[inline]
    fn from_utc_micros(us: u64) -> Self {
        let timestamp = UTCTimestamp::from_utc_micros(us);
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to microseconds measured from the Unix Epoch.
    #[inline]
    fn as_utc_micros(&self) -> u64 {
        self.as_utc_timestamp().as_utc_micros()
    }

    /// Create from nanoseconds measured from the Unix Epoch.
    #[inline]
    fn from_utc_nanos(ms: u64) -> Self {
        let timestamp = UTCTimestamp::from_utc_nanos(ms);
        Self::from_utc_timestamp(timestamp)
    }

    /// Convert to nanoseconds measured from the Unix Epoch.
    #[inline]
    fn as_utc_nanos(&self) -> u64 {
        self.as_utc_timestamp().as_utc_nanos()
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
/// UTC Day count.
/// UTC Days is the number of days since the Unix Epoch.
pub struct UTCDay(u32);

impl UTCDay {
    /// Create UTC Day from integer.
    /// Const evaluation alternative to `From<u32>`
    #[inline]
    pub const fn from_u32(u: u32) -> Self {
        Self(u)
    }

    /// UTC Day as internal integer
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    /// Consume UTC Day to internal integer
    /// Const evaluation alternative to `Into<u32>`
    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0
    }

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
    #[inline]
    fn from_utc_secs(s: u64) -> Self {
        Self((s / SECONDS_PER_DAY) as u32)
    }

    #[inline]
    fn as_utc_secs(&self) -> u64 {
        (self.0 as u64) * SECONDS_PER_DAY
    }

    #[inline]
    fn from_utc_millis(ms: u64) -> Self {
        Self((ms / MILLIS_PER_DAY) as u32)
    }

    #[inline]
    fn as_utc_millis(&self) -> u64 {
        (self.0 as u64) * MILLIS_PER_DAY
    }

    #[inline]
    fn from_utc_micros(us: u64) -> Self {
        Self((us / MICROS_PER_DAY) as u32)
    }

    #[inline]
    fn as_utc_micros(&self) -> u64 {
        (self.0 as u64) * MICROS_PER_DAY
    }

    #[inline]
    fn from_utc_nanos(ns: u64) -> Self {
        Self((ns / NANOS_PER_DAY) as u32)
    }

    #[inline]
    fn as_utc_nanos(&self) -> u64 {
        (self.0 as u64) * NANOS_PER_DAY
    }

    #[inline]
    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self {
        timestamp.as_utc_day()
    }

    #[inline]
    fn as_utc_timestamp(&self) -> UTCTimestamp {
        UTCTimestamp::from_day(*self)
    }
}

impl From<Duration> for UTCDay {
    #[inline]
    fn from(duration: Duration) -> Self {
        Self::from_utc_duration(duration)
    }
}

impl From<UTCTimestamp> for UTCDay {
    #[inline]
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_utc_timestamp(timestamp)
    }
}

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
)]
/// UTC Time of Day
pub struct UTCTimeOfDay(u64);

impl UTCTimeOfDay {
    /// Unchecked method to create time of day from nanoseconds
    ///
    /// ### Safety
    /// Unsafe if the user passes an invalid time-of-day nanoseconds component (exceeding NANOS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_nanos_unchecked(ns: u64) -> Self {
        Self(ns)
    }

    /// Unchecked method to create time of day from microseconds
    ///
    /// ### Safety
    /// Unsafe if the user passes an invalid time-of-day microsecond component (exceeding MICROS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_micros_unchecked(us: u64) -> Self {
        Self(us * NANOS_PER_MICRO)
    }

    /// Unchecked method to create time of day from milliseconds
    ///
    /// ### Safety
    /// Unsafe if the user passes an invalid time-of-day millisecond component (exceeding MILLIS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_millis_unchecked(ms: u32) -> Self {
        Self((ms as u64) * NANOS_PER_MILLI)
    }

    /// Unchecked method to create time of day from seconds
    ///
    /// ### Safety
    /// Unsafe if the user passes an invalid time-of-day seconds component (exceeding SECONDS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_secs_unchecked(s: u32) -> Self {
        Self((s as u64) * NANOS_PER_SECOND)
    }

    const fn _ns_from_hhmmss(hrs: u8, mins: u8, secs: u8, subsec_ns: u32) -> u64 {
        (subsec_ns as u64)
            + (hrs as u64) * NANOS_PER_HOUR
            + (mins as u64) * NANOS_PER_MINUTE
            + (secs as u64) * NANOS_PER_SECOND
    }

    /// Unchecked method to create UTC time of day from hours, minutes, seconds and subsecond (nanosecond) components
    ///
    /// # Safety
    /// Unsafe if the user passes a measure of time exceeding a day.
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_hhmmss_unchecked(hrs: u8, mins: u8, secs: u8, subsec_ns: u32) -> Self {
        Self(Self::_ns_from_hhmmss(hrs, mins, secs, subsec_ns))
    }

    /// Try to create UTC time of day from nanoseconds
    pub fn try_from_nanos(ns: u64) -> Result<Self> {
        if ns >= NANOS_PER_DAY {
            return Err(anyhow!(
                "Nanoseconds not within a day! (ns: {})",
                ns
            ));
        }
        Ok(Self(ns))
    }

    /// Try to create UTC time of day from microseconds
    pub fn try_from_micros(us: u64) -> Result<Self> {
        let ns = us.checked_mul(NANOS_PER_MICRO)
            .ok_or(anyhow!("Microseconds out of range!"))?;
        Self::try_from_nanos(ns)
    }

    /// Try to create UTC time of day from milliseconds
    pub fn try_from_millis(ms: u32) -> Result<Self> {
        Self::try_from_nanos((ms as u64) * NANOS_PER_MILLI)
    }

    /// Try to create UTC time of day from seconds
    pub fn try_from_secs(s: u32) -> Result<Self> {
        Self::try_from_nanos((s as u64) * NANOS_PER_SECOND)
    }

    /// Try to create UTC time of day from hours, minutes, seconds and subsecond (nanosecond) components
    pub fn try_from_hhmmss(hrs: u8, mins: u8, secs: u8, subsec_ns: u32) -> Result<Self> {
        Self::try_from_nanos(Self::_ns_from_hhmmss(hrs, mins, secs, subsec_ns))
    }

    /// Consume self into nanoseconds
    #[inline]
    pub const fn to_nanos(self) -> u64 {
        self.0
    }

    /// Time of day as nanoseconds
    #[inline]
    pub const fn as_nanos(&self) -> u64 {
        self.0
    }

    /// Time of day as microseconds
    #[inline]
    pub const fn as_micros(&self) -> u64 {
        self.0 / NANOS_PER_MICRO
    }

    /// Time of day as milliseconds
    #[inline]
    pub const fn as_millis(&self) -> u32 {
        (self.0 / NANOS_PER_MILLI) as u32
    }

    /// Time of day as seconds
    #[inline]
    pub const fn as_secs(&self) -> u32 {
        (self.0 / NANOS_PER_SECOND) as u32
    }

    /// Time of day as hours, minutes and seconds (hhmmss) components
    ///
    /// Returns tuple `(hrs: u8, mins: u8, secs: u8, subsec_ns: u32)`
    pub const fn as_hhmmss(&self) -> (u8, u8, u8) {
        let hrs = (self.0 / NANOS_PER_HOUR) as u8;
        let mins = ((self.0 % NANOS_PER_HOUR) / NANOS_PER_MINUTE) as u8;
        let secs = ((self.0 % NANOS_PER_MINUTE) / NANOS_PER_SECOND) as u8;
        (hrs, mins, secs)
    }

    /// Return subsecond component of time of day (in nanoseconds)
    #[inline]
    pub const fn as_subsec_ns(&self) -> u32 {
        (self.0 % NANOS_PER_SECOND)  as u32
    }
}

#[cfg(test)]
mod test {
    use core::time::Duration;

    use anyhow::Result;

    use crate::{time::{UTCDay, UTCTimeOfDay, UTCTimestamp, UTCTransformations}, constants::{SECONDS_PER_DAY, NANOS_PER_DAY, NANOS_PER_SECOND}};

    #[test]
    fn test_from_days_and_nanos() -> Result<()> {
        let test_cases = [
            (UTCTimestamp::from_utc_nanos(0), UTCDay::from_u32(0), UTCTimeOfDay::try_from_secs(0)?, 4),
            (
                UTCTimestamp::from_utc_nanos(123456789),
                UTCDay::from_u32(0),
                UTCTimeOfDay::try_from_nanos(123456789)?,
                4,
            ),
            (
                UTCTimestamp::from_utc_millis(1686756677000),
                UTCDay::from_u32(19522),
                UTCTimeOfDay::try_from_nanos(55_877_000_000_000)?,
                3,
            ),
            (
                UTCTimestamp::from_utc_millis(1709220677000),
                UTCDay::from_u32(19782),
                UTCTimeOfDay::try_from_micros(55_877_000_000)?,
                4,
            ),
            (
                UTCTimestamp::from_utc_millis(1677684677000),
                UTCDay::from_u32(19417),
                UTCTimeOfDay::try_from_millis(55_877_000)?,
                3,
            ),
            (
                UTCTimestamp::from_utc_duration(Duration::new(
                    (((u32::MAX as u64) + 1) * SECONDS_PER_DAY) - 1,
                    (NANOS_PER_SECOND - 1) as u32,
                )),
                UTCDay::from_u32(u32::MAX),
                UTCTimeOfDay::try_from_nanos(NANOS_PER_DAY - 1)?,
                0,
            ),
        ];

        for (expected_timestamp, utc_days, tod, weekday) in test_cases {
            let timestamp = UTCTimestamp::from_day_and_tod(utc_days, tod);
            assert_eq!(timestamp, expected_timestamp);
            assert_eq!(UTCDay::from_utc_timestamp(timestamp), utc_days);
            assert_eq!(timestamp.as_tod(), tod);
            assert_eq!(utc_days.as_utc_weekday(), weekday);
        }

        Ok(())
    }
}
