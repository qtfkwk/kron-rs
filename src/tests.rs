use super::*;

const EPOCH_ISO8601: &str = "1970-01-01T00:00:00Z";

#[test]
fn epoch_display() {
    let kron = Kron::epoch();
    assert_eq!(format!("{kron}"), EPOCH_ISO8601);
}

#[test]
fn epoch_format() {
    assert_eq!(Kron::epoch().format(&ISO8601).unwrap(), EPOCH_ISO8601);
}

#[test]
fn epoch_timestamp() {
    let kron = Kron::epoch();
    assert_eq!(kron.dt.unix_timestamp(), 0);
    assert_eq!(kron.dt.unix_timestamp_nanos(), 0);
}

#[test]
fn timestamp_0_display() {
    let kron = Kron::timestamp(0).unwrap();
    assert_eq!(format!("{kron}"), EPOCH_ISO8601);
}

#[test]
fn timestamp_0_format() {
    assert_eq!(
        Kron::timestamp(0).unwrap().format(&ISO8601).unwrap(),
        EPOCH_ISO8601
    );
}
