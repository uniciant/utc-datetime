//! Constants module.
//!
//! Provides useful time constants for transformations.

/** Hours per day       */ pub const HOURS_PER_DAY: u64 = 24;
/** Minutes per day     */ pub const MINUTES_PER_DAY: u64 = HOURS_PER_DAY * 60;
/** Seconds per day     */ pub const SECONDS_PER_DAY: u64 = MINUTES_PER_DAY * 60;
/** Milliseconds per day*/ pub const MILLIS_PER_DAY: u64 = SECONDS_PER_DAY * 1000;
/** Microseconds per day*/ pub const MICROS_PER_DAY: u64 = MILLIS_PER_DAY * 1000;
/** Nanoseconds per day */ pub const NANOS_PER_DAY: u64 = MICROS_PER_DAY * 1000;

/** Minutes per hour        */ pub const MINUTES_PER_HOUR: u64 = 60;
/** Seconds per hour        */ pub const SECONDS_PER_HOUR: u64 = MINUTES_PER_HOUR * 60;
/** Milliseconds per hour   */ pub const MILLIS_PER_HOUR: u64 = SECONDS_PER_HOUR * 1000;
/** Microseconds per hour   */ pub const MICROS_PER_HOUR: u64 = MILLIS_PER_HOUR * 1000;
/** Nanoseconds per hour    */ pub const NANOS_PER_HOUR: u64 = MICROS_PER_HOUR * 1000;

/** Seconds per minute      */ pub const SECONDS_PER_MINUTE: u64 = 60;
/** Milliseconds per minute */ pub const MILLIS_PER_MINUTE: u64 = SECONDS_PER_MINUTE * 1000;
/** Microseconds per minute */ pub const MICROS_PER_MINUTE: u64 = MILLIS_PER_MINUTE * 1000;
/** Nanoseconds per minute  */ pub const NANOS_PER_MINUTE: u64 = MICROS_PER_MINUTE * 1000;

/** Milliseconds per second */ pub const MILLIS_PER_SECOND: u64 = 1000;
/** Microseconds per second */ pub const MICROS_PER_SECOND: u64 = MILLIS_PER_SECOND * 1000;
/** Microseconds per second */ pub const NANOS_PER_SECOND: u64 = MICROS_PER_SECOND * 1000;

/** Microseconds per millisecond*/ pub const MICROS_PER_MILLI: u64 = 1000;
/** Nanoseconds per millisecond */ pub const NANOS_PER_MILLI: u64 = MICROS_PER_MILLI * 1000;

/** Nanoseconds per microsecond */ pub const NANOS_PER_MICRO: u64 = 1000;