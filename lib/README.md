# About

Date/time library

*See also the [`kron`] crate.*

[`kron`]: https://crates.io/crates/kron

# Example

```rust
use kron_lib::*;

// Get current timestamp
let now = Kron::now();

// Print it ~ "2023-05-26T13:07:42Z\n"
println!("{now}");

// Get a specific timestamp (0 = UNIX epoch)
let ts = Kron::timestamp(626609862).unwrap();

// Default format (ISO8601)
assert_eq!(format!("{ts}"), "1989-11-09T10:17:42Z");
assert_eq!(ts.to_string(), "1989-11-09T10:17:42Z");

// Format constant
assert_eq!(ts.format(&COMPACT).unwrap(), "19891109-101742Z");
assert_eq!(ts.format(&ISO8601).unwrap(), "1989-11-09T10:17:42Z");

// Create a custom format and use it
let custom_fmt = KronFormat::new(
    "[weekday repr:short] [month]/[day]/[year] [hour]:[minute]",
).unwrap();
assert_eq!(ts.format(&custom_fmt).unwrap(), "Thu 11/09/1989 10:17");

// Use a custom format string directly
assert_eq!(
    ts.format_str("[hour]:[minute]:[second] [month]/[day]/[year]").unwrap(),
    "10:17:42 11/09/1989",
);

// Get the UNIX timestamp in seconds/nanoseconds by accessing the internal
// `time::OffsetDateTime`
assert_eq!(ts.dt.unix_timestamp(), 626609862);
assert_eq!(ts.dt.unix_timestamp_nanos(), 626609862000000000);
```

# Format syntax

Component | Description
---|---
`[day {padding:{zero,space,none}}]` | Day of month
`[hour {padding:{zero,space,none},repr:{12,24}}]` | Clock hour
`[minute {padding:{zero,space,none}}]` | Minute within the clock hour
`[month {padding:{zero,space,none},repr:{numerical,long,short},case_sensitive:{true,false}}]` | Month
`[offset_hour {padding:{zero,space,none},sign:{automatic,mandatory}}]` | Whole hours offset from UTC
`[offset_minute {padding:{zero,space,none}]` | Minutes within the hour offset from UTC
`[offset_second {padding:{zero,space,none}]` | Seconds within the minute offset from UTC
`[ordinal {padding:{zero,space,none}}]` | Day of year
`[period {case:{lower,upper},case_sensitive:{true,false}}]` | AM/PM
`[second {padding:{zero,space,none}}]` | Second within the clock minute
`[subsecond {digits:1+,1,2,3,4,5,6,7,8,9}]` | Subsecond within the clock second
`[unix_timestamp {precision:{second,millisecond,microsecond,nanosecond},sign:{automatic,mandatory}}]` | Unix timestamp
`[week_number {padding:{zero,space,none},repr:{iso,sunday,monday}}]` | Week of the year
`[weekday {repr:{long,short,sunday,monday},one_indexed:{false,true},case_sensitive:{true,false}}]` | Day of the week
`[year {padding:{zero,space,none},repr:{full,last_two},base:{calendar,iso_week},sign:{automatic,mandatory}}]` | Year

See <https://time-rs.github.io/book/api/format-description.html> for additional
details.

# Changelog

Please find the [`CHANGELOG.md`] in the [repository].

[`CHANGELOG.md`]: https://github.com/qtfkwk/kron-rs/blob/main/CHANGELOG.md
[repository]: https://github.com/qtfkwk/kron-rs/

