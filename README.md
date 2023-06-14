# UTC Datetime
Simple and fast UTC dates, timestamps and datetimes library for Rust.

## NOTE
Only capable of expressing times and dates SINCE the Unix Epoch `1970/01/01 00:00:00`. This library takes advantage of this assumption to simplify the API and internal logic.

## References
Date/timestamp conversion algorithims have been based upon Howard Hinnant's paper:
[`chrono`-Compatible Low-Level Date Algorithms](http://howardhinnant.github.io/date_algorithms.html)

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
* [MIT License](https://opensource.org/licenses/MIT)

at your option.