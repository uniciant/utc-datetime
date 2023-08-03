use core::time::Duration;

use anyhow::Result;

use utc_dt::{time::{UTCDay, UTCTimeOfDay, UTCTimestamp, UTCTransformations}, constants::NANOS_PER_SECOND};

#[test]
fn test_from_days_and_nanos() -> Result<()> {
    let test_cases = [
        (UTCTimestamp::from_nanos(0), UTCDay::ZERO, UTCTimeOfDay::try_from_secs(0)?, 4),
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

    for (expected_timestamp, utc_days, tod, weekday) in test_cases {
        let timestamp = UTCTimestamp::from_day_and_tod(utc_days, tod);
        assert_eq!(timestamp, expected_timestamp);
        assert_eq!(UTCDay::from_timestamp(timestamp), utc_days);
        assert_eq!(timestamp.as_tod(), tod);
        assert_eq!(utc_days.as_weekday(), weekday);
    }

    Ok(())
}
