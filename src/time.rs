use std::time::Duration;

use anyhow::{
    Result,
    anyhow
};

use crate::date::UTCDate;

const SECONDS_PER_DAY: u64 = 86400;
const MILLIS_PER_SECOND: u64 = 1000;
const MICROS_PER_SECOND: u64 = MILLIS_PER_SECOND * 1000;
const NANOS_PER_SECOND: u64 = MICROS_PER_SECOND * 1000;

/// UTC Timestamp.
/// A UTC Timestamp the duration since the Unix Epoch (1970).
pub struct UTCTimestamp(Duration);

impl UTCTimestamp {
    pub fn to_time_of_day_nanos(&self) -> u64 {
        (self.0.as_secs() * NANOS_PER_SECOND) + (self.0.subsec_nanos() as u64)
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
        if nanos >= (NANOS_PER_SECOND as u32) {
            return Err(anyhow!("Nanoseconds should total less than a second! (nanos: {})", nanos));
        }
        Ok(Self(Duration::new(secs, nanos)))
    }

    /// Reference:
    /// http://howardhinnant.github.io/date_algorithms.html#days_from_civil
    ///
    /// Simplified for unsigned days/years
    pub fn try_from_date_and_nanos(year: u32, month: u8, day: u8) -> Result<Self> {
        let m = month as u32;
        let d = day as u32;
        let y = year - ((m <= 2) as u32);
        let era = y / 400;
        let yoe = y;
        let doy = ((153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5) + d - 1;
        let doe = (yoe * 365) + (yoe / 4) + (yoe / 100) + doy;
        let utc_day = ((era * 146097) + doe) - 719468;
        Self::try_from_days_and_nanos(utc_day, 0u32)
    }

    pub fn try_from_days_and_nanos<T: Into<u64>, U: Into<u32>>(d: T, ns: U) -> Result<Self> {
        let secs = d.into() * SECONDS_PER_DAY;
        let nanos = ns.into();
        Self::try_from_components(secs, nanos)
    }


    pub fn from_millis<T: Into<u64>>(ms: T) -> Self {
        Self(Duration::from_millis(ms.into()))
    }

    pub fn from_micros<T: Into<u64>>(us: T) -> Self {
        Self(Duration::from_micros(us.into()))
    }

    pub fn from_nanos<T: Into<u64>>(ns: T) -> Self {
        Self(Duration::from_nanos(ns.into()))
    }
}

impl From<UTCDate> for UTCTimestamp {
    fn from(date: UTCDate) -> Self {
        Self::try_from_date_and_nanos(date.year(), date.month(), date.day()).unwrap()
    }
}
