use utc_dt::{
    date::UTCDate,
    time::{UTCDay, UTCTimeOfDay, UTCTimestamp, UTCTransformations},
    UTCDatetime, UTCError,
};

#[test]
fn test_datetime_from_raw_components() -> Result<(), UTCError> {
    let test_cases = [
        (1970, 1, 1, 0, 0, 0, 0, 0, UTCDay::ZERO), // thu, 00:00:00.000
        (
            2023,
            6,
            14,
            09,
            20,
            09,
            648_000_000,
            33_609_648_000_000,
            UTCDay::try_from_u64(19522)?,
        ), // wed, 09:20:09.648
    ];

    // run raw components test cases
    for (year, month, day, hrs, mins, secs, subsec_ns, expected_tod_ns, expected_day) in test_cases
    {
        let date = UTCDate::try_from_components(year, month, day)?;
        let tod = UTCTimeOfDay::try_from_hhmmss(hrs, mins, secs, subsec_ns)?;
        let datetime = UTCDatetime::from_components(date, tod);
        assert_eq!(datetime.as_date().as_day(), expected_day);
        assert_eq!(datetime.as_tod().as_nanos(), expected_tod_ns);
    }

    // test display & debug
    #[cfg(feature = "std")]
    let datetime = UTCDatetime::try_from_system_time().unwrap();
    #[cfg(not(feature = "std"))]
    let datetime = UTCDatetime::from_millis(1686824288903);
    // test to/as components
    let (date, tod) = datetime.as_components();
    assert_eq!((date, tod), datetime.to_components());
    // test from timestamp
    #[cfg(feature = "std")]
    let timestamp = UTCTimestamp::try_from_system_time().unwrap();
    #[cfg(not(feature = "std"))]
    let timestamp = UTCTimestamp::from_millis(1686824288903);
    let datetime_from_timestamp = UTCDatetime::from_timestamp(timestamp);
    assert_eq!(UTCDatetime::from(timestamp), datetime_from_timestamp);
    assert_eq!(datetime_from_timestamp.as_timestamp(), timestamp);
    // test from duration
    let duration = timestamp.as_duration();
    let datetime_from_duration = UTCDatetime::from_duration(duration);
    assert_eq!(UTCDatetime::from(duration), datetime_from_duration);
    assert_eq!(datetime_from_duration.as_duration(), duration);
    // test unit conversions
    let secs_from_datetime = datetime.as_secs();
    let millis_from_datetime = datetime.as_millis() as u64;
    let micros_from_datetime = datetime.as_micros() as u64;
    let nanos_from_datetime = datetime.as_nanos() as u64;
    let datetime_from_secs = UTCDatetime::from_secs(secs_from_datetime);
    let datetime_from_millis = UTCDatetime::from_millis(millis_from_datetime);
    let datetime_from_micros = UTCDatetime::from_micros(micros_from_datetime);
    let datetime_from_nanos = UTCDatetime::from_nanos(nanos_from_datetime);
    assert!(datetime_from_secs <= datetime);
    assert!(datetime_from_millis <= datetime);
    assert!(datetime_from_micros <= datetime);
    assert!(datetime_from_nanos <= datetime);
    // test hashing
    #[cfg(feature = "std")]
    {
        use std::collections::HashSet;
        let mut hash_set: HashSet<UTCDatetime> = HashSet::new();
        hash_set.insert(datetime);
        assert!(hash_set.contains(&datetime));
        assert_eq!(&datetime, hash_set.get(&datetime).unwrap());
    }
    // test default, clone & copy, ord
    assert_eq!(UTCDatetime::default().clone(), UTCDatetime::MIN);
    let datetime_copy = datetime;
    assert_eq!(datetime_copy, datetime);
    assert_eq!(UTCDatetime::MIN, datetime_copy.min(UTCDatetime::MIN));
    assert_eq!(UTCDatetime::MAX, datetime_copy.max(UTCDatetime::MAX));
    // test limits
    assert_eq!(
        UTCDatetime::from_timestamp(UTCTimestamp::MAX),
        UTCDatetime::MAX
    );
    assert_eq!(
        UTCDatetime::from_timestamp(UTCTimestamp::ZERO),
        UTCDatetime::MIN
    );

    Ok(())
}

#[test]
fn test_datetime_iso_conversions() -> Result<(), UTCError> {
    let test_cases = [
        (1970, 1, 1, 0, 0, "1970-01-01T00:00:00Z"), // thu, 00:00:00
        (1970, 1, 1, 0, 3, "1970-01-01T00:00:00.000Z"), // thu, 00:00:00.000
        (1970, 1, 1, 0, 9, "1970-01-01T00:00:00.000000000Z"), // thu, 00:00:00.000000000
        (1970, 1, 1, 0, 11, "1970-01-01T00:00:00.000000000Z"), // thu, 00:00:00.000000000
        (
            2023,
            6,
            14,
            33_609_648_000_000,
            3,
            "2023-06-14T09:20:09.648Z",
        ), // wed, 09:20:09.648
    ];
    let mut buf = [0; UTCDatetime::iso_datetime_len(9)];

    // run iso conversion test cases
    for (year, month, day, tod_ns, precision, iso_datetime) in test_cases {
        let date = UTCDate::try_from_components(year, month, day)?;
        let tod = UTCTimeOfDay::try_from_nanos(tod_ns)?;
        let datetime_from_components = UTCDatetime::from_components(date, tod);
        let datetime_from_iso = UTCDatetime::try_from_iso_datetime(iso_datetime)?;
        #[cfg(feature = "alloc")]
        assert_eq!(
            datetime_from_components.as_iso_datetime(precision),
            iso_datetime
        );
        let written = datetime_from_components.write_iso_datetime(&mut buf, precision)?;
        let iso_raw_str = core::str::from_utf8(&buf[..written]).unwrap();
        assert_eq!(iso_raw_str.len(), UTCDatetime::iso_datetime_len(precision));
        assert_eq!(iso_datetime.as_bytes(), &buf[..written]);
        assert_eq!(iso_datetime, iso_raw_str);
        assert_eq!(datetime_from_iso, datetime_from_components);
        // test maybe-invalid buf len
        let mut buf = [0; 3];
        let result = datetime_from_components.write_iso_datetime(&mut buf, precision);
        if buf.len() < UTCDatetime::iso_datetime_len(precision) {
            assert!(result.is_err())
        } else {
            assert!(result.is_ok())
        }
    }

    // test invalid iso dates
    assert!(UTCDatetime::try_from_iso_datetime("197a-01-01T00:00:00Z").is_err());
    assert!(UTCDatetime::try_from_iso_datetime("1970-01-01T00:a0:00Z").is_err());
    assert!(UTCDatetime::try_from_iso_datetime("1970-01-01T00:a0").is_err());

    // test display & debug
    #[cfg(feature = "std")]
    {
        let datetime = UTCDatetime::try_from_system_time().unwrap();
        println!("{:?}:{datetime}", datetime);
    }
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_datetime_serde() {
    let datetime = UTCDatetime::from_secs(1724493234);
    let v = serde_json::to_value(&datetime).unwrap();
    assert_eq!(datetime, serde_json::from_value(v).unwrap());
}
