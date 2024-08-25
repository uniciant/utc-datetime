use core::time::Duration;
use std::collections::HashSet;

use utc_dt::{
    constants::{MICROS_PER_DAY, MILLIS_PER_DAY, NANOS_PER_DAY, NANOS_PER_SECOND, SECONDS_PER_DAY},
    time::{UTCDay, UTCTimeOfDay, UTCTimestamp, UTCTransformations},
    UTCError,
};

#[test]
fn test_utc_timestamp() -> Result<(), UTCError> {
    let test_cases = [
        (
            UTCTimestamp::from_nanos(0),
            UTCDay::ZERO,
            UTCTimeOfDay::try_from_secs(0)?,
            4,
        ),
        (
            UTCTimestamp::from_nanos(123456789),
            UTCDay::ZERO,
            UTCTimeOfDay::try_from_nanos(123456789)?,
            4,
        ),
        (
            UTCTimestamp::from_millis(1686756677000),
            UTCDay::try_from_u64(19522)?,
            UTCTimeOfDay::try_from_nanos(55_877_000_000_000)?,
            3,
        ),
        (
            UTCTimestamp::from_millis(1709220677000),
            UTCDay::try_from_u64(19782)?,
            UTCTimeOfDay::try_from_micros(55_877_000_000)?,
            4,
        ),
        (
            UTCTimestamp::from_millis(1677684677000),
            UTCDay::try_from_u64(19417)?,
            UTCTimeOfDay::try_from_millis(55_877_000)?,
            3,
        ),
        (
            UTCTimestamp::from_duration(Duration::MAX),
            UTCDay::MAX,
            UTCTimeOfDay::try_from_nanos(25215 * NANOS_PER_SECOND + NANOS_PER_SECOND - 1)?,
            4,
        ),
    ];

    let mut hash_set: HashSet<UTCTimestamp> = HashSet::new();

    for (expected_timestamp, utc_days, tod, weekday) in test_cases {
        let timestamp = UTCTimestamp::from_day_and_tod(utc_days, tod);
        // test timestamp to/from components
        assert_eq!(timestamp, expected_timestamp);
        assert_eq!(UTCDay::from_timestamp(timestamp), utc_days);
        assert_eq!(timestamp.as_tod(), tod);
        assert_eq!(utc_days.as_weekday(), weekday);
        // test timestamp to/from days
        let timestamp_from_day = UTCTimestamp::from_day(utc_days);
        assert_eq!(timestamp_from_day.as_day(), utc_days);
        assert_eq!(timestamp_from_day.as_tod(), UTCTimeOfDay::ZERO);
        assert_eq!(timestamp_from_day, UTCTimestamp::from(utc_days));
        // test timestamp to/from durations
        let duration_from_timestamp = timestamp.to_duration();
        let timestamp_from_duration = UTCTimestamp::from_duration(duration_from_timestamp);
        assert_eq!(
            timestamp_from_duration,
            UTCTimestamp::from(duration_from_timestamp)
        );
        assert_eq!(timestamp_from_duration, expected_timestamp);
        assert_eq!(duration_from_timestamp, expected_timestamp.as_duration());
        // test unit conversions
        let secs_from_timestamp = expected_timestamp.as_secs();
        let millis_from_timestamp = expected_timestamp.as_millis() as u64;
        let micros_from_timestamp = expected_timestamp.as_micros() as u64;
        let nanos_from_timestamp = expected_timestamp.as_nanos() as u64;
        let timestamp_from_secs = UTCTimestamp::from_secs(secs_from_timestamp);
        let timestamp_from_millis = UTCTimestamp::from_millis(millis_from_timestamp);
        let timestamp_from_micros = UTCTimestamp::from_micros(micros_from_timestamp);
        let timestamp_from_nanos = UTCTimestamp::from_nanos(nanos_from_timestamp);
        assert!(timestamp_from_secs <= expected_timestamp);
        assert!(timestamp_from_millis <= expected_timestamp);
        assert!(timestamp_from_micros <= expected_timestamp);
        assert!(timestamp_from_nanos <= expected_timestamp);
        // test hashing
        hash_set.insert(expected_timestamp);
        assert!(hash_set.contains(&expected_timestamp));
        assert_eq!(
            &expected_timestamp,
            hash_set.get(&expected_timestamp).unwrap()
        );
    }

    // test from system time
    #[cfg(feature = "std")]
    let timestamp = UTCTimestamp::try_from_system_time().unwrap();
    #[cfg(not(feature = "std"))]
    let timestamp = UTCTimestamp::from_millis(1686824288903);
    assert!(timestamp <= UTCTimestamp::MAX);
    assert!(timestamp >= UTCTimestamp::ZERO);
    // test debug
    println!("{:?}", timestamp);
    // test default, clone & copy, ord
    assert_eq!(UTCTimestamp::default().clone(), UTCTimestamp::ZERO);
    let timestamp_copy = timestamp;
    assert_eq!(timestamp_copy, timestamp);
    assert_eq!(UTCTimestamp::ZERO, timestamp_copy.min(UTCTimestamp::ZERO));
    assert_eq!(UTCTimestamp::MAX, timestamp_copy.max(UTCTimestamp::MAX));
    // test operation methods
    assert_eq!(timestamp.saturating_add(UTCTimestamp::ZERO), timestamp);
    assert_eq!(
        timestamp.saturating_add(UTCTimestamp::MAX),
        UTCTimestamp::MAX
    );
    assert_eq!(timestamp.saturating_add_duration(Duration::ZERO), timestamp);
    assert_eq!(
        timestamp.saturating_add_duration(Duration::MAX),
        UTCTimestamp::MAX
    );
    assert_eq!(timestamp.saturating_add_secs(0), timestamp);
    assert_eq!(timestamp.saturating_add_secs(u64::MAX), UTCTimestamp::MAX);
    assert_eq!(
        timestamp.saturating_add_millis(1000),
        timestamp.saturating_add_secs(1)
    );
    assert_eq!(
        timestamp.saturating_add_micros(1000),
        timestamp.saturating_add_millis(1)
    );
    assert_eq!(
        timestamp.saturating_add_nanos(1000),
        timestamp.saturating_add_micros(1)
    );
    assert_eq!(timestamp.saturating_sub(UTCTimestamp::ZERO), timestamp);
    assert_eq!(
        timestamp.saturating_sub(UTCTimestamp::MAX),
        UTCTimestamp::ZERO
    );
    assert_eq!(timestamp.saturating_sub_duration(Duration::ZERO), timestamp);
    assert_eq!(
        timestamp.saturating_sub_duration(Duration::MAX),
        UTCTimestamp::ZERO
    );
    assert_eq!(timestamp.saturating_sub_secs(0), timestamp);
    assert_eq!(timestamp.saturating_sub_secs(u64::MAX), UTCTimestamp::ZERO);
    assert_eq!(
        timestamp.saturating_sub_millis(1000),
        timestamp.saturating_sub_secs(1)
    );
    assert_eq!(
        timestamp.saturating_sub_micros(1000),
        timestamp.saturating_sub_millis(1)
    );
    assert_eq!(
        timestamp.saturating_sub_nanos(1000),
        timestamp.saturating_sub_micros(1)
    );
    assert_eq!(timestamp.saturating_mul(u32::MIN), UTCTimestamp::ZERO);
    assert_eq!(
        timestamp.saturating_mul(u32::MAX).saturating_mul(u32::MAX),
        UTCTimestamp::MAX
    );
    assert_eq!(
        timestamp
            .checked_div(u32::MAX)
            .unwrap()
            .checked_div(u32::MAX),
        Some(UTCTimestamp::ZERO)
    );
    assert_eq!(timestamp.checked_div(u32::MIN), None);
    // test operation implementations
    let one = UTCTimestamp::from_nanos(1);
    let two = UTCTimestamp::from_nanos(2);
    let three = UTCTimestamp::from_nanos(3);
    let one_duration = Duration::from_nanos(1);
    assert_eq!(one + one, two);
    assert_eq!(one + one_duration, two);
    assert_eq!(two - one, one);
    assert_eq!(two - one_duration, one);
    assert_eq!(two * 1, two);
    assert_eq!(1 * two, two);
    assert_eq!(two / 2, one);
    let mut assign = UTCTimestamp::ZERO;
    assign += one_duration;
    assign += two;
    assert_eq!(assign, three);
    assign -= one;
    assign -= one_duration;
    assert_eq!(assign, one);
    assign *= 2;
    assert_eq!(assign, two);
    assign /= 2;
    assert_eq!(assign, one);
    Ok(())
}

#[test]
fn test_utc_day() -> Result<(), UTCError> {
    // test from system time
    #[cfg(feature = "std")]
    let utc_day = UTCDay::try_from_system_time().unwrap();
    #[cfg(not(feature = "std"))]
    let utc_day = UTCDay::from_millis(1686824288903);
    assert!(utc_day <= UTCDay::MAX);
    assert!(utc_day >= UTCDay::ZERO);
    // test debug
    println!("{:?} (days since epoch)", utc_day);
    // test from u64
    let u64_from_max = UTCDay::MAX.to_u64();
    let u64_invalid = u64_from_max + 1;
    assert!(UTCDay::try_from_u64(u64_from_max).is_ok());
    assert!(UTCDay::try_from(u64_invalid).is_err());
    // test from duration
    let duration_from_utc_day = utc_day.as_duration();
    let utc_day_from_duration = UTCDay::from_duration(duration_from_utc_day);
    assert_eq!(utc_day_from_duration, utc_day);
    assert_eq!(utc_day_from_duration, UTCDay::from(duration_from_utc_day));
    // test from timestamp
    let timestamp_from_utc_day = utc_day.as_timestamp();
    let utc_day_from_timestamp = UTCDay::from_timestamp(timestamp_from_utc_day);
    assert_eq!(utc_day_from_timestamp, utc_day);
    assert_eq!(utc_day_from_timestamp, UTCDay::from(timestamp_from_utc_day));
    // test unit conversions
    let secs_from_utc_day = utc_day.as_secs();
    let millis_from_utc_day = utc_day.as_millis() as u64;
    let micros_from_utc_day = utc_day.as_micros() as u64;
    let nanos_from_utc_day = utc_day.as_nanos() as u64;
    let utc_day_from_secs = UTCDay::from_secs(secs_from_utc_day);
    let utc_day_from_millis = UTCDay::from_millis(millis_from_utc_day);
    let utc_day_from_micros = UTCDay::from_micros(micros_from_utc_day);
    let utc_day_from_nanos = UTCDay::from_nanos(nanos_from_utc_day);
    assert_eq!(utc_day_from_secs, utc_day);
    assert!(utc_day_from_millis <= utc_day);
    assert!(utc_day_from_micros <= utc_day);
    assert!(utc_day_from_nanos <= utc_day);
    // test hashing
    let mut hash_set: HashSet<UTCDay> = HashSet::new();
    hash_set.insert(utc_day);
    assert!(hash_set.contains(&utc_day));
    assert_eq!(&utc_day, hash_set.get(&utc_day).unwrap());
    // test default, clone & copy, ord
    assert_eq!(UTCDay::default().clone(), UTCDay::ZERO);
    let utc_day_copy = utc_day;
    assert_eq!(utc_day_copy, utc_day);
    assert_eq!(UTCDay::ZERO, utc_day_copy.min(UTCDay::ZERO));
    assert_eq!(UTCDay::MAX, utc_day_copy.max(UTCDay::MAX));
    // test operation methods
    assert_eq!(utc_day.saturating_add(UTCDay::ZERO), utc_day);
    assert_eq!(utc_day.saturating_add(UTCDay::MAX), UTCDay::MAX);
    assert_eq!(
        utc_day.saturating_add(unsafe { UTCDay::from_u64_unchecked(u64::MAX) }),
        UTCDay::MAX
    );
    assert_eq!(utc_day.saturating_add_u64(0), utc_day);
    assert_eq!(utc_day.saturating_add_u64(u64::MAX), UTCDay::MAX);
    assert_eq!(utc_day.saturating_sub(UTCDay::ZERO), utc_day);
    assert_eq!(utc_day.saturating_sub(UTCDay::MAX), UTCDay::ZERO);
    assert_eq!(utc_day.saturating_sub_u64(0), utc_day);
    assert_eq!(utc_day.saturating_sub_u64(u64::MAX), UTCDay::ZERO);
    assert_eq!(utc_day.saturating_mul(u64::MIN), UTCDay::ZERO);
    assert_eq!(utc_day.saturating_mul(u64::MAX), UTCDay::MAX);
    assert_eq!(utc_day.checked_div(u64::MAX), Some(UTCDay::ZERO));
    assert_eq!(utc_day.checked_div(u64::MIN), None);
    // test operation implementations
    let one = UTCDay::try_from_u64(1)?;
    let two = UTCDay::try_from_u64(2)?;
    let three = UTCDay::try_from_u64(3)?;
    assert_eq!(one + one, two);
    assert_eq!(one + 1, two);
    assert_eq!(two - one, one);
    assert_eq!(two - 1, one);
    assert_eq!(two * 1, two);
    assert_eq!(1 * two, two);
    assert_eq!(two / 2, one);
    let mut assign = UTCDay::ZERO;
    assign += 1;
    assign += two;
    assert_eq!(assign, three);
    assign -= one;
    assign -= 1;
    assert_eq!(assign, one);
    assign *= 2;
    assert_eq!(assign, two);
    assign /= 2;
    assert_eq!(assign, one);
    Ok(())
}

#[test]
fn test_utc_tod() -> Result<(), UTCError> {
    // test from system time
    #[cfg(feature = "std")]
    let timestamp = UTCTimestamp::try_from_system_time().unwrap();
    #[cfg(not(feature = "std"))]
    let timestamp = UTCTimestamp::from_millis(1686824288903);
    let tod_from_timestamp = UTCTimeOfDay::from_timestamp(timestamp);
    // test from hhmmss
    let (hrs, mins, secs) = tod_from_timestamp.as_hhmmss();
    let subsec_ns = tod_from_timestamp.as_subsec_ns();
    let tod_from_hhmmss = UTCTimeOfDay::try_from_hhmmss(hrs, mins, secs, subsec_ns)?;
    assert_eq!(tod_from_hhmmss, tod_from_timestamp);
    assert_eq!(
        unsafe { UTCTimeOfDay::from_hhmmss_unchecked(hrs, mins, secs, subsec_ns) },
        tod_from_timestamp
    );
    assert!(UTCTimeOfDay::try_from_hhmmss(25, mins, secs, subsec_ns).is_err());
    assert!(UTCTimeOfDay::try_from_hhmmss(24, 0, 0, 0).is_err());
    assert!(UTCTimeOfDay::try_from_hhmmss(23, 59, 59, (NANOS_PER_SECOND - 1) as u32).is_ok());
    assert!(UTCTimeOfDay::try_from_hhmmss(u8::MAX, u8::MAX, u8::MAX, u32::MAX).is_err());
    // test iso conversions
    #[cfg(feature = "alloc")]
    {
        let iso_from_tod = tod_from_timestamp.as_iso_tod(9);
        let tod_from_iso = UTCTimeOfDay::try_from_iso_tod(&iso_from_tod)?;
        assert_eq!(tod_from_iso, tod_from_timestamp);
        assert_eq!(
            UTCTimeOfDay::try_from_iso_tod("T00:00:00Z")?,
            UTCTimeOfDay::ZERO
        );
        assert_eq!(
            UTCTimeOfDay::try_from_iso_tod("T23:59:59.999999999Z")?,
            UTCTimeOfDay::MAX
        );
        assert!(UTCTimeOfDay::try_from_iso_tod("Taa:59:59.999999999Z").is_err()); // invalid hour
        assert!(UTCTimeOfDay::try_from_iso_tod("T23:aa:59.999999999Z").is_err()); // invalid mins
        assert!(UTCTimeOfDay::try_from_iso_tod("T23:59:aa.999999999Z").is_err()); // invalid secs
        assert!(UTCTimeOfDay::try_from_iso_tod("T23:59:59.a99999999Z").is_err()); // invalid subsec
        assert!(UTCTimeOfDay::try_from_iso_tod("T23:59:59.9999999990Z").is_err());
        // invalid precision
    }
    // test no-alloc iso conversions
    let mut buf = [0; UTCTimeOfDay::iso_tod_len(9)];
    for precision in 0..13 {
        let written = tod_from_timestamp.write_iso_tod(&mut buf, precision)?;
        let iso_raw_str = core::str::from_utf8(&buf[..written]).unwrap();
        assert_eq!(iso_raw_str.len(), UTCTimeOfDay::iso_tod_len(precision));
        #[cfg(feature = "alloc")]
        assert_eq!(tod_from_timestamp.as_iso_tod(precision), iso_raw_str);
        // test maybe-invalid buf len
        let mut buf = [0; 5];
        let result = tod_from_timestamp.write_iso_tod(&mut buf, precision);
        if buf.len() < UTCTimeOfDay::iso_tod_len(precision) {
            assert!(result.is_err())
        } else {
            assert!(result.is_ok())
        }
    }

    // test unit conversions
    let secs_from_tod = tod_from_timestamp.as_secs();
    let millis_from_tod = tod_from_timestamp.as_millis();
    let micros_from_tod = tod_from_timestamp.as_micros();
    let nanos_from_tod = tod_from_timestamp.as_nanos();
    let tod_from_secs = UTCTimeOfDay::try_from_secs(secs_from_tod)?;
    let tod_from_millis = UTCTimeOfDay::try_from_millis(millis_from_tod)?;
    let tod_from_micros = UTCTimeOfDay::try_from_micros(micros_from_tod)?;
    let tod_from_nanos = UTCTimeOfDay::try_from_nanos(nanos_from_tod)?;
    assert!(UTCTimeOfDay::try_from_secs(SECONDS_PER_DAY as u32).is_err());
    assert!(UTCTimeOfDay::try_from_millis(MILLIS_PER_DAY as u32).is_err());
    assert!(UTCTimeOfDay::try_from_micros(MICROS_PER_DAY).is_err());
    assert!(UTCTimeOfDay::try_from_nanos(NANOS_PER_DAY).is_err());
    assert!(tod_from_secs <= tod_from_timestamp);
    assert!(tod_from_millis <= tod_from_timestamp);
    assert!(tod_from_micros <= tod_from_timestamp);
    assert_eq!(tod_from_nanos, tod_from_timestamp);
    assert_eq!(nanos_from_tod, tod_from_nanos.to_nanos());
    // test display, debug, default, clone & copy, ord
    println!("{:?}:{tod_from_timestamp}", tod_from_timestamp);
    assert_eq!(UTCTimeOfDay::default().clone(), UTCTimeOfDay::ZERO);
    let tod_copy = tod_from_timestamp;
    assert_eq!(tod_copy, tod_from_timestamp);
    assert_eq!(UTCTimeOfDay::ZERO, tod_copy.min(UTCTimeOfDay::ZERO));
    assert_eq!(UTCTimeOfDay::MAX, tod_copy.max(UTCTimeOfDay::MAX));
    // test hash
    let mut hash_set: HashSet<UTCTimeOfDay> = HashSet::new();
    hash_set.insert(tod_from_timestamp);
    assert!(hash_set.contains(&tod_from_timestamp));
    assert_eq!(
        &tod_from_timestamp,
        hash_set.get(&tod_from_timestamp).unwrap()
    );
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn test_time_serde() {
    let timestamp = UTCTimestamp::from_day(UTCDay::try_from_u64(19959).unwrap());
    let v = serde_json::to_value(&timestamp).unwrap();
    assert_eq!(timestamp, serde_json::from_value(v).unwrap());

    let day = UTCDay::try_from_u64(19959).unwrap();
    let v = serde_json::to_value(&day).unwrap();
    assert_eq!(day, serde_json::from_value(v).unwrap());

    let time_of_day = UTCTimeOfDay::try_from_hhmmss(17, 50, 23, 0).unwrap();
    let v = serde_json::to_value(&time_of_day).unwrap();
    assert_eq!(time_of_day, serde_json::from_value(v).unwrap());
}
