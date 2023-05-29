# About

Date/time CLI utility

*See also the [`kron-lib`] crate.*

[`kron-lib`]: https://crates.io/crates/kron-lib

# Usage

```
$ kron -h
Date/time CLI utility

Usage: kron [OPTIONS] [ARG]...

Arguments:
  [ARG]...  Argument ["%s[%.f]" timestamp, default: now]

Options:
  -f, --format <FORMAT>  Format (COMPACT, ISO8601, ISO8601NS) [default: ISO8601]
  -r, --readme           Print readme
  -h, --help             Print help
  -V, --version          Print version
```

```
$ kron -V
kron 2.1.0
```

# Examples

## Now

```
$ kron
2023-05-29T14:43:19Z
```

## Alternate formats

```
$ kron -f COMPACT
20230529-144319Z
```

```
$ kron -f ISO8601NS
2023-05-29T14:43:19.240644157Z
```

## Custom formats

```
$ kron -f '[hour]:[minute]'
14:43
```

```
$ kron -f '[weekday repr:long], [month repr:long] [day], [year]'
Monday, May 29, 2023
```

## Single timestamp

```
$ kron 0
1970-01-01T00:00:00Z
```

## Multiple timestamps

```
$ kron 0 3600 7200
1970-01-01T00:00:00Z
1970-01-01T01:00:00Z
1970-01-01T02:00:00Z
```

## Subsecond timestamp

```
$ kron -f ISO8601NS 626609862.1234
1989-11-09T10:17:42.123400000Z
```

## Invalid timestamp

```
$ kron not-a-timestamp
Error: Invalid timestamp: "not-a-timestamp"
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

