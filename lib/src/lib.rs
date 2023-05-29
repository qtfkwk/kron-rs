#![doc = include_str!("../README.md")]

//--------------------------------------------------------------------------------------------------

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use time::OffsetDateTime;

//--------------------------------------------------------------------------------------------------

pub const FMT_COMPACT: &str = "[year][month][day]-[hour][minute][second]Z";
pub const FMT_ISO8601: &str = "[year]-[month]-[day]T[hour]:[minute]:[second]Z";
pub const FMT_ISO8601NS: &str = "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:9]Z";

//--------------------------------------------------------------------------------------------------

lazy_static! {
    pub static ref COMPACT: KronFormat<'static> = KronFormat::new(FMT_COMPACT).unwrap();
    pub static ref ISO8601: KronFormat<'static> = KronFormat::new(FMT_ISO8601).unwrap();
    pub static ref ISO8601NS: KronFormat<'static> = KronFormat::new(FMT_ISO8601NS).unwrap();
}

//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests;

//--------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct KronFormat<'a> {
    fmt: Vec<time::format_description::FormatItem<'a>>,
}

impl KronFormat<'_> {
    pub fn new(s: &str) -> Result<KronFormat> {
        Ok(KronFormat {
            fmt: time::format_description::parse(s)?,
        })
    }

    pub fn from(s: &str) -> Result<KronFormat> {
        match s {
            "COMPACT" => Ok(COMPACT.clone()),
            "ISO8601" => Ok(ISO8601.clone()),
            "ISO8601NS" => Ok(ISO8601NS.clone()),
            _ => KronFormat::new(s),
            //_ => Err(anyhow!("Invalid format: {s:?}")),
        }
    }
}

//--------------------------------------------------------------------------------------------------

pub struct Kron {
    pub dt: OffsetDateTime,
}

impl Kron {
    pub fn now() -> Kron {
        Kron {
            dt: OffsetDateTime::now_utc(),
        }
    }

    pub fn from(s: &str) -> Result<Kron> {
        if s.contains('.') && s.parse::<f64>().is_ok() {
            let mut p = s.split('.');
            let mut ts = Kron::timestamp(p.next().unwrap().parse::<i64>()?)?;
            let mut nanos = p.next().unwrap().to_string();
            while nanos.len() < 9 {
                nanos.push('0');
            }
            ts.dt = ts.dt.replace_nanosecond(nanos.parse::<u32>()?)?;
            Ok(ts)
        } else if let Ok(n) = s.parse::<i64>() {
            Kron::timestamp(n)
        } else {
            Err(anyhow!("Invalid timestamp: {s:?}"))
        }
    }

    pub fn timestamp(n: i64) -> Result<Kron> {
        Ok(Kron {
            dt: OffsetDateTime::from_unix_timestamp(n)?,
        })
    }

    pub fn epoch() -> Kron {
        Kron::timestamp(0).unwrap()
    }

    pub fn format(&self, f: &KronFormat) -> Result<String> {
        Ok(self.dt.format(&f.fmt)?)
    }

    pub fn format_str(&self, s: &'static str) -> Result<String> {
        self.format(&KronFormat::new(s)?)
    }
}

impl std::fmt::Display for Kron {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.dt.format(&ISO8601.fmt).unwrap())
    }
}
