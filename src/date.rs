use anyhow::{
    Result,
    anyhow
};

use crate::time::UTCTimestamp;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        if date.year < 1970 {
            return Err(anyhow!("Year out of range! (year: {:04})", year))
        }
        if date.month == 0 || date.month > 12 {
            return Err(anyhow!("Month out of range! (month: {:02})", month));
        }
        if date.day == 0 || date.day > date.days_in_month() {
            return Err(anyhow!("Day out of range! (day: {:02}) (yyyy-mm: {:04}-{:02})", day, month, year));
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
            _ => panic!("Month out of range! {:2}", self.month)
        }
    }
}

impl From<UTCTimestamp> for UTCDate {
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_utc_days(timestamp.to_utc_days())
    }
}

#[test]
fn test_utc_date_from_components() -> Result<()> {
    let test_cases = [
        (2023, 6, 14, true),  // valid recent date
        (1970, 1, 1, true), // valid epoch date
        (2024, 2, 29, true), // valid leap day
        (1969, 12, 31, false), // invalid before epoch
        (2023, 2, 29, false), // invalid date
        (2023, 0, 10, false), // invalid date, month out of range
        (2023, 13, 10, false), // invalid date, month out of range
        (2023, 9, 31, false), // invalid date, day out of range
        (2023, 9, 0, false), // invalid date, day out of range
    ];

    for (year, month, day, case_is_valid) in test_cases {
        match UTCDate::try_from_components(year, month, day) {
            Ok(_) => {
                if !case_is_valid {
                    return Err(anyhow!("Case passed unexpectedly. (date: {:04}-{:02}-{:02})", year, month, day));
                }
                continue;
            },
            Err(e) => {
                if case_is_valid {
                    return Err(e);
                }
                continue;
            }
        }   
    }

    Ok(())
}

#[test]
fn test_from_utc_days() -> Result<()> {
    let test_cases = [
        (0, 1970, 1, 1),
        (30, 1970, 1, 31),
        (19522, 2023, 6, 14),
        (381112, 3013, 6, 14),
    ];

    for (day_utc, year, month, day) in test_cases {
        let date = UTCDate::from_utc_days(day_utc);
        let expected = UTCDate {
            year,
            month,
            day
        };
        assert_eq!(date, expected);
    }

    Ok(())
}