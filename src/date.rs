//! Date module.
//!
//! Implements transformations required for the
//! proleptic Gregorian Calendar (the *civil* calendar),
//! to create UTC dates.

use core::{time::Duration, fmt::{Display, Formatter}};

use anyhow::{Result, bail};

use crate::time::{UTCDay, UTCTimestamp, UTCTransformations};

/// UTC Date.
///
/// A UTC Date is any calendar date since the Unix epoch date (inclusive).
///
/// ## Examples
/// ```rust,ignore
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
/// ```
///
/// ## Safety
/// Unchecked methods are provided for use in hot paths requiring high levels of optimisation.
/// These methods assume valid input.
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
    pub const MIN: Self = Self { era: 4, yoe: 370, month: 1, day: 1 };

    /// The maximum UTC Date supported.
    ///
    /// Equal to `November 9, 584_554_051_223`
    ///
    /// Maximum date support is limited by the maximum `UTCTimestamp`.
    /// UTCDate can physically store dates up to `December 31, 1_717_986_918_399`
    pub const MAX: Self = Self { era: 1_461_385_128, yoe: 23, month: 11, day: 9 };

    /// The maximum year supported
    pub const MAX_YEAR: u64 = 584_554_051_223;

    /// The minimum year supported
    pub const MIN_YEAR: u64 = 1970;

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
        Self { era: era as u32, yoe: yoe as u16, month, day }
    }

    /// Try to create a UTC Date from provided year, month and day.
    pub fn try_from_components(year: u64, month: u8, day: u8) -> Result<Self> {
        if year < Self::MIN_YEAR || year > Self::MAX_YEAR {
            bail!("Year out of range! (year: {:04})", year);
        }
        if month == 0 || month > 12 {
            bail!("Month out of range! (month: {:02})", month);
        }
        // force create
        let date = unsafe { Self::from_components_unchecked(year, month, day) };
        // then check
        if date.day == 0 || date.day > date.days_in_month() {
            bail!("Day out of range! (date: {date}");
        }
        if date > UTCDate::MAX {
            bail!("Date out of range! (date: {date}");
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
        Self { era, yoe: yoe as u16, month, day }
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
        let days = (era as u64 * 146097)  + doe as u64 - 719468;
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
        let year = self.yoe as u64 + (self.era as u64 * 400) + (self.month <=2) as u64;
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
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Month out of range! {:2}", self.month),
        }
    }

    /// Try parse date from string in the format:
    /// * `YYYY-MM-DD`
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    pub fn try_from_iso_date(iso: &str) -> Result<Self> {
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
    #[cfg(feature = "std")]
    pub fn as_iso_date(&self) -> String {
        format!("{self}")
    }
}

impl UTCTransformations for UTCDate {
    fn from_secs(s: u64) -> Self {
        let utc_day = UTCDay::from_secs(s);
        Self::from_day(utc_day)
    }

    fn as_secs(&self) -> u64 {
        self.as_day().as_secs()
    }

    fn from_millis(s: u64) -> Self {
        let utc_day = UTCDay::from_millis(s);
        Self::from_day(utc_day)
    }

    fn as_millis(&self) -> u128 {
        self.as_day().as_millis()
    }

    fn from_micros(s: u64) -> Self {
        let utc_day = UTCDay::from_micros(s);
        Self::from_day(utc_day)
    }

    fn as_micros(&self) -> u128 {
        self.as_day().as_micros()
    }

    fn from_nanos(s: u64) -> Self {
        let utc_day = UTCDay::from_nanos(s);
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
