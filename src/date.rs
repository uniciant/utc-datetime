//! Date module.
//!
//! Implements transformations required for the
//! proleptic Gregorian Calendar (the *civil* calendar),
//! to create UTC dates.

use core::{time::Duration, cmp::Ordering, fmt::{Display, Formatter}};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct UTCDate {
    era: u32,
    yoe: u16,
    month: u8,
    day: u8,
}

impl Ord for UTCDate {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.era < other.era { return Ordering::Less }
        if self.era > other.era { return Ordering::Greater }
        if self.yoe < other.yoe { return Ordering::Less }
        if self.yoe > other.yoe { return Ordering::Greater }
        if self.month < other.month { return Ordering::Less }
        if self.month > other.month { return Ordering::Greater }
        if self.day < other.day { return Ordering::Less }
        if self.day > other.day { return Ordering::Greater }
        Ordering::Equal
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
    pub const MIN: UTCDate = UTCDate { era: 4, yoe: 370, month: 1, day: 1 };

    /// The maximum UTC Date supported.
    ///
    /// Equal to `November 9, 584_554_051_223`
    ///
    /// Maximum date support is limited by the maximum `UTCTimestamp`.
    /// UTCDate can physically store dates up to `December 31, 1_717_986_918_399`
    pub const MAX: UTCDate = UTCDate { era: 1_461_385_128, yoe: 23, month: 11, day: 9 };

    /// The maximum year supported
    pub const MAX_YEAR: u64 = 584_554_051_223;

    /// The minimum year supported
    pub const MIN_YEAR: u64 = 1970;

    /// Unchecked method to create a UTC Date from provided year, month and day.
    ///
    /// # Safety
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
        UTCDay::from_u64(days)
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

    fn as_millis(&self) -> u64 {
        self.as_day().as_millis()
    }

    fn from_micros(s: u64) -> Self {
        let utc_day = UTCDay::from_micros(s);
        Self::from_day(utc_day)
    }

    fn as_micros(&self) -> u64 {
        self.as_day().as_micros()
    }

    fn from_nanos(s: u64) -> Self {
        let utc_day = UTCDay::from_nanos(s);
        Self::from_day(utc_day)
    }

    fn as_nanos(&self) -> u64 {
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

#[cfg(test)]
mod test {
    use anyhow::{Result, bail};

    use crate::{date::UTCDate, constants::SECONDS_PER_DAY};

    #[test]
    fn test_date_from_components() -> Result<()> {
        let test_cases = [
            (2023, 6, 14, true, false, 30),   // valid recent date
            (1970, 1, 1, true, false, 31),    // valid epoch date
            (2024, 2, 29, true, true, 29),   // valid leap day
            (2024, 3, 1, true, true, 31),   // valid leap year
            (1969, 12, 31, false, false, 31), // invalid before epoch
            (2023, 2, 29, false, false, 28),  // invalid date
            (2023, 0, 10, false, false, 0),  // invalid date, month out of range
            (2023, 13, 10, false, false, 0), // invalid date, month out of range
            (2023, 9, 31, false, false, 30),  // invalid date, day out of range
            (2023, 9, 0, false, false, 30),   // invalid date, day out of range
            (UTCDate::MAX_YEAR, 11, 09, true, false, 30), // valid max date
            (UTCDate::MAX_YEAR, u8::MAX, u8::MAX, false, false, 0), // invalid max date
        ];

        for (year, month, day, case_is_valid, is_leap_year, days_in_month) in test_cases {
            match UTCDate::try_from_components(year, month, day) {
                Ok(date) => {
                    if !case_is_valid {
                        bail!(
                            "Case passed unexpectedly. (date: {:04}-{:02}-{:02})",
                            year,
                            month,
                            day
                        );
                    }
                    assert_eq!(is_leap_year, date.is_leap_year());
                    assert_eq!(days_in_month, date.days_in_month());
                }
                Err(e) => {
                    if case_is_valid {
                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_date_from_day() -> Result<()> {
        use crate::time::UTCDay;
        let test_cases = [
            (UTCDay::from(0), 1970, 1, 1),
            (UTCDay::from(30), 1970, 1, 31),
            (UTCDay::from(19522), 2023, 6, 14),
            (UTCDay::from(381112), 3013, 6, 14),
            (UTCDay::from(213503982334601), UTCDate::MAX_YEAR, 11, 09),
        ];

        for (utc_day, year, month, day) in test_cases {
            let date_from_day = UTCDate::from_day(utc_day);
            let date_from_comp = UTCDate::try_from_components(year, month, day)?;
            let day_from_date = date_from_comp.as_day();
            assert_eq!(date_from_day, date_from_comp);
            assert_eq!(utc_day, day_from_date);
            assert_eq!((year, month, day), date_from_comp.as_components());
            assert_eq!((year, month, day), date_from_comp.to_components());
        }

        Ok(())
    }

    #[test]
    fn test_date_iso_conversions() -> Result<()> {
        let test_cases = [
            (2023, 6, 14, true, "2023-06-14"),   // valid recent date
            (1970, 1, 1, true, "1970-01-01"),    // valid epoch date
            (2024, 2, 29, true, "2024-02-29"),   // valid leap day
            (2000, 2, 29, true, "2000-02-29"),   // valid leap day
            (1969, 12, 31, false, "1969-12-31"), // invalid before epoch
            (2023, 2, 29, false, "2023-02-29"),  // invalid date
            (2023, 0, 10, false, "2023-00-10"),  // invalid date, month out of range
            (2023, 13, 10, false, "2023-13-10"), // invalid date, month out of range
            (2023, 9, 31, false, "2023-09-31"),  // invalid date, day out of range
            (2023, 9, 0, false, "2023-09-00"),   // invalid date, day out of range
        ];

        for (year, month, day, case_is_valid, iso_date) in test_cases {
            match UTCDate::try_from_iso_date(iso_date) {
                Ok(_) => {
                    if !case_is_valid {
                        bail!(
                            "Case passed unexpectedly. (date: {:04}-{:02}-{:02})",
                            year,
                            month,
                            day
                        );
                    }

                    let date_from_comp = UTCDate::try_from_components(year, month, day)?;
                    assert_eq!(iso_date, date_from_comp.as_iso_date());
                }
                Err(e) => {
                    if case_is_valid {
                        return Err(e);
                    }
                }
            }

        }

        Ok(())
    }

    #[test]
    fn test_date_transformations() -> Result<()> {
        use crate::time::UTCTransformations;
        use crate::time::UTCTimestamp;

        let test_cases = [
            (UTCTimestamp::from_secs(0), 1970, 1, 1),
            (UTCTimestamp::from_secs(2592000), 1970, 1, 31),
            (UTCTimestamp::from_secs(1686700800), 2023, 6, 14),
            (UTCTimestamp::from_secs(32928076800), 3013, 6, 14),
            (UTCTimestamp::from_secs(371085174288000), 11761191, 1, 20),
            (UTCTimestamp::from_secs(u64::MAX - u64::MAX % SECONDS_PER_DAY), 584554051223, 11, 9),
        ];

        for (timestamp, year, month, day) in test_cases {
            let date_from_components = UTCDate::try_from_components(year, month, day)?;
            let timestamp_from_date = date_from_components.as_timestamp();
            let date_from_timestamp = UTCDate::from_timestamp(timestamp);
            assert_eq!(date_from_timestamp, date_from_components);
            assert_eq!(timestamp, timestamp_from_date);
        }

        Ok(())
    }
}
