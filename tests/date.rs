use std::collections::HashSet;

use utc_dt::{
    constants::{MICROS_PER_DAY, MILLIS_PER_DAY, NANOS_PER_DAY, SECONDS_PER_DAY},
    date::UTCDate,
    time::{UTCDay, UTCTimestamp, UTCTransformations},
    UTCError,
};

#[test]
fn test_date_from_components() {
    let test_cases = [
        (2023, 6, 14, true, false, 30),               // valid recent date
        (1970, 1, 1, true, false, 31),                // valid epoch date
        (2024, 2, 29, true, true, 29),                // valid leap day
        (2024, 3, 1, true, true, 31),                 // valid leap year
        (1969, 12, 31, false, false, 31),             // invalid before epoch
        (2023, 2, 29, false, false, 28),              // invalid date
        (2023, 0, 10, false, false, 0),               // invalid date, month out of range
        (2023, 13, 10, false, false, 0),              // invalid date, month out of range
        (2023, 9, 31, false, false, 30),              // invalid date, day out of range
        (2023, 9, 0, false, false, 30),               // invalid date, day out of range
        (UTCDate::MAX_YEAR, 11, 09, true, false, 30), // valid max date
        (UTCDate::MAX_YEAR, 12, 31, false, false, 0), // invalid max date
        (UTCDate::MAX_YEAR, u8::MAX, u8::MAX, false, false, 0), // invalid max date
    ];

    for (year, month, day, case_is_valid, is_leap_year, days_in_month) in test_cases {
        match UTCDate::try_from_components(year, month, day) {
            Ok(date) => {
                assert!(case_is_valid);
                assert_eq!(is_leap_year, date.is_leap_year());
                assert_eq!(days_in_month, date.days_in_month());
            }
            Err(_) => {
                assert!(!case_is_valid);
            }
        }
    }
}

#[test]
fn test_date_from_day() -> Result<(), UTCError> {
    let test_cases = [
        (UTCDay::ZERO, 1970, 1, 1),
        (UTCDay::try_from_u64(30)?, 1970, 1, 31),
        (UTCDay::try_from_u64(19522)?, 2023, 6, 14),
        (UTCDay::try_from_u64(381112)?, 3013, 6, 14),
        (UTCDay::MAX, UTCDate::MAX_YEAR, 11, 09),
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
fn test_date_iso_conversions() -> Result<(), UTCError> {
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
        (2023, 9, 0, false, "202a-09-00"),   // invalid date, year not integer
        (2023, 9, 0, false, "2023-0a-00"),   // invalid date, month not integer
        (2023, 9, 0, false, "2023-09-0a"),   // invalid date, day not integer
    ];
    let mut buf = [0; UTCDate::ISO_DATE_LEN];

    for (year, month, day, case_is_valid, iso_date) in test_cases {
        match UTCDate::try_from_iso_date(iso_date) {
            Ok(date_from_iso) => {
                assert!(case_is_valid);
                let date_from_comp = UTCDate::try_from_components(year, month, day)?;
                assert_eq!(date_from_comp, date_from_iso);
                #[cfg(feature = "alloc")]
                assert_eq!(iso_date, date_from_comp.as_iso_date());
                let written = date_from_comp.write_iso_date(&mut buf)?;
                assert_eq!(iso_date.as_bytes(), &buf[..written]);
                assert_eq!(iso_date, core::str::from_utf8(&buf[..written]).unwrap());
                // test invalid buf len
                let mut buf = [0; 1];
                assert!(date_from_comp.write_iso_date(&mut buf).is_err());
            }
            Err(_) => {
                assert!(!case_is_valid);
            }
        }
    }

    // test transform from system time
    #[cfg(feature = "std")]
    {
        let date_from_system_time = UTCDate::try_from_system_time().unwrap();
        assert!(date_from_system_time >= UTCDate::MIN);
        assert!(date_from_system_time <= UTCDate::MAX);
        // test debug & display
        println!("{:?}:{date_from_system_time}", date_from_system_time);
        // test default, clone & copy, ord
        assert_eq!(UTCDate::default().clone(), UTCDate::MIN);
        let date_copy = date_from_system_time;
        assert_eq!(date_copy, date_from_system_time);
        assert_eq!(UTCDate::MIN, date_copy.min(UTCDate::MIN));
        assert_eq!(UTCDate::MAX, date_copy.max(UTCDate::MAX));
        // test limits
        assert_eq!(UTCDate::from_day(UTCDay::MAX), UTCDate::MAX);
        assert_eq!(UTCDate::from_day(UTCDay::ZERO), UTCDate::MIN);
    }
    Ok(())
}

#[test]
fn test_date_transformations() -> Result<(), UTCError> {
    let test_cases = [
        (UTCTimestamp::from_secs(0), 1970, 1, 1),
        (UTCTimestamp::from_secs(2592000), 1970, 1, 31),
        (UTCTimestamp::from_secs(1686700800), 2023, 6, 14),
        (UTCTimestamp::from_secs(32928076800), 3013, 6, 14),
        (UTCTimestamp::from_secs(371085174288000), 11761191, 1, 20),
        (
            UTCTimestamp::from_secs(u64::MAX - u64::MAX % SECONDS_PER_DAY),
            584554051223,
            11,
            9,
        ),
    ];

    let mut hash_set: HashSet<UTCDate> = HashSet::new();

    for (timestamp, year, month, day) in test_cases {
        let date_from_components = UTCDate::try_from_components(year, month, day)?;
        // test transformations to/from durations
        let duration_from_date = date_from_components.as_duration();
        let date_from_duration = UTCDate::from_duration(timestamp.as_duration());
        assert_eq!(date_from_duration, date_from_components);
        assert_eq!(timestamp.as_duration(), duration_from_date);
        // test transformations to/from timestamps
        let timestamp_from_date = date_from_components.as_timestamp();
        let date_from_timestamp = UTCDate::from_timestamp(timestamp);
        assert_eq!(date_from_timestamp, date_from_components);
        assert_eq!(timestamp, timestamp_from_date);
        // test From implementations
        let date_from_duration = UTCDate::from(timestamp.as_duration());
        assert_eq!(date_from_components, date_from_duration);
        let date_from_timestamp = UTCDate::from(timestamp);
        assert_eq!(date_from_components, date_from_timestamp);
        let day = timestamp.as_day();
        let date_from_day = UTCDate::from(day);
        assert_eq!(date_from_components, date_from_day);
        // test unit conversions
        let secs = timestamp.as_secs();
        let millis = timestamp.as_millis() as u64;
        let micros = timestamp.as_micros() as u64;
        let nanos = timestamp.as_nanos() as u64;
        let date_from_secs = UTCDate::from_secs(secs);
        let date_from_millis = UTCDate::from_millis(millis);
        let date_from_micros = UTCDate::from_micros(micros);
        let date_from_nanos = UTCDate::from_nanos(nanos);
        assert_eq!(date_from_components, date_from_secs);
        let secs_from_date = date_from_secs.as_secs();
        let millis_from_date = date_from_millis.as_millis() as u64;
        let micros_from_date = date_from_micros.as_micros() as u64;
        let nanos_from_date = date_from_nanos.as_nanos() as u64;
        assert!(secs - secs_from_date < SECONDS_PER_DAY);
        assert!(millis - millis_from_date < MILLIS_PER_DAY);
        assert!(micros - micros_from_date < MICROS_PER_DAY);
        assert!(nanos - nanos_from_date < NANOS_PER_DAY);
        // test hashing
        hash_set.insert(date_from_components);
        assert!(hash_set.contains(&date_from_components));
        assert_eq!(
            &date_from_components,
            hash_set.get(&date_from_components).unwrap()
        );
    }

    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_date_serde() {
    let date = UTCDate::from_day(UTCDay::try_from_u64(19959).unwrap());
    let v = serde_json::to_value(&date).unwrap();
    assert_eq!(date, serde_json::from_value(v).unwrap())
}
