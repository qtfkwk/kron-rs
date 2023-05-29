use super::*;

const EPOCH_ISO8601: &str = "1970-01-01T00:00:00Z";
const EPOCH_ISO8601NS: &str = "1970-01-01T00:00:00.000000000Z";

#[test]
fn epoch_display() {
    let kron = Kron::epoch();
    assert_eq!(format!("{kron}"), EPOCH_ISO8601);
    assert_eq!(format!("{}", kron), EPOCH_ISO8601);
    assert_eq!(kron.to_string(), EPOCH_ISO8601);
}

#[test]
fn epoch_format_iso8601() {
    assert_eq!(Kron::epoch().format(&ISO8601).unwrap(), EPOCH_ISO8601);
}

#[test]
fn epoch_format_iso8601ns() {
    assert_eq!(Kron::epoch().format(&ISO8601NS).unwrap(), EPOCH_ISO8601NS);
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
    let kron = Kron::timestamp(0).unwrap();
    assert_eq!(kron.format(&ISO8601).unwrap(), EPOCH_ISO8601);
    assert_eq!(kron.format(&ISO8601NS).unwrap(), EPOCH_ISO8601NS);
}
