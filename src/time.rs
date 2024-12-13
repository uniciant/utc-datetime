//! Time module.
//!
//! Implements core time concepts via UTC Timestamps, UTC Days and UTC Time-of-Days.

use crate::constants::*;
use crate::util::StrWriter;
use core::error::Error;
use core::fmt::{Display, Formatter, Write};
use core::num::ParseIntError;
use core::ops::*;
use core::time::Duration;

#[cfg(feature = "alloc")]
use alloc::{format, string::String};

#[cfg(feature = "std")]
use std::time::{SystemTime, SystemTimeError};

/// UTC Timestamp.
///
/// A UTC Timestamp is a Duration since the Unix Epoch.
///
/// ## Examples
#[cfg_attr(not(feature = "std"), doc = "```rust,ignore")]
#[cfg_attr(feature = "std", doc = "```rust")]
/// use core::time::Duration;
///
/// use utc_dt::time::{
///     UTCTimestamp,
///     UTCDay,
///     UTCTimeOfDay,
/// };
///
/// // An example duration.
/// // When a duration is used, it is assumed to be relative to the unix epoch.
/// // Thursday, 15 June 2023 10:18:08.903
/// let example_duration = Duration::from_millis(1686824288903);
///
/// // UTC Timestamp from a duration
/// let utc_timestamp = UTCTimestamp::from(example_duration); // OR
/// let utc_timestamp = UTCTimestamp::from_duration(example_duration);
/// // UTC timestamp from the local system time.
/// // Not available for #![no_std]
/// let utc_timestamp = UTCTimestamp::try_from_system_time().unwrap();
/// // UTC Timestamp from a time measurement (for secs, millis, micros, nanos)
/// let utc_timestamp = UTCTimestamp::from_millis(1686824288903);
/// // Use UTC Timestamp to get a time measurement since the epoch (for secs, millis, micros, nanos)
/// let utc_millis = utc_timestamp.as_millis();
/// // Use UTC Timestamp to get time-of-day
/// let utc_tod: UTCTimeOfDay = utc_timestamp.as_tod();
/// // Use UTC Timestamp to get days since epoch (ie. UTC Day)
/// let utc_day: UTCDay = utc_timestamp.as_day();
/// // UTC Timestamp from UTC Day and time-of-day components
/// let utc_timestamp = UTCTimestamp::from_day_and_tod(utc_day, utc_tod);
/// // Manipulate UTC Timestamps with standard math operators
/// assert_eq!(utc_timestamp + utc_timestamp, utc_timestamp * 2);
/// assert_eq!(utc_timestamp - example_duration, UTCTimestamp::ZERO);
/// // Easily apply offsets of various measurements to timestamps
/// let utc_timestamp_plus_1s = utc_timestamp.saturating_add_millis(1000);
/// let utc_timestamp_minus_1s = utc_timestamp.saturating_sub_secs(1);
/// ```
///
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UTCTimestamp(Duration);

impl UTCTimestamp {
    /// The 'Zero' UTC Timestamp
    ///
    /// Equivalent to the instant of the epoch
    pub const ZERO: UTCTimestamp = UTCTimestamp(Duration::ZERO);

    /// The maximum UTC Timestamp
    ///
    /// Equal to `November 9, 584_554_051_223`
    pub const MAX: UTCTimestamp = UTCTimestamp(Duration::MAX);

    /// Create a UTC Timestamp from UTC day
    #[inline]
    pub const fn from_day(day: UTCDay) -> Self {
        let secs = day.0 * SECONDS_PER_DAY;
        Self(Duration::from_secs(secs))
    }

    /// Create a UTC Timestamp from UTC day and time-of-day components
    #[inline]
    pub const fn from_day_and_tod(day: UTCDay, tod: UTCTimeOfDay) -> Self {
        let secs = (day.0 * SECONDS_PER_DAY).saturating_add(tod.as_secs() as u64);
        let subsec_ns = tod.as_subsec_ns();
        Self(Duration::new(secs, subsec_ns))
    }

    /// Try to create a UTC Timestamp from the local system time.
    #[cfg(feature = "std")]
    pub fn try_from_system_time() -> Result<Self, SystemTimeError> {
        let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(UTCTimestamp(duration))
    }

    /// Create UTC Timestamp from a duration.
    /// Constant evaluation alternative to `From<Duration>`.
    #[inline]
    pub const fn from_duration(d: Duration) -> Self {
        Self(d)
    }

    /// UTC Timestamp as internal Duration since the Unix Epoch.
    #[inline]
    pub const fn as_duration(&self) -> Duration {
        self.0
    }

    /// Consume UTC Timestamp into the internal Duration since the Unix Epoch.
    #[inline]
    pub const fn to_duration(self) -> Duration {
        self.0
    }

    /// Get the UTC time-of-day in nanoseconds.
    #[inline]
    pub const fn as_tod(&self) -> UTCTimeOfDay {
        let ns = ((self.0.as_secs() % SECONDS_PER_DAY) * NANOS_PER_SECOND)
            + (self.0.subsec_nanos() as u64);
        // SAFETY: nanos is within NANOS_PER_DAY
        unsafe { UTCTimeOfDay::from_nanos_unchecked(ns) }
    }

    /// Get the number of UTC days since the Unix Epoch.
    #[inline]
    pub const fn as_day(&self) -> UTCDay {
        UTCDay(self.0.as_secs() / SECONDS_PER_DAY)
    }

    /// Create UTC Timestamp from seconds since the Unix Epoch.
    #[inline]
    pub const fn from_secs(secs: u64) -> Self {
        UTCTimestamp(Duration::from_secs(secs))
    }

    /// Convert to seconds measured from the Unix Epoch.
    #[inline]
    pub const fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }

    /// Create UTC Timestamp from milliseconds since the Unix Epoch.
    #[inline]
    pub const fn from_millis(millis: u64) -> Self {
        UTCTimestamp(Duration::from_millis(millis))
    }

    /// Convert to milliseconds measured from the Unix Epoch.
    #[inline]
    pub const fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    /// Create UTC Timestamp from microseconds since the Unix Epoch.
    #[inline]
    pub const fn from_micros(micros: u64) -> Self {
        UTCTimestamp(Duration::from_micros(micros))
    }

    /// Convert to microseconds measured from the Unix Epoch.
    #[inline]
    pub const fn as_micros(&self) -> u128 {
        self.0.as_micros()
    }

    /// Create UTC Timestamp from nanoseconds since the Unix Epoch.
    #[inline]
    pub const fn from_nanos(nanos: u64) -> Self {
        UTCTimestamp(Duration::from_nanos(nanos))
    }

    /// Convert to seconds measured from the Unix Epoch.
    #[inline]
    pub const fn as_nanos(&self) -> u128 {
        self.0.as_nanos()
    }

    /// Checked `UTCTimestamp` addition. Computes `self + other`, returning [`None`]
    /// if overflow occurred.
    #[inline]
    pub const fn checked_add(self, rhs: UTCTimestamp) -> Option<UTCTimestamp> {
        match self.0.checked_add(rhs.0) {
            Some(duration) => Some(UTCTimestamp(duration)),
            None => None,
        }
    }

    /// Checked `UTCTimestamp` addition with `Duration`. Computes `self + other`, returning [`None`]
    /// if overflow occurred.
    #[inline]
    pub const fn checked_add_duration(self, rhs: Duration) -> Option<UTCTimestamp> {
        match self.0.checked_add(rhs) {
            Some(duration) => Some(UTCTimestamp(duration)),
            None => None,
        }
    }

    /// Saturating `UTCTimestamp` addition. Computes `self + other`, returning [`UTCTimestamp::MAX`]
    /// if overflow occurred.
    #[inline]
    pub const fn saturating_add(self, rhs: UTCTimestamp) -> UTCTimestamp {
        match self.checked_add(rhs) {
            Some(res) => res,
            None => UTCTimestamp::MAX,
        }
    }

    /// Saturating `UTCTimestamp` addition with `Duration`. Computes `self + other`, returning [`UTCTimestamp::MAX`]
    /// if overflow occurred.
    #[inline]
    pub const fn saturating_add_duration(self, rhs: Duration) -> UTCTimestamp {
        match self.checked_add_duration(rhs) {
            Some(res) => res,
            None => UTCTimestamp::MAX,
        }
    }

    /// Saturating `UTCTimestamp` addition with nanoseconds. Computes `self + other`, returning [`UTCTimestamp::MAX`]
    /// if overflow occurred.
    #[inline]
    pub const fn saturating_add_nanos(self, rhs: u64) -> UTCTimestamp {
        self.saturating_add(UTCTimestamp::from_nanos(rhs))
    }

    /// Saturating `UTCTimestamp` addition with microseconds. Computes `self + other`, returning [`UTCTimestamp::MAX`]
    /// if overflow occurred.
    #[inline]
    pub const fn saturating_add_micros(self, rhs: u64) -> UTCTimestamp {
        self.saturating_add(UTCTimestamp::from_micros(rhs))
    }

    /// Saturating `UTCTimestamp` addition with milliseconds. Computes `self + other`, returning [`UTCTimestamp::MAX`]
    /// if overflow occurred.
    #[inline]
    pub const fn saturating_add_millis(self, rhs: u64) -> UTCTimestamp {
        self.saturating_add(UTCTimestamp::from_millis(rhs))
    }

    /// Saturating `UTCTimestamp` addition with seconds. Computes `self + other`, returning [`UTCTimestamp::MAX`]
    /// if overflow occurred.
    #[inline]
    pub const fn saturating_add_secs(self, rhs: u64) -> UTCTimestamp {
        self.saturating_add(UTCTimestamp::from_secs(rhs))
    }

    /// Checked `UTCTimestamp` subtraction. Computes `self - other`, returning [`None`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn checked_sub(self, rhs: UTCTimestamp) -> Option<UTCTimestamp> {
        match self.0.checked_sub(rhs.0) {
            Some(duration) => Some(UTCTimestamp(duration)),
            None => None,
        }
    }

    /// Checked `UTCTimestamp` subtraction with `Duration`. Computes `self - other`, returning [`None`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn checked_sub_duration(self, rhs: Duration) -> Option<UTCTimestamp> {
        match self.0.checked_sub(rhs) {
            Some(duration) => Some(UTCTimestamp(duration)),
            None => None,
        }
    }

    /// Saturating `UTCTimestamp` subtraction. Computes `self - other`, returning [`UTCTimestamp::ZERO`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn saturating_sub(self, rhs: UTCTimestamp) -> UTCTimestamp {
        match self.checked_sub(rhs) {
            Some(res) => res,
            None => UTCTimestamp::ZERO,
        }
    }

    /// Saturating `UTCTimestamp` subtraction with `Duration`. Computes `self - other`, returning [`UTCTimestamp::ZERO`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn saturating_sub_duration(self, rhs: Duration) -> UTCTimestamp {
        match self.checked_sub_duration(rhs) {
            Some(res) => res,
            None => UTCTimestamp::ZERO,
        }
    }

    /// Saturating `UTCTimestamp` subtraction with nanoseconds. Computes `self + other`, returning [`UTCTimestamp::ZERO`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn saturating_sub_nanos(self, rhs: u64) -> UTCTimestamp {
        self.saturating_sub(UTCTimestamp::from_nanos(rhs))
    }

    /// Saturating `UTCTimestamp` subtraction with microseconds. Computes `self + other`, returning [`UTCTimestamp::ZERO`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn saturating_sub_micros(self, rhs: u64) -> UTCTimestamp {
        self.saturating_sub(UTCTimestamp::from_micros(rhs))
    }

    /// Saturating `UTCTimestamp` subtraction with milliseconds. Computes `self + other`, returning [`UTCTimestamp::ZERO`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn saturating_sub_millis(self, rhs: u64) -> UTCTimestamp {
        self.saturating_sub(UTCTimestamp::from_millis(rhs))
    }

    /// Saturating `UTCTimestamp` subtraction with seconds. Computes `self + other`, returning [`UTCTimestamp::ZERO`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn saturating_sub_secs(self, rhs: u64) -> UTCTimestamp {
        self.saturating_sub(UTCTimestamp::from_secs(rhs))
    }

    /// Checked `UTCTimestamp` multiplication. Computes `self * other`, returning
    /// [`None`] if overflow occurred.
    #[inline]
    pub const fn checked_mul(self, rhs: u32) -> Option<UTCTimestamp> {
        match self.0.checked_mul(rhs) {
            Some(duration) => Some(UTCTimestamp(duration)),
            None => None,
        }
    }

    /// Saturating `UTCTimestamp` multiplication. Computes `self * other`, returning
    /// [`UTCTimestamp::MAX`] if overflow occurred.
    #[inline]
    pub const fn saturating_mul(self, rhs: u32) -> UTCTimestamp {
        match self.checked_mul(rhs) {
            Some(res) => res,
            None => UTCTimestamp::MAX,
        }
    }

    /// Checked `UTCTimestamp` division. Computes `self / other`, returning [`None`]
    /// if `other` == 0.
    #[inline]
    pub const fn checked_div(self, rhs: u32) -> Option<UTCTimestamp> {
        match self.0.checked_div(rhs) {
            Some(duration) => Some(UTCTimestamp(duration)),
            None => None,
        }
    }
}

impl From<Duration> for UTCTimestamp {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

impl From<UTCDay> for UTCTimestamp {
    #[inline]
    fn from(day: UTCDay) -> Self {
        UTCTimestamp::from_day(day)
    }
}

impl Add for UTCTimestamp {
    type Output = UTCTimestamp;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding timestamps")
    }
}

impl Add<Duration> for UTCTimestamp {
    type Output = UTCTimestamp;

    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add_duration(rhs)
            .expect("overflow when adding timestamps")
    }
}

impl AddAssign for UTCTimestamp {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl AddAssign<Duration> for UTCTimestamp {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs
    }
}

impl Sub for UTCTimestamp {
    type Output = UTCTimestamp;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting timestamps")
    }
}

impl Sub<Duration> for UTCTimestamp {
    type Output = UTCTimestamp;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub_duration(rhs)
            .expect("overflow when subtracting timestamps")
    }
}

impl SubAssign for UTCTimestamp {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<Duration> for UTCTimestamp {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for UTCTimestamp {
    type Output = UTCTimestamp;

    fn mul(self, rhs: u32) -> Self::Output {
        self.checked_mul(rhs)
            .expect("overflow when multiplying timestamp by scalar")
    }
}

impl Mul<UTCTimestamp> for u32 {
    type Output = UTCTimestamp;

    fn mul(self, rhs: UTCTimestamp) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<u32> for UTCTimestamp {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs
    }
}

impl Div<u32> for UTCTimestamp {
    type Output = UTCTimestamp;

    fn div(self, rhs: u32) -> Self::Output {
        self.checked_div(rhs)
            .expect("divide by zero error when dividing timestamp by scalar")
    }
}

impl DivAssign<u32> for UTCTimestamp {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs
    }
}

/// Common methods for creating and converting between UTC structures.
///
/// ## Examples
#[cfg_attr(not(feature = "std"), doc = "```rust,ignore")]
#[cfg_attr(feature = "std", doc = "```rust")]
/// use core::time::Duration;
///
/// use utc_dt::UTCDatetime;
/// use utc_dt::time::{
///     UTCTimestamp,
///     UTCDay,
///     UTCTimeOfDay,
///     UTCTransformations,
/// };
/// use utc_dt::date::UTCDate;
///
/// // An example duration.
/// // When a duration is used, it is assumed to be relative to the unix epoch.
/// // Thursday, 15 June 2023 10:18:08.903
/// let example_duration = Duration::from_millis(1686824288903);
/// // UTC Timestamp from a duration
/// let utc_timestamp = UTCTimestamp::from(example_duration);
///
/// // Example shortcuts using `UTCTransformations`
/// // UTC Day / UTC Date / UTC Datetime from a duration
/// let utc_day = UTCDay::from_duration(example_duration); // OR
/// let utc_day = UTCDay::from(example_duration);
/// let utc_date = UTCDate::from_duration(example_duration); // OR
/// let utc_date = UTCDate::from(example_duration);
/// let utc_datetime = UTCDatetime::from_duration(example_duration); // OR
/// let utc_datetime = UTCDatetime::from(example_duration);
///
/// // UTC Day / UTC Date / UTC Datetime from a timestamp
/// let utc_day = UTCDay::from_timestamp(utc_timestamp); // OR
/// let utc_day = UTCDay::from(utc_timestamp);
/// let utc_date = UTCDate::from_timestamp(utc_timestamp); // OR
/// let utc_date = UTCDate::from(utc_timestamp);
/// let utc_datetime = UTCDatetime::from_timestamp(utc_timestamp); // OR
/// let utc_datetime = UTCDatetime::from(utc_timestamp);
///
/// // UTC Day / UTC Date / UTC Datetime from local system time
/// // Not available for #![no_std]
/// let utc_day = UTCDay::try_from_system_time().unwrap();
/// let utc_date = UTCDate::try_from_system_time().unwrap();
/// let utc_datetime = UTCDatetime::try_from_system_time().unwrap();
///
/// // UTC Day / UTC Date / UTC Datetime from u64 epoch measurements
/// let utc_day = UTCDay::from_secs(1686824288);
/// let utc_date = UTCDate::from_millis(1686824288_000);
/// let utc_datetime = UTCDate::from_micros(1686824288_000_000);
///
/// // Convert from UTC Day / UTC Date / UTC Datetime back to various types
/// let utc_duration: Duration = utc_day.as_duration();
/// let utc_timestamp: UTCTimestamp = utc_date.as_timestamp();
/// let utc_secs: u64 = utc_date.as_secs();
/// let utc_millis: u128 = utc_datetime.as_millis();
/// let utc_micros: u128 = utc_day.as_micros();
/// let utc_nanos: u128 = utc_date.as_nanos();
/// ```
///
pub trait UTCTransformations
where
    Self: Sized,
{
    /// Create from a duration measured from the Unix Epoch.
    #[inline]
    fn from_duration(duration: Duration) -> Self {
        let timestamp = UTCTimestamp(duration);
        Self::from_timestamp(timestamp)
    }

    /// Convert to a duration measured from the Unix Epoch.
    #[inline]
    fn as_duration(&self) -> Duration {
        self.as_timestamp().as_duration()
    }

    /// Create from seconds measured from the Unix Epoch.
    #[inline]
    fn from_secs(secs: u64) -> Self {
        let timestamp = UTCTimestamp::from_secs(secs);
        Self::from_timestamp(timestamp)
    }

    /// Convert to seconds measured from the Unix Epoch.
    #[inline]
    fn as_secs(&self) -> u64 {
        self.as_timestamp().as_secs()
    }

    /// Create from milliseconds measured from the Unix Epoch.
    #[inline]
    fn from_millis(millis: u64) -> Self {
        let timestamp = UTCTimestamp::from_millis(millis);
        Self::from_timestamp(timestamp)
    }

    /// Convert to milliseconds measured from the Unix Epoch.
    #[inline]
    fn as_millis(&self) -> u128 {
        self.as_timestamp().as_millis()
    }

    /// Create from microseconds measured from the Unix Epoch.
    #[inline]
    fn from_micros(micros: u64) -> Self {
        let timestamp = UTCTimestamp::from_micros(micros);
        Self::from_timestamp(timestamp)
    }

    /// Convert to microseconds measured from the Unix Epoch.
    #[inline]
    fn as_micros(&self) -> u128 {
        self.as_timestamp().as_micros()
    }

    /// Create from nanoseconds measured from the Unix Epoch.
    #[inline]
    fn from_nanos(nanos: u64) -> Self {
        let timestamp = UTCTimestamp::from_nanos(nanos);
        Self::from_timestamp(timestamp)
    }

    /// Convert to nanoseconds measured from the Unix Epoch.
    #[inline]
    fn as_nanos(&self) -> u128 {
        self.as_timestamp().as_nanos()
    }

    /// Create from local system time
    #[cfg(feature = "std")]
    fn try_from_system_time() -> Result<Self, SystemTimeError> {
        let timestamp = UTCTimestamp::try_from_system_time()?;
        Ok(Self::from_timestamp(timestamp))
    }

    /// Create from a UTC timestamp.
    fn from_timestamp(timestamp: UTCTimestamp) -> Self;
    /// Convert to a UTC timestamp.
    fn as_timestamp(&self) -> UTCTimestamp;
}

/// UTC Day count.
///
/// UTC Day is equal to the number of days since the Unix Epoch.
///
/// ## Examples
#[cfg_attr(not(feature = "std"), doc = "```rust,ignore")]
#[cfg_attr(feature = "std", doc = "```rust")]
/// use utc_dt::time::UTCDay;
///
/// // UTC Day from an integer
/// let utc_day = UTCDay::try_from_u64(19523).unwrap();
/// // Integer from UTC Day
/// let day_u64 = utc_day.as_u64(); // OR
/// let day_u64 = utc_day.to_u64();
/// // Use UTC Day to get the weekday
/// let weekday = utc_day.as_weekday();
/// // Manipulate UTC Days with standard math operators
/// assert_eq!(utc_day - utc_day, utc_day / u64::MAX);
/// assert_eq!(utc_day + 19523, utc_day * 2);
/// ```
///
/// ## Safety
/// Unchecked methods are provided for use in hot paths requiring high levels of optimisation.
/// These methods assume valid input.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UTCDay(u64);

impl UTCDay {
    /// The zero UTC day value.
    ///
    /// Equal to the epoch day.
    pub const ZERO: Self = Self(0);

    /// The maximum time of day value.
    ///
    /// Maximum day support is limited by the maximum `UTCTimestamp`.
    pub const MAX: Self = Self(213_503_982_334_601);

    /// Create UTC Day from integer
    ///
    /// ## Safety
    /// Unsafe if the user passes an unsupported count of UTC days, exceeding `UTCDay::MAX`.
    #[inline]
    pub const unsafe fn from_u64_unchecked(u: u64) -> Self {
        Self(u)
    }

    /// Try create UTC Day from integer.
    pub fn try_from_u64(u: u64) -> Result<Self, UTCDayErrOutOfRange> {
        let day = unsafe { Self::from_u64_unchecked(u) };
        if day > Self::MAX {
            return Err(UTCDayErrOutOfRange(day.0));
        }
        Ok(day)
    }

    /// UTC Day as internal integer
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    /// Consume UTC Day to internal integer
    #[inline]
    pub const fn to_u64(self) -> u64 {
        self.0
    }

    /// Calculate and return the day of the week in numerical form
    /// `[0, 6]` represents `[Sun, Sat]`
    ///
    /// Reference:
    /// <http://howardhinnant.github.io/date_algorithms.html#weekday_from_days>
    pub fn as_weekday(&self) -> u8 {
        ((self.0 + 4) % 7) as u8
    }

    /// Checked `UTCDay` addition. Computes `self + other`, returning [`None`]
    /// if overflow occurred.
    #[inline]
    pub fn checked_add(self, rhs: UTCDay) -> Option<UTCDay> {
        self.0
            .checked_add(rhs.0)
            .map(|u| UTCDay(u).min(UTCDay::MAX))
    }

    /// Checked `UTCDay` addition with `u64`. Computes `self + other`, returning [`None`]
    /// if overflow occurred.
    #[inline]
    pub fn checked_add_u64(self, rhs: u64) -> Option<UTCDay> {
        self.0.checked_add(rhs).map(|u| UTCDay(u).min(UTCDay::MAX))
    }

    /// Saturating `UTCDay` addition. Computes `self + other`, returning [`UTCDay::MAX`]
    /// if overflow occurred.
    #[inline]
    pub fn saturating_add(self, rhs: UTCDay) -> UTCDay {
        match self.checked_add(rhs) {
            Some(res) => res,
            None => UTCDay::MAX,
        }
    }

    /// Saturating `UTCDay` addition with `u64`. Computes `self + other`, returning [`UTCDay::MAX`]
    /// if overflow occurred.
    #[inline]
    pub fn saturating_add_u64(self, rhs: u64) -> UTCDay {
        match self.checked_add_u64(rhs) {
            Some(res) => res,
            None => UTCDay::MAX,
        }
    }

    /// Checked `UTCDay` subtraction. Computes `self - other`, returning [`None`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn checked_sub(self, rhs: UTCDay) -> Option<UTCDay> {
        match self.0.checked_sub(rhs.0) {
            Some(u) => Some(UTCDay(u)),
            None => None,
        }
    }

    /// Checked `UTCDay` subtraction with `u64`. Computes `self - other`, returning [`None`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn checked_sub_u64(self, rhs: u64) -> Option<UTCDay> {
        match self.0.checked_sub(rhs) {
            Some(u) => Some(UTCDay(u)),
            None => None,
        }
    }

    /// Saturating `UTCDay` subtraction. Computes `self - other`, returning [`UTCDay::ZERO`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn saturating_sub(self, rhs: UTCDay) -> UTCDay {
        match self.checked_sub(rhs) {
            Some(res) => res,
            None => UTCDay::ZERO,
        }
    }

    /// Saturating `UTCDay` subtraction with `u64`. Computes `self - other`, returning [`UTCDay::ZERO`]
    /// if the result would be negative or if overflow occurred.
    #[inline]
    pub const fn saturating_sub_u64(self, rhs: u64) -> UTCDay {
        match self.checked_sub_u64(rhs) {
            Some(res) => res,
            None => UTCDay::ZERO,
        }
    }

    /// Checked `UTCDay` multiplication. Computes `self * other`, returning
    /// [`None`] if overflow occurred.
    #[inline]
    pub fn checked_mul(self, rhs: u64) -> Option<UTCDay> {
        self.0.checked_mul(rhs).map(|u| UTCDay(u).min(UTCDay::MAX))
    }

    /// Saturating `UTCDay` multiplication. Computes `self * other`, returning
    /// [`UTCDay::MAX`] if overflow occurred.
    #[inline]
    pub fn saturating_mul(self, rhs: u64) -> UTCDay {
        match self.checked_mul(rhs) {
            Some(res) => res,
            None => UTCDay::MAX,
        }
    }

    /// Checked `UTCDay` division. Computes `self / other`, returning [`None`]
    /// if `other` == 0.
    #[inline]
    pub const fn checked_div(self, rhs: u64) -> Option<UTCDay> {
        match self.0.checked_div(rhs) {
            Some(u) => Some(UTCDay(u)),
            None => None,
        }
    }
}

/// Error type for UTCDay out of range
#[derive(Debug, Clone)]
pub struct UTCDayErrOutOfRange(u64);

impl Display for UTCDayErrOutOfRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "UTC day ({}) exceeding maximum", self.0)
    }
}

impl Error for UTCDayErrOutOfRange {}

impl UTCTransformations for UTCDay {
    #[inline]
    fn from_secs(secs: u64) -> Self {
        Self(secs / SECONDS_PER_DAY)
    }

    #[inline]
    fn as_secs(&self) -> u64 {
        self.0 * SECONDS_PER_DAY
    }

    #[inline]
    fn from_millis(millis: u64) -> Self {
        Self(millis / MILLIS_PER_DAY)
    }

    #[inline]
    fn as_millis(&self) -> u128 {
        self.0 as u128 * MILLIS_PER_DAY as u128
    }

    #[inline]
    fn from_micros(micros: u64) -> Self {
        Self(micros / MICROS_PER_DAY)
    }

    #[inline]
    fn as_micros(&self) -> u128 {
        self.0 as u128 * MICROS_PER_DAY as u128
    }

    #[inline]
    fn from_nanos(nanos: u64) -> Self {
        Self(nanos / NANOS_PER_DAY)
    }

    #[inline]
    fn as_nanos(&self) -> u128 {
        self.0 as u128 * NANOS_PER_DAY as u128
    }

    #[inline]
    fn from_timestamp(timestamp: UTCTimestamp) -> Self {
        timestamp.as_day()
    }

    #[inline]
    fn as_timestamp(&self) -> UTCTimestamp {
        UTCTimestamp::from_day(*self)
    }
}

impl Add for UTCDay {
    type Output = UTCDay;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).expect("overflow when adding days")
    }
}

impl Add<u64> for UTCDay {
    type Output = UTCDay;

    fn add(self, rhs: u64) -> Self::Output {
        self.checked_add_u64(rhs)
            .expect("overflow when adding days")
    }
}

impl AddAssign for UTCDay {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl AddAssign<u64> for UTCDay {
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs
    }
}

impl Sub for UTCDay {
    type Output = UTCDay;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting days")
    }
}

impl Sub<u64> for UTCDay {
    type Output = UTCDay;

    fn sub(self, rhs: u64) -> Self::Output {
        self.checked_sub_u64(rhs)
            .expect("overflow when subtracting days")
    }
}

impl SubAssign for UTCDay {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<u64> for UTCDay {
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

impl Mul<u64> for UTCDay {
    type Output = UTCDay;

    fn mul(self, rhs: u64) -> Self::Output {
        self.checked_mul(rhs)
            .expect("overflow when multiplying day by scalar")
    }
}

impl Mul<UTCDay> for u64 {
    type Output = UTCDay;

    fn mul(self, rhs: UTCDay) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<u64> for UTCDay {
    fn mul_assign(&mut self, rhs: u64) {
        *self = *self * rhs
    }
}

impl Div<u64> for UTCDay {
    type Output = UTCDay;

    fn div(self, rhs: u64) -> Self::Output {
        self.checked_div(rhs)
            .expect("divide by zero error when dividing day by scalar")
    }
}

impl DivAssign<u64> for UTCDay {
    fn div_assign(&mut self, rhs: u64) {
        *self = *self / rhs
    }
}

impl TryFrom<u64> for UTCDay {
    type Error = UTCDayErrOutOfRange;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::try_from_u64(value)
    }
}

impl From<Duration> for UTCDay {
    #[inline]
    fn from(duration: Duration) -> Self {
        Self::from_duration(duration)
    }
}

impl From<UTCTimestamp> for UTCDay {
    #[inline]
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_timestamp(timestamp)
    }
}

/// UTC Time of Day
///
/// A time of day measurement with nanosecond resolution.
///
/// ## Examples
#[cfg_attr(not(feature = "std"), doc = "```rust,ignore")]
#[cfg_attr(feature = "std", doc = "```rust")]
/// use utc_dt::time::UTCTimeOfDay;
///
/// // UTC Time of Day from a time measurement (for secs, millis, micros, nanos)
/// let utc_tod = UTCTimeOfDay::try_from_millis(37088903).unwrap(); // OR
/// let utc_tod = unsafe { UTCTimeOfDay::from_millis_unchecked(37088903) };
/// // UTC Time of Day from hours, minutes, seconds and subseconds
/// let utc_tod = UTCTimeOfDay::try_from_hhmmss(10, 18, 08, 903_000_000).unwrap(); // OR
/// let utc_tod = unsafe { UTCTimeOfDay::from_hhmmss_unchecked(10, 18, 08, 903_000_000) };
/// // UTC Time of Day as a time measurement (for secs, millis, micros, nanos)
/// let utc_tod_us = utc_tod.as_micros();
/// // UTC Time of Day as hours, minutes and seconds
/// let (hrs, mins, secs) = utc_tod.as_hhmmss();
/// // UTC Time of Day subsecond component (in nanoseconds)
/// let subsec_ns = utc_tod.as_subsec_ns();
/// // Parse a UTC Time of Day from an ISO 8601 time string `(Thh:mm:ssZ)`
/// let utc_tod = UTCTimeOfDay::try_from_iso_tod("T10:18:08.903Z").unwrap();
/// // Get a time of day string formatted according to ISO 8601 `(Thh:mm:ssZ)`
/// const PRECISION_MICROS: usize = 6;
/// let iso_tod = utc_tod.as_iso_tod(PRECISION_MICROS);
/// assert_eq!(iso_tod, "T10:18:08.903000Z");
/// // Write ISO 8601 time of day str to a stack buffer
/// let mut buf = [0; UTCTimeOfDay::iso_tod_len(PRECISION_MICROS)];
/// let _bytes_written = utc_tod.write_iso_tod(&mut buf, PRECISION_MICROS).unwrap();
/// let iso_tod_str = core::str::from_utf8(&buf).unwrap();
/// assert_eq!(iso_tod_str, "T10:18:08.903000Z");
/// ```
///
/// ## Safety
/// Unchecked methods are provided for use in hot paths requiring high levels of optimisation.
/// These methods assume valid input.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UTCTimeOfDay(u64);

impl Display for UTCTimeOfDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let (hrs, mins, secs) = self.as_hhmmss();
        write!(
            f,
            "T{:02}:{:02}:{:02}.{:09}Z",
            hrs,
            mins,
            secs,
            self.as_subsec_ns()
        )
    }
}

impl UTCTimeOfDay {
    /// The zero time of day value
    pub const ZERO: Self = Self(0);

    /// The maximum time of day value.
    ///
    /// Equal to the number of nanoseconds in a day.
    pub const MAX: Self = Self(NANOS_PER_DAY - 1);

    /// The minimum length of an ISO time (in UTF8 characters)
    pub const MIN_ISO_TOD_LEN: usize = 10;

    /// The maximum supported subsecond precision of an ISO time
    pub const MAX_ISO_TOD_PRECISION: usize = 9;

    /// Unchecked method to create time of day from nanoseconds
    ///
    /// ### Safety
    /// Unsafe if the user passes an invalid time-of-day nanoseconds component (exceeding NANOS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_nanos_unchecked(nanos: u64) -> Self {
        Self(nanos)
    }

    /// Unchecked method to create time of day from microseconds
    ///
    /// ### Safety
    /// Unsafe if the user passes an invalid time-of-day microsecond component (exceeding MICROS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_micros_unchecked(micros: u64) -> Self {
        Self(micros * NANOS_PER_MICRO)
    }

    /// Unchecked method to create time of day from milliseconds
    ///
    /// ### Safety
    /// Unsafe if the user passes an invalid time-of-day millisecond component (exceeding MILLIS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_millis_unchecked(millis: u32) -> Self {
        Self((millis as u64) * NANOS_PER_MILLI)
    }

    /// Unchecked method to create time of day from seconds
    ///
    /// ### Safety
    /// Unsafe if the user passes an invalid time-of-day seconds component (exceeding SECONDS_PER_DAY).
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_secs_unchecked(secs: u32) -> Self {
        Self((secs as u64) * NANOS_PER_SECOND)
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
    pub fn try_from_nanos(nanos: u64) -> Result<Self, UTCTimeOfDayError> {
        // SAFETY: we immediately check that nanos was within NANOS_PER_DAY (tod does not exceed UTCTimeOfDay::MAX)
        let tod = unsafe { Self::from_nanos_unchecked(nanos) };
        if tod > Self::MAX {
            return Err(UTCTimeOfDayError::ExcessNanos(nanos));
        }
        Ok(tod)
    }

    /// Try to create UTC time of day from microseconds
    pub fn try_from_micros(micros: u64) -> Result<Self, UTCTimeOfDayError> {
        // SAFETY: we immediately check that micros was within MICROS_PER_DAY (tod does not exceed UTCTimeOfDay::MAX)
        let tod = unsafe { Self::from_micros_unchecked(micros) };
        if tod > Self::MAX {
            return Err(UTCTimeOfDayError::ExcessMicros(micros));
        }
        Ok(tod)
    }

    /// Try to create UTC time of day from milliseconds
    pub fn try_from_millis(millis: u32) -> Result<Self, UTCTimeOfDayError> {
        // SAFETY: we immediately check that millis was within MILLIS_PER_DAY (tod does not exceed UTCTimeOfDay::MAX)
        let tod = unsafe { Self::from_millis_unchecked(millis) };
        if tod > Self::MAX {
            return Err(UTCTimeOfDayError::ExcessMillis(millis));
        }
        Ok(tod)
    }

    /// Try to create UTC time of day from seconds
    pub fn try_from_secs(secs: u32) -> Result<Self, UTCTimeOfDayError> {
        // SAFETY: we immediately check that secs was within SECONDS_PER_DAY (tod does not exceed UTCTimeOfDay::MAX)
        let tod = unsafe { Self::from_secs_unchecked(secs) };
        if tod > Self::MAX {
            return Err(UTCTimeOfDayError::ExcessSeconds(secs));
        }
        Ok(tod)
    }

    /// Try to create UTC time of day from hours, minutes, seconds and subsecond (nanosecond) components
    ///
    /// Inputs are not limited by divisions. eg. 61 minutes is valid input, 61 seconds, etc.
    /// The time described must not exceed the number of nanoseconds in a day.
    pub fn try_from_hhmmss(
        hrs: u8,
        mins: u8,
        secs: u8,
        subsec_ns: u32,
    ) -> Result<Self, UTCTimeOfDayError> {
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
    /// Returns tuple `(hrs: u8, mins: u8, secs: u8)`
    pub const fn as_hhmmss(&self) -> (u8, u8, u8) {
        let hrs = (self.0 / NANOS_PER_HOUR) as u8;
        let mins = ((self.0 % NANOS_PER_HOUR) / NANOS_PER_MINUTE) as u8;
        let secs = ((self.0 % NANOS_PER_MINUTE) / NANOS_PER_SECOND) as u8;
        (hrs, mins, secs)
    }

    /// Return subsecond component of time of day (in nanoseconds)
    #[inline]
    pub const fn as_subsec_ns(&self) -> u32 {
        (self.0 % NANOS_PER_SECOND) as u32
    }

    /// Time of day from UTC timestamp
    pub const fn from_timestamp(timestamp: UTCTimestamp) -> Self {
        timestamp.as_tod()
    }

    /// Try parse time-of-day from an ISO str in the format:
    /// * `Thh:mm:ssZ`
    /// * `Thh:mm:ss.nnnZ` (up to 9 decimal places)
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    pub fn try_from_iso_tod(iso: &str) -> Result<Self, UTCTimeOfDayError> {
        let len = iso.len();
        if len < Self::MIN_ISO_TOD_LEN {
            return Err(UTCTimeOfDayError::InsufficientStrLen(
                len,
                Self::MIN_ISO_TOD_LEN,
            ));
        }
        let (hour_str, rem) = iso[1..].split_at(2); // remainder = ":mm:ss.nnnZ"
        let (minute_str, rem) = rem[1..].split_at(2); // remainder = ":ss.nnnZ"
        let (second_str, rem) = rem[1..].split_at(2); // remainder = ".nnnZ"
        let hrs: u8 = hour_str.parse()?;
        let mins: u8 = minute_str.parse()?;
        let secs: u8 = second_str.parse()?;
        // calculate subseconds
        let rem_len = rem.len();
        let subsec_ns: u32 = if rem_len > 1 {
            let subsec_str = &rem[1..(rem_len - 1)]; // "nnn"
            let precision: u32 = subsec_str.len() as u32;
            if precision > Self::MAX_ISO_TOD_PRECISION as u32 {
                return Err(UTCTimeOfDayError::ExcessPrecision(precision));
            }
            if precision == 0 {
                0
            } else {
                let subsec: u32 = subsec_str.parse()?;
                subsec * 10u32.pow(Self::MAX_ISO_TOD_PRECISION as u32 - precision)
            }
        } else {
            0
        };
        Self::try_from_hhmmss(hrs, mins, secs, subsec_ns)
    }

    /// Return time-of-day as a string in the format:
    /// * Precision = `0`: `Thh:mm:ssZ`
    /// * Precision = `3`: `Thh:mm:ss.nnnZ`
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    #[cfg(feature = "alloc")]
    pub fn as_iso_tod(&self, precision: usize) -> String {
        let len = Self::iso_tod_len(precision);
        let mut s = format!("{self}");
        s.truncate(len - 1);
        s.push('Z');
        s
    }

    /// Internal truncated buffer write
    #[inline]
    pub(crate) fn _write_iso_tod_trunc(&self, w: &mut StrWriter) {
        // unwrap infallible
        write!(w, "{self}").unwrap();
        w.buf[w.written - 1] = b'Z';
    }

    /// Write time-of-day to a buffer in the format:
    /// * Precision = `0`: `Thh:mm:ssZ`
    /// * Precision = `3`: `Thh:mm:ss.nnnZ`
    ///
    /// The buffer should have a minimum length as given by [UTCTimeOfDay::iso_tod_len].
    ///
    /// A buffer of insufficient length will error ([UTCTimeOfDayError::InsufficientStrLen]).
    ///
    /// Returns number of UTF8 characters (bytes) written
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    pub fn write_iso_tod(
        &self,
        buf: &mut [u8],
        precision: usize,
    ) -> Result<usize, UTCTimeOfDayError> {
        let write_len = Self::iso_tod_len(precision);
        if write_len > buf.len() {
            return Err(UTCTimeOfDayError::InsufficientStrLen(buf.len(), write_len));
        }
        let mut writer = StrWriter::new(&mut buf[..write_len]);
        self._write_iso_tod_trunc(&mut writer);
        Ok(writer.written)
    }

    /// Calculate the number of characters in an ISO time-of-day str
    #[inline]
    pub const fn iso_tod_len(precision: usize) -> usize {
        if precision == 0 {
            Self::MIN_ISO_TOD_LEN
        } else if precision < Self::MAX_ISO_TOD_PRECISION {
            Self::MIN_ISO_TOD_LEN + precision + 1
        } else {
            // clamp to precision to max
            Self::MIN_ISO_TOD_LEN + Self::MAX_ISO_TOD_PRECISION + 1
        }
    }
}

/// Error type for UTCTimeOfDay methods
#[derive(Debug, Clone)]
pub enum UTCTimeOfDayError {
    /// Error raised parsing int to string
    ParseErr(ParseIntError),
    /// Error raised due to excessive ISO precision
    ExcessPrecision(u32),
    /// Error raised due to nanos exceeding nanos in a day
    ExcessNanos(u64),
    /// Error raised due to micros exceeding micros in a day
    ExcessMicros(u64),
    /// Error raised due to millis exceeding millis in a day
    ExcessMillis(u32),
    /// Error raised due to seconds exceeding seconds in a day
    ExcessSeconds(u32),
    /// Error raised due to insufficient length of input ISO time-of-day str
    InsufficientStrLen(usize, usize),
}

impl Display for UTCTimeOfDayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ParseErr(e) => e.fmt(f),
            Self::ExcessPrecision(p) => write!(f, "ISO precision ({p}) exceeds maximum of 9"),
            Self::ExcessNanos(n) => write!(f, "nanoseconds ({n}) not within a day"),
            Self::ExcessMicros(u) => write!(f, "microseconds ({u}) not within a day"),
            Self::ExcessMillis(m) => write!(f, "milliseconds ({m}) not within a day"),
            Self::ExcessSeconds(s) => write!(f, "seconds ({s}) not within a day"),
            Self::InsufficientStrLen(l, m) => {
                write!(f, "insufficient ISO time str len ({l}), {m} required")
            }
        }
    }
}

impl Error for UTCTimeOfDayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseErr(e) => e.source(),
            _ => None,
        }
    }
}

impl From<ParseIntError> for UTCTimeOfDayError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseErr(value)
    }
}
