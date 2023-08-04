use anyhow::Result;

use utc_dt::{UTCDatetime, date::UTCDate, time::{UTCTimeOfDay, UTCDay}};

#[test]
fn test_datetime_from_raw_components() -> Result<()> {
    let test_cases = [
        (1970, 1, 1, 0, 0, 0, 0, 0, UTCDay::ZERO), // thu, 00:00:00.000
        (2023, 6, 14, 09, 20, 09, 648_000_000, 33_609_648_000_000, UTCDay::try_from_u64(19522)?), // wed, 09:20:09.648
    ];

    // run raw components test cases
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
    use std::collections::HashSet;

    use utc_dt::{date::UTCDate, time::{UTCTimeOfDay, UTCTransformations, UTCTimestamp}};

    let test_cases = [
        (1970, 1, 1, 0, None, "1970-01-01T00:00:00Z"), // thu, 00:00:00
        (1970, 1, 1, 0, Some(0), "1970-01-01T00:00:00.Z"), // thu, 00:00:00.
        (1970, 1, 1, 0, Some(3), "1970-01-01T00:00:00.000Z"), // thu, 00:00:00.000
        (1970, 1, 1, 0, Some(9), "1970-01-01T00:00:00.000000000Z"), // thu, 00:00:00.000000000
        (1970, 1, 1, 0, Some(11), "1970-01-01T00:00:00.000000000Z"), // thu, 00:00:00.000000000
        (2023, 6, 14, 33_609_648_000_000, Some(3), "2023-06-14T09:20:09.648Z"), // wed, 09:20:09.648
    ];

    // run iso conversion test cases
    for (year, month, day, tod_ns, precision, iso_datetime) in test_cases {
        let date = UTCDate::try_from_components(year, month, day)?;
        let tod = UTCTimeOfDay::try_from_nanos(tod_ns)?;
        let datetime_from_components = UTCDatetime::from_components(date, tod);
        let datetime_from_iso = UTCDatetime::try_from_iso_datetime(iso_datetime)?;
        assert_eq!(datetime_from_components.as_iso_datetime(precision), iso_datetime);
        assert_eq!(datetime_from_iso, datetime_from_components);
    }

    // test invalid iso dates
    assert!(UTCDatetime::try_from_iso_datetime("197a-01-01T00:00:00Z").is_err());
    assert!(UTCDatetime::try_from_iso_datetime("1970-01-01T00:a0:00Z").is_err());
    // test display & debug
    let datetime_from_system_time = UTCDatetime::try_from_system_time()?;
    println!("{:?}:{datetime_from_system_time}", datetime_from_system_time);
    // test to/as components
    let (date, tod) = datetime_from_system_time.as_components();
    assert_eq!((date, tod), datetime_from_system_time.to_components());
    // test from timestamp
    let timestamp = UTCTimestamp::try_from_system_time()?;
    let datetime_from_timestamp = UTCDatetime::from_timestamp(timestamp);
    assert_eq!(UTCDatetime::from(timestamp), datetime_from_timestamp);
    assert_eq!(datetime_from_timestamp.as_timestamp(), timestamp);
    // test from duration
    let duration = timestamp.as_duration();
    let datetime_from_duration = UTCDatetime::from_duration(duration);
    assert_eq!(UTCDatetime::from(duration), datetime_from_duration);
    assert_eq!(datetime_from_duration.as_duration(), duration);
    // test unit conversions
    let secs_from_datetime = datetime_from_system_time.as_secs();
    let millis_from_datetime = datetime_from_system_time.as_millis() as u64;
    let micros_from_datetime = datetime_from_system_time.as_micros() as u64;
    let nanos_from_datetime = datetime_from_system_time.as_nanos() as u64;
    let datetime_from_secs = UTCDatetime::from_secs(secs_from_datetime);
    let datetime_from_millis = UTCDatetime::from_millis(millis_from_datetime);
    let datetime_from_micros = UTCDatetime::from_micros(micros_from_datetime);
    let datetime_from_nanos = UTCDatetime::from_nanos(nanos_from_datetime);
    assert!(datetime_from_secs <= datetime_from_system_time);
    assert!(datetime_from_millis <= datetime_from_system_time);
    assert!(datetime_from_micros <= datetime_from_system_time);
    assert!(datetime_from_nanos <= datetime_from_system_time);
    // test hashing
    let mut hash_set: HashSet<UTCDatetime> = HashSet::new();
    hash_set.insert(datetime_from_system_time);
    assert!(hash_set.contains(&datetime_from_system_time));
    assert_eq!(&datetime_from_system_time, hash_set.get(&datetime_from_system_time).unwrap());
    // test default, clone & copy, ord
    assert_eq!(UTCDatetime::default().clone(), UTCDatetime::MIN);
    let datetime_copy = datetime_from_system_time;
    assert_eq!(datetime_copy, datetime_from_system_time);
    assert_eq!(UTCDatetime::MIN, datetime_copy.min(UTCDatetime::MIN));
    assert_eq!(UTCDatetime::MAX, datetime_copy.max(UTCDatetime::MAX));
    // test limits
    assert_eq!(UTCDatetime::from_timestamp(UTCTimestamp::MAX), UTCDatetime::MAX);
    assert_eq!(UTCDatetime::from_timestamp(UTCTimestamp::ZERO), UTCDatetime::MIN);

    Ok(())
}
