use anyhow::{
    Result,
    anyhow
};

use crate::time::UTCTimestamp;

#[derive(Clone)]
pub struct UTCDate {
    year: u32,
    month: u8,
    day: u8
}

impl UTCDate {
    pub fn try_from_components(year: u32, month: u8, day: u8) -> Result<Self> {
        // force create
        let date = Self {
            year,
            month,
            day,
        };
        if date.month == 0 || date.month > 12 {
            return Err(anyhow!("Month out of range! (month: {})", month));
        }
        if date.day > date.days_in_month() {
            return Err(anyhow!("Day out of range! (day: {}) (yyyy-mm: {}-{:02})", day, month, year));
        }
        Ok(date)
    }

    /// Reference:
    /// http://howardhinnant.github.io/date_algorithms.html#civil_from_days
    ///
    /// Simplified for unsigned days/years
    pub fn from_utc_days(days: u32) -> Self {
        let z = days + 719468;
        let era = z / 146097;
        let doe = z - (era * 146097);
        let yoe = (doe - (doe / 1460) + (doe / 36524) - (doe / 146096)) / 365;
        let doy = doe - (365 * yoe) - (yoe / 4) + (yoe / 100);
        let mp = ((5 * doy) + 2) / 153;
        let day = (doy - (((153 * mp) + 2) / 5) + 1) as u8;
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
        let year = yoe + era * 400 + (month <= 2) as u32;
        Self {
            day,
            month,
            year,
        }
    }

    /// Return day component of date
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Return month component of date
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Return year component of date
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Returns whether date is within a leap year.
    /// Reference:
    /// http://howardhinnant.github.io/date_algorithms.html#is_leap
    pub fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0)
        && ((self.year % 100 != 0)
            || (self.year % 400 == 0))
    }

    /// Returns the number of days within the month of the date.
    /// Leap years are acconted for.
    pub fn days_in_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() { 29 } else { 28 },
            _ => panic!("Month out of range! {}", self.month)
        }
    }
}

impl From<UTCTimestamp> for UTCDate {
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_utc_days(timestamp.to_utc_days())
    }
}
