//! Date module.
//!
//! Implements transformations required for the
//! proleptic Gregorian Calendar (the *civil* calendar),
//! to create UTC dates.

use core::time::Duration;

use anyhow::{anyhow, Result};

use crate::time::{UTCDay, UTCTimestamp, UTCTransformations};

/// UTC Date.
/// A UTC Date is any calendar date since the Unix epoch date (inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UTCDate {
    year: u32,
    month: u8,
    day: u8,
}

impl UTCDate {
    const fn _from_components_unchecked(year: u32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Unchecked method to create a UTC Date from provided year, month and day.
    ///
    /// # Safety
    /// Unsafe if the user passes an invalid calendar year, month and day combination.
    /// Invalid inputs are not checked and may cause a panic in other methods.
    #[inline]
    pub const unsafe fn from_components(year: u32, month: u8, day: u8) -> Self {
        Self::_from_components_unchecked(year, month, day)
    }

    /// Try to create a UTC Date from provided year, month and day.
    pub fn try_from_components(year: u32, month: u8, day: u8) -> Result<Self> {
        // force create
        let date = Self::_from_components_unchecked(year, month, day);
        // then check
        if date.year < 1970 {
            return Err(anyhow!("Year out of range! (year: {:04})", year));
        }
        if date.month == 0 || date.month > 12 {
            return Err(anyhow!("Month out of range! (month: {:02})", month));
        }
        if date.day == 0 || date.day > date.days_in_month() {
            return Err(anyhow!(
                "Day out of range! (day: {:02}) (yyyy-mm: {:04}-{:02})",
                day,
                month,
                year
            ));
        }
        Ok(date)
    }

    /// Create a UTC Date from the number of days since the epoch.
    ///
    /// Reference:
    /// <http://howardhinnant.github.io/date_algorithms.html#civil_from_days>
    ///
    /// Simplified for unsigned days/years
    pub const fn from_utc_day(utc_day: UTCDay) -> Self {
        let z = utc_day.as_u32() + 719468;
        let era = z / 146097;
        let doe = z - (era * 146097);
        let yoe = (doe - (doe / 1460) + (doe / 36524) - (doe / 146096)) / 365;
        let doy = doe - (365 * yoe) - (yoe / 4) + (yoe / 100);
        let mp = ((5 * doy) + 2) / 153;
        let day = (doy - (((153 * mp) + 2) / 5) + 1) as u8;
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
        let year = yoe + era * 400 + (month <= 2) as u32;
        Self { day, month, year }
    }

    /// Get the days since the epoch from the UTC Date
    ///
    /// Reference:
    /// <http://howardhinnant.github.io/date_algorithms.html#days_from_civil>
    ///
    /// Simplified for unsigned days/years
    pub const fn as_utc_day(&self) -> UTCDay {
        let m = self.month as u32;
        let d = self.day as u32;
        let y = self.year - ((m <= 2) as u32);
        let era = y / 400;
        let yoe = y - era * 400;
        let doy = ((153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5) + d - 1;
        let doe = (yoe * 365) + (yoe / 4) - (yoe / 100) + doy;
        let days = (era * 146097) + doe - 719468;
        UTCDay::from_u32(days)
    }

    /// Get copy of the date components as integers
    ///
    /// Returns tuple: `(year: u32, month: u8, day: u8)`
    #[inline]
    pub const fn as_components(&self) -> (u32, u8, u8) {
        (self.year, self.month, self.day)
    }

    /// Consume self into date components as integers
    ///
    /// Returns tuple: `(year: u32, month: u8, day: u8)`
    #[inline]
    pub const fn to_components(self) -> (u32, u8, u8) {
        (self.year, self.month, self.day)
    }

    /// Return day component of date
    #[inline]
    pub const fn as_day(&self) -> u8 {
        self.day
    }

    /// Return month component of date
    #[inline]
    pub const fn as_month(&self) -> u8 {
        self.month
    }

    /// Return year component of date
    #[inline]
    pub const fn as_year(&self) -> u32 {
        self.year
    }

    /// Returns whether date is within a leap year.
    ///
    /// Reference:
    /// <http://howardhinnant.github.io/date_algorithms.html#is_leap>
    #[inline]
    pub const fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0) && ((self.year % 100 != 0) || (self.year % 400 == 0))
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

    /// Return date as a string in the format:
    /// `YYYY-MM-DD`
    ///
    /// Conforms to ISO 8601:
    /// <https://www.w3.org/TR/NOTE-datetime>
    #[cfg(feature = "std")]
    #[inline]
    pub fn as_iso_date(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl UTCTransformations for UTCDate {
    fn from_utc_secs(s: u64) -> Self {
        let utc_day = UTCDay::from_utc_secs(s);
        Self::from_utc_day(utc_day)
    }

    fn as_utc_secs(&self) -> u64 {
        self.as_utc_day().as_utc_secs()
    }

    fn from_utc_millis(s: u64) -> Self {
        let utc_day = UTCDay::from_utc_millis(s);
        Self::from_utc_day(utc_day)
    }

    fn as_utc_millis(&self) -> u64 {
        self.as_utc_day().as_utc_millis()
    }

    fn from_utc_micros(s: u64) -> Self {
        let utc_day = UTCDay::from_utc_micros(s);
        Self::from_utc_day(utc_day)
    }

    fn as_utc_micros(&self) -> u64 {
        self.as_utc_day().as_utc_micros()
    }

    fn from_utc_nanos(s: u64) -> Self {
        let utc_day = UTCDay::from_utc_nanos(s);
        Self::from_utc_day(utc_day)
    }

    fn as_utc_nanos(&self) -> u64 {
        self.as_utc_day().as_utc_nanos()
    }

    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self {
        let utc_day = UTCDay::from_utc_timestamp(timestamp);
        Self::from_utc_day(utc_day)
    }

    fn as_utc_timestamp(&self) -> UTCTimestamp {
        self.as_utc_day().as_utc_timestamp()
    }
}

impl From<Duration> for UTCDate {
    fn from(duration: Duration) -> Self {
        Self::from_utc_duration(duration)
    }
}

impl From<UTCTimestamp> for UTCDate {
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_utc_timestamp(timestamp)
    }
}

impl From<UTCDay> for UTCDate {
    fn from(utc_day: UTCDay) -> Self {
        Self::from_utc_day(utc_day)
    }
}

#[cfg(test)]
mod test {
    use anyhow::{anyhow, Result};

    use crate::date::UTCDate;
    use crate::time::UTCDay;

    #[test]
    fn test_utc_date_from_components() -> Result<()> {
        let test_cases = [
            (2023, 6, 14, true),   // valid recent date
            (1970, 1, 1, true),    // valid epoch date
            (2024, 2, 29, true),   // valid leap day
            (1969, 12, 31, false), // invalid before epoch
            (2023, 2, 29, false),  // invalid date
            (2023, 0, 10, false),  // invalid date, month out of range
            (2023, 13, 10, false), // invalid date, month out of range
            (2023, 9, 31, false),  // invalid date, day out of range
            (2023, 9, 0, false),   // invalid date, day out of range
        ];

        for (year, month, day, case_is_valid) in test_cases {
            match UTCDate::try_from_components(year, month, day) {
                Ok(_) => {
                    if !case_is_valid {
                        return Err(anyhow!(
                            "Case passed unexpectedly. (date: {:04}-{:02}-{:02})",
                            year,
                            month,
                            day
                        ));
                    }
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
    fn test_from_utc_day() -> Result<()> {
        let test_cases = [
            (UTCDay::from(0), 1970, 1, 1),
            (UTCDay::from(30), 1970, 1, 31),
            (UTCDay::from(19522), 2023, 6, 14),
            (UTCDay::from(381112), 3013, 6, 14),
        ];

        for (utc_day, year, month, day) in test_cases {
            let date = UTCDate::from_utc_day(utc_day);
            let expected = UTCDate { year, month, day };
            assert_eq!(date, expected);
        }

        Ok(())
    }

    #[test]
    fn test_to_utc_day() -> Result<()> {
        let test_cases = [
            (UTCDay::from(0), 1970, 1, 1),
            (UTCDay::from(30), 1970, 1, 31),
            (UTCDay::from(19522), 2023, 6, 14),
            (UTCDay::from(381112), 3013, 6, 14),
        ];

        for (expected, year, month, day) in test_cases {
            let date = UTCDate::try_from_components(year, month, day)?;
            let utc_day = date.as_utc_day();
            assert_eq!(utc_day, expected);
        }

        Ok(())
    }
}
