#![doc = include_str!("../README.md")]

//--------------------------------------------------------------------------------------------------

use anyhow::Result;
use lazy_static::lazy_static;
use time::OffsetDateTime;

//--------------------------------------------------------------------------------------------------

pub const FMT_COMPACT: &str = "[year][month][day]-[hour][minute][second]Z";
pub const FMT_ISO8601: &str = "[year]-[month]-[day]T[hour]:[minute]:[second]Z";

//--------------------------------------------------------------------------------------------------

lazy_static! {
    pub static ref COMPACT: KronFormat = KronFormat::new(FMT_COMPACT).unwrap();
    pub static ref ISO8601: KronFormat = KronFormat::new(FMT_ISO8601).unwrap();
}

//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests;

//--------------------------------------------------------------------------------------------------

pub struct KronFormat {
    fmt: Vec<time::format_description::FormatItem<'static>>,
}

impl KronFormat {
    pub fn new(s: &'static str) -> Result<KronFormat> {
        Ok(KronFormat {
            fmt: time::format_description::parse(s)?,
        })
    }
}

//--------------------------------------------------------------------------------------------------

pub struct Kron {
    dt: OffsetDateTime,
}

impl Kron {
    pub fn now() -> Kron {
        Kron {
            dt: OffsetDateTime::now_utc(),
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
