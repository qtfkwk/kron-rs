use kron::*;

const EPOCH_ISO8601: &str = "1970-01-01T00:00:00Z";

#[test]
fn epoch_display() {
    let kron = Kron::epoch();
    assert_eq!(format!("{kron}"), EPOCH_ISO8601);
    assert_eq!(kron.to_string(), EPOCH_ISO8601);
}

#[test]
fn epoch_timestamp() {
    let kron = Kron::epoch();
    assert_eq!(kron.dt.unix_timestamp(), 0);
    assert_eq!(kron.dt.unix_timestamp_nanos(), 0);
}

#[test]
fn epoch_format_iso8601() {
    assert_eq!(Kron::epoch().format(&ISO8601).unwrap(), EPOCH_ISO8601);
}

#[test]
fn epoch_format_compact() {
    assert_eq!(Kron::epoch().format(&COMPACT).unwrap(), "19700101-000000Z");
}

#[test]
fn epoch_custom_format() {
    let kron = Kron::epoch();
    let custom_fmt =
        KronFormat::new("[weekday repr:short] [month]/[day]/[year] [hour]:[minute]").unwrap();
    assert_eq!(kron.format(&custom_fmt).unwrap(), "Thu 01/01/1970 00:00");
}

#[test]
fn epoch_custom_format_str() {
    let kron = Kron::epoch();
    let custom_fmt_str = "[hour]:[minute]:[second] [month]/[day]/[year]";
    assert_eq!(
        kron.format_str(custom_fmt_str).unwrap(),
        "00:00:00 01/01/1970"
    );
}

#[test]
fn timestamp_0_display() {
    let kron = Kron::timestamp(0).unwrap();
    assert_eq!(format!("{kron}"), EPOCH_ISO8601);
    assert_eq!(kron.to_string(), EPOCH_ISO8601);
}

#[test]
fn timestamp_0_format() {
    assert_eq!(
        Kron::timestamp(0).unwrap().format(&ISO8601).unwrap(),
        EPOCH_ISO8601
    );
}
