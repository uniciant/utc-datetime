/// Date module.
///
/// Implements transformations required for the
/// proplectic Gregorian Calendar (the *civil* calendar).
pub mod date;
/// Time module.
///
/// Implements the UTC Timestmap as a Duration
pub mod time;

use anyhow::{
    Result,
    anyhow
};

use date::UTCDate;
use time::{
    UTCTimestamp,
    UTCTransformations,
    NANOS_PER_DAY,
    NANOS_PER_HOUR,
    NANOS_PER_MINUTE,
    NANOS_PER_SECOND,
};

pub struct UTCDatetime {
    date: UTCDate,
    time_of_day_nanos: u64,
}

impl UTCTransformations for UTCDatetime {
    fn from_utc_timestamp(timestamp: UTCTimestamp) -> Self {
        let tod_ns = timestamp.to_time_of_day_nanos();
        let date = UTCDate::from_utc_timestamp(timestamp);
        Self::from_components(date, tod_ns)
    }
}

impl UTCDatetime {
    fn from_components(date: UTCDate, time_of_day_nanos: u64) -> Self {
        Self {
            date,
            time_of_day_nanos,
        }
    }

    pub fn try_from_components(date: UTCDate, time_of_day_nanos: u64) -> Result<Self> {
        if time_of_day_nanos >= NANOS_PER_DAY {
            return Err(anyhow!("Nanoseconds not within a day! (time_of_day_ns: {})", time_of_day_nanos));
        }
        Ok(Self::from_components(date, time_of_day_nanos))
    }

    pub fn try_from_raw_components(year: u32, month: u8, day: u8, time_of_day_nanos: u64) -> Result<Self> {
        let date = UTCDate::try_from_components(year, month, day)?;
        Ok(Self::try_from_components(date, time_of_day_nanos)?)
    }

    pub fn to_components(&self) -> (UTCDate, u64) {
        (self.date, self.time_of_day_nanos)
    }

    pub fn as_components(self) -> (UTCDate, u64) {
        (self.date, self.time_of_day_nanos)
    }

    pub fn to_date(&self) -> UTCDate {
        self.date
    }

    pub fn as_date(self) -> UTCDate {
        self.date
    }

    pub fn to_time_of_day_nanos(&self) -> u64 {
        self.time_of_day_nanos
    }

    pub fn as_time_of_day_nanos(self) -> u64 {
        self.time_of_day_nanos
    }

    pub fn to_years_days_months(&self) -> (u32, u8, u8) {
        (self.date.year(), self.date.month(), self.date.day())
    }

    pub fn to_subseconds_nanos(&self) -> u32 {
        (self.time_of_day_nanos % NANOS_PER_SECOND) as u32
    }

    pub fn to_hours_minutes_seconds(&self) -> (u8, u8, u8) {
        let hours = (self.time_of_day_nanos / NANOS_PER_HOUR) as u8;
        let minutes = ((self.time_of_day_nanos % NANOS_PER_HOUR) / NANOS_PER_MINUTE) as u8;
        let seconds = ((self.time_of_day_nanos % NANOS_PER_MINUTE) / NANOS_PER_SECOND) as u8;
        (hours, minutes, seconds)
    }

    /// Return datetime as a string in the format:
    /// `YYYY-MM-DDThh:mm:ssZ`
    ///
    /// Conforms to ISO 8601:
    /// https://www.w3.org/TR/NOTE-datetime
    pub fn to_iso_datetime(&self) -> String {
        let date = self.date.to_iso_date();
        let (hours, minutes, seconds) = self.to_hours_minutes_seconds();
        format!("{date}T{:02}:{:02}:{:02}Z", hours, minutes, seconds)
    }
}

impl From<UTCTimestamp> for UTCDatetime {
    fn from(timestamp: UTCTimestamp) -> Self {
        Self::from_utc_timestamp(timestamp)
    }
}