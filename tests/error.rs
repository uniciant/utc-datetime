use core::fmt::Display;
use utc_dt::date::{UTCDate, UTCDateError};
use utc_dt::time::{UTCDay, UTCTimeOfDayError};
use utc_dt::{UTCDatetimeError, UTCError};

#[cfg(feature = "std")]
fn check_errors<T: std::error::Error + Display>(errors: &[T]) {
    for error in errors {
        print!("Error Display test: {error}");
        if let Some(source) = error.source() {
            print!(", caused by {source}");
        }
        print!("\n");
    }
}

#[cfg(not(feature = "std"))]
fn check_errors<T: Display>(errors: &[T]) {
    for error in errors {
        print!("Error Display test: {error}");
        print!("\n");
    }
}

#[test]
fn test_errors() {
    let utc_date_errors = [
        UTCDateError::ParseErr("a".parse::<u32>().unwrap_err()),
        UTCDateError::DateOutOfRange(UTCDate::MAX),
        UTCDateError::DayOutOfRange(UTCDate::MIN),
        UTCDateError::InvalidStrLen(30),
        UTCDateError::MonthOutOfRange(13),
        UTCDateError::YearOutOfRange(1969),
    ];
    check_errors(&utc_date_errors);
    let utc_tod_errors = [
        UTCTimeOfDayError::ParseErr("a".parse::<u32>().unwrap_err()),
        UTCTimeOfDayError::ExcessMicros(0),
        UTCTimeOfDayError::ExcessMillis(0),
        UTCTimeOfDayError::ExcessNanos(0),
        UTCTimeOfDayError::ExcessSeconds(0),
        UTCTimeOfDayError::ExcessPrecision(0),
        UTCTimeOfDayError::InsufficientStrLen(10, 20),
    ];
    check_errors(&utc_tod_errors);
    let utc_day_error = [UTCDay::try_from_u64(213_503_982_334_602).unwrap_err()];
    check_errors(&utc_day_error);
    let utc_datetime_errors = [
        utc_date_errors[0].clone().into(),
        utc_tod_errors[0].clone().into(),
        UTCDatetimeError::InsufficientStrLen(10, 20),
    ];
    check_errors(&utc_datetime_errors);
    let utc_errors: [UTCError; 4] = [
        utc_date_errors[1].clone().into(),
        utc_tod_errors[1].clone().into(),
        utc_day_error[0].clone().into(),
        utc_datetime_errors[0].clone().into(),
    ];
    check_errors(&utc_errors.clone());
}
