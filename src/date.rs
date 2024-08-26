//! Date module.
//!
//! Implements transformations required for the
//! proleptic Gregorian Calendar (the *civil* calendar),
//! to create UTC dates.

use crate::time::{UTCDay, UTCTimestamp, UTCTransformations};
use crate::util::StrWriter;
use core::fmt::{Display, Formatter, Write};
use core::num::ParseIntError;
use core::time::Duration;

#[cfg(feature = "alloc")]
use alloc::{format, string::String};

// TODO <https://github.com/rust-lang/rust/issues/103765>
#[cfg(feature = "nightly")]
use core::error::Error;
#[cfg(all(feature = "std", not(feature = "nightly")))]
use std::error::Error;

/// UTC Date.
///
/// A UTC Date is any calendar date since the Unix epoch date (inclusive).
///
/// ## Examples
#[cfg_attr(not(feature = "std"), doc = "```rust,ignore")]
#[cfg_attr(feature = "std", doc = "```rust")]
/// use utc_dt::time::UTCDay;
/// use utc_dt::date::UTCDate;
///
/// // UTC Day from an integer
/// let utc_day = UTCDay::try_from_u64(19523).unwrap();
///
/// // UTC Date directly from components
/// let utc_date = UTCDate::try_from_components(2023, 6, 15).unwrap(); // OR
/// let utc_date = unsafe { UTCDate::from_components_unchecked(2023, 6, 15) };
/// // UTC Date from UTC Day
/// let utc_date = UTCDate::from_day(utc_day);
/// // Check whether date occurs within leap year
/// let is_leap_year: bool = utc_date.is_leap_year();
/// // Get number of days within date's month
/// let days_in_month: u8 = utc_date.days_in_month();
/// // Get the date in integer forms
/// let (year, month, day) = utc_date.as_components();
/// // UTC Day from UTC Date
/// let utc_day = utc_date.as_day();
/// // Parse a UTC Date from an ISO 8601 date string `(YYYY-MM-DD)`
/// let utc_date = UTCDate::try_from_iso_date("2023-06-15").unwrap();
/// // Get date string formatted according to ISO 8601 `(YYYY-MM-DD)`
/// // Not available for #![no_std]
/// let iso_date = utc_date.as_iso_date();
/// assert_eq!(iso_date, "2023-06-15");
/// // Write ISO 8601 date str to a stack buffer
/// let mut buf = [0; UTCDate::ISO_DATE_LEN];
/// let _bytes_written = utc_date.write_iso_date(&mut buf).unwrap();
/// let iso_date_str = core::str::from_utf8(&buf).unwrap();
/// assert_eq!(iso_date_str, "2023-06-15");
/// ```
///
/// ## Safety
/// Unchecked methods are provided for use in hot paths requiring high levels of optimisation.
/// These methods assume valid input.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UTCDate {
    era: u32,
    yoe: u16,
    month: u8,
    day: u8,
}

impl Default for UTCDate {
    fn default() -> Self {
        Self::MIN
    }
}

impl Display for UTCDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let (year, month, day) = self.as_components();
        write!(f, "{:04}-{:02}-{:02}", year, month, day)
    }
}

impl UTCDate {
    /// The minimum UTC Date supported
    ///
    /// Equal to the epoch at Jan 1, 1970.
    pub const MIN: Self = Self {
        era: 4,
        yoe: 369,
        month: 1,
        day: 1,
    };

    /// The maximum UTC Date supported.
    ///
    /// Equal to `November 9, 584_554_051_223`
    ///
    /// Maximum date support is limited by the maximum `UTCTimestamp`.
    /// UTCDate can physically store dates up to `December 31, 1_717_986_918_399`
    pub const MAX: Self = Self {
        era: 1_461_385_128,
        yoe: 23,
        month: 11,
        day: 9,
    };

    /// The maximum year supported
    pub const MAX_YEAR: u64 = 584_554_051_223;

    /// The minimum year supported
    pub const MIN_YEAR: u64 = 1970;

    /// The length of an ISO date (in characters)
    pub const ISO_DATE_LEN: usize = 10;

    /// Unchecked method to create a UTC Date from provided year, month and day.
    ///
    /// ## Safety
    /// Unsafe if the user passes an invalid calendar year, month and day combination.
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_components_unchecked(year: u64, month: u8, day: u8) -> Self {
        let year = year - (month <= 2) as u64;
        let era = year / 400;
        let yoe = year - (era * 400);
        Self {
            era: era as u32,
            yoe: yoe as u16,
            month,
            day,
        }
    }

    /// Try to create a UTC Date from provided year, month and day.
    pub fn try_from_components(year: u64, month: u8, day: u8) -> Result<Self, UTCDateError> {
        if !(Self::MIN_YEAR..=Self::MAX_YEAR).contains(&year) {
            return Err(UTCDateError::YearOutOfRange(year));
        }
        if month == 0 || month > 12 {
            return Err(UTCDateError::MonthOutOfRange(month));
        }
        // SAFETY: we have checked year and month are within range
        let date = unsafe { Self::from_components_unchecked(year, month, day) };
        // Then check days
        if date.day == 0 || date.day > date.days_in_month() {
            return Err(UTCDateError::DayOutOfRange(date));
        }
        if date > UTCDate::MAX {
            return Err(UTCDateError::DateOutOfRange(date));
        }
        Ok(date)
    }

    /// Create a UTC Date from the number of days since the epoch.
    ///
    /// Reference:
    /// <http://howardhinnant.github.io/date_algorithms.html#civil_from_days>
    ///
    /// Simplified for unsigned days/years
    pub const fn from_day(utc_day: UTCDay) -> Self {
        let z: u64 = utc_day.as_u64() + 719468;
        let era: u32 = (z / 146097) as u32;
        let doe = (z - (era as u64 * 146097)) as u32;
        let yoe = (doe - (doe / 1460) + (doe / 36524) - (doe / 146096)) / 365;
        let doy = doe - (365 * yoe) - (yoe / 4) + (yoe / 100);
        let mp = ((5 * doy) + 2) / 153;
        let day = (doy - (((153 * mp) + 2) / 5) + 1) as u8;
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
        Self {
            era,
            yoe: yoe as u16,
            month,
            day,
        }
    }

    /// Get the days since the epoch from the UTC Date
    ///
    /// Reference:
    /// <http://howardhinnant.github.io/date_algorithms.html#days_from_civil>
    ///
    /// Simplified for unsigned days/years
    pub const fn as_day(&self) -> UTCDay {
        let m = self.month as u16;
        let d = self.day as u16;
        let era = self.era;
        let yoe = self.yoe as u32;
        let doy = ((153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5) + d - 1;
        let doe = (yoe * 365) + (yoe / 4) - (yoe / 100) + doy as u32;
        let days = (era as u64 * 146097) + doe as u64 - 719468;
        // SAFETY: days is not exceeding UTCDay::MAX
        unsafe { UTCDay::from_u64_unchecked(days) }
    }

    /// Get copy of the date components as integers
    ///
    /// Returns tuple: `(year: u64, month: u8, day: u8)`
    #[inline]
    pub const fn as_components(&self) -> (u64, u8, u8) {
        let year = self.yoe as u64 + (self.era as u64 * 400) + (self.month <= 2) as u64;
        (year, self.month, self.day)
    }

    /// Consume self into date components as integers
    ///
    /// Returns tuple: `(year: u64, month: u8, day: u8)`
    #[inline]
    pub const fn to_components(self) -> (u64, u8, u8) {
        let year = self.yoe as u64 + (self.era as u64 * 400) + (self.month <= 2) as u64;
        (year, self.month, self.day)
    }

    /// Returns whether date is within a leap year.
    ///
    /// Reference:
    /// <http://howardhinnant.github.io/date_algorithms.html#is_leap>
    #[inline]
    pub const fn is_leap_year(&self) -> bool {
        let yoe_adj = self.yoe + (self.month <= 2) as u16;
        (yoe_adj % 4 == 0) && ((yoe_adj % 100 != 0) || (yoe_adj % 400 == 0))
    }

    /// Returns the number of days within the month of the date.
    /// Leap years are accounted for.
    pub fn days_in_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            _ => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
        }
    }

    /// Try parse date from str in the format:
    /// * `YYYY-MM-DD`
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    pub fn try_from_iso_date(iso: &str) -> Result<Self, UTCDateError> {
        let len = iso.len();
        if len != Self::ISO_DATE_LEN {
            return Err(UTCDateError::InvalidStrLen(len));
        }
        // handle slice
        let (year_str, rem) = iso.split_at(4); // remainder = "-MM-DD"
        let (month_str, rem) = rem[1..].split_at(2); // remainder = "-DD"
        let day_str = &rem[1..];
        // parse
        let year: u64 = year_str.parse()?;
        let month: u8 = month_str.parse()?;
        let day: u8 = day_str.parse()?;
        Self::try_from_components(year, month, day)
    }

    /// Return date as a string in the format:
    /// * `YYYY-MM-DD`
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    #[cfg(feature = "alloc")]
    pub fn as_iso_date(&self) -> String {
        format!("{self}")
    }

    /// Internal truncated buffer write
    #[inline]
    pub(crate) fn _write_iso_date_trunc(&self, w: &mut StrWriter) {
        // unwrap infallible
        write!(w, "{self}").unwrap();
    }

    /// Write an ISO date to a buffer in the format:
    /// * `YYYY-MM-DD`
    ///
    /// The buffer should have minimum length of [UTCDate::ISO_DATE_LEN] (10).
    ///
    /// A buffer of insufficient length will error ([UTCDateError::InvalidStrLen]).
    ///
    /// Returns number of UTF8 characters (bytes) written
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    pub fn write_iso_date(&self, buf: &mut [u8]) -> Result<usize, UTCDateError> {
        let write_len = Self::ISO_DATE_LEN;
        if write_len > buf.len() {
            return Err(UTCDateError::InvalidStrLen(buf.len()));
        }
        let mut writer = StrWriter::new(&mut buf[..write_len]);
        self._write_iso_date_trunc(&mut writer);
        Ok(writer.written)
    }
}

impl UTCTransformations for UTCDate {
    fn from_secs(secs: u64) -> Self {
        let utc_day = UTCDay::from_secs(secs);
        Self::from_day(utc_day)
    }

    fn as_secs(&self) -> u64 {
        self.as_day().as_secs()
    }

    fn from_millis(millis: u64) -> Self {
        let utc_day = UTCDay::from_millis(millis);
        Self::from_day(utc_day)
    }

    fn as_millis(&self) -> u128 {
        self.as_day().as_millis()
    }

    fn from_micros(micros: u64) -> Self {
        let utc_day = UTCDay::from_micros(micros);
        Self::from_day(utc_day)
    }

    fn as_micros(&self) -> u128 {
        self.as_day().as_micros()
    }

    fn from_nanos(nanos: u64) -> Self {
        let utc_day = UTCDay::from_nanos(nanos);
        Self::from_day(utc_day)
    }

    fn as_nanos(&self) -> u128 {
        self.as_day().as_nanos()
    }

    fn from_timestamp(timestamp: UTCTimestamp) -> Self {
        let utc_day = UTCDay::from_timestamp(timestamp);
        Self::from_day(utc_day)
    }

    fn as_timestamp(&self) -> UTCTimestamp {
        self.as_day().as_timestamp()
    }
}

impl From<Duration> for UTCDate {
    fn from(duration: Duration) -> Self {
        Self::from_duration(duration)
    }
}

impl From<UTCTimestamp> for UTCDate {
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_timestamp(timestamp)
    }
}

impl From<UTCDay> for UTCDate {
    fn from(utc_day: UTCDay) -> Self {
        Self::from_day(utc_day)
    }
}

/// Error type for UTCDate methods
#[derive(Debug)]
pub enum UTCDateError {
    /// Error raised parsing int to string
    ParseErr(ParseIntError),
    /// Error raised due to out of range year
    YearOutOfRange(u64),
    /// Error raised due to out of range month
    MonthOutOfRange(u8),
    /// Error raised due to out of range day
    DayOutOfRange(UTCDate),
    /// Error raised due to out of range date
    DateOutOfRange(UTCDate),
    /// Error raised due to invalid ISO date length
    InvalidStrLen(usize),
}

impl Display for UTCDateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ParseErr(e) => e.fmt(f),
            Self::YearOutOfRange(y) => write!(f, "Year ({y}) out of range!"),
            Self::MonthOutOfRange(m) => write!(f, "Month ({m}) out of range!"),
            Self::DayOutOfRange(d) => write!(f, "Day ({d}) out of range!"),
            Self::DateOutOfRange(date) => write!(f, "Date ({date}) out of range!"),
            Self::InvalidStrLen(l) => write!(f, "Invalid ISO date str length ({l}), 10 required"),
        }
    }
}

#[cfg(any(feature = "std", feature = "nightly"))]
impl Error for UTCDateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseErr(e) => e.source(),
            _ => None,
        }
    }
}

impl From<ParseIntError> for UTCDateError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseErr(value)
    }
}
