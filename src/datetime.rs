use crate::{
    date::UTCDate,
    time::UTCTimestamp
};

pub struct UTCDatetime {
    date: UTCDate,
    time_of_day_nanos: u64,
}

impl UTCDatetime {
    pub fn from_components(date: UTCDate, time_of_day_nanos: u64) -> Self {
        Self {
            date,
            time_of_day_nanos,
        }
    }

    pub fn to_date(&self) -> UTCDate {
        self.date.clone()
    }

    pub fn to_time_of_day_nanos(&self) -> u64 {
        self.time_of_day_nanos
    }
}

impl From<UTCTimestamp> for UTCDatetime {
    fn from(timestamp: UTCTimestamp) -> Self {
        let tod_ns = timestamp.to_time_of_day_nanos();
        let date = UTCDate::from_utc_days(timestamp.to_utc_days());
        Self::from_components(date, tod_ns)
    }
}