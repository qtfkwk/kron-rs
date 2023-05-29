# About

Kron consists of a library crate ([`kron-lib`]) and a binary crate ([`kron`])
that each provide an interface to using dates and times.

[`kron-lib`]: lib
[`kron`]: cli

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

