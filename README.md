```
use kron::*;

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
```

