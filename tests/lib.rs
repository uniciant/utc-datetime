use anyhow::Result;

use utc_dt::{UTCDatetime, date::UTCDate, time::{UTCTimeOfDay, UTCDay}};

#[test]
fn test_datetime_from_raw_components() -> Result<()> {
    let test_cases = [
        (1970, 1, 1, 0, 0, 0, 0, 0, UTCDay::ZERO), // thu, 00:00:00.000
        (2023, 6, 14, 09, 20, 09, 648_000_000, 33_609_648_000_000, UTCDay::try_from_u64(19522)?), // wed, 09:20:09.648
    ];

    for (year, month, day, hrs, mins, secs, subsec_ns, expected_tod_ns, expected_day) in test_cases {
        let date = UTCDate::try_from_components(year, month, day)?;
        let tod = UTCTimeOfDay::try_from_hhmmss(hrs, mins, secs, subsec_ns)?;
        let datetime = UTCDatetime::from_components(date, tod);
        assert_eq!(datetime.as_date().as_day(), expected_day);
        assert_eq!(datetime.as_tod().as_nanos(), expected_tod_ns);
    }

    Ok(())
}

#[cfg(feature = "std")]
#[test]
fn test_datetime_iso_conversions() -> Result<()> {
    use utc_dt::{date::UTCDate, time::UTCTimeOfDay};

    let test_cases = [
        (1970, 1, 1, 0, None, "1970-01-01T00:00:00Z"), // thu, 00:00:00
        (1970, 1, 1, 0, Some(0), "1970-01-01T00:00:00.Z"), // thu, 00:00:00.
        (1970, 1, 1, 0, Some(3), "1970-01-01T00:00:00.000Z"), // thu, 00:00:00.000
        (1970, 1, 1, 0, Some(9), "1970-01-01T00:00:00.000000000Z"), // thu, 00:00:00.000000000
        (1970, 1, 1, 0, Some(11), "1970-01-01T00:00:00.000000000Z"), // thu, 00:00:00.000000000
        (2023, 6, 14, 33_609_648_000_000, Some(3), "2023-06-14T09:20:09.648Z"), // wed, 09:20:09.648
    ];

    for (year, month, day, tod_ns, precision, iso_datetime) in test_cases {
        let date = UTCDate::try_from_components(year, month, day)?;
        let tod = UTCTimeOfDay::try_from_nanos(tod_ns)?;
        let datetime_from_components = UTCDatetime::from_components(date, tod);
        let datetime_from_iso = UTCDatetime::try_from_iso_datetime(iso_datetime)?;
        assert_eq!(datetime_from_components.as_iso_datetime(precision), iso_datetime);
        assert_eq!(datetime_from_iso, datetime_from_components)
    }

    Ok(())
}
