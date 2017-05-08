use diesel::backend::Backend;
use diesel::types::{FromSql, IsNull, Text, ToSql};
use std::error::Error;
use std::io::Write;
use std::fmt;
use std::str::FromStr;
use super::entities::Mbid;

#[derive(Debug)]
struct MbidFromSqlError {}

impl fmt::Display for MbidFromSqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.description())
    }
}

impl Error for MbidFromSqlError {
    fn description(&self) -> &str
    {
        "Failed reading `Mbid` from SQL."
    }
}

impl<DB: Backend<RawValue = String>> FromSql<Text, DB> for Mbid {
    fn from_sql(value: Option<&String>) -> Result<Self, Box<Error + Send + Sync>>
    {
        let str_value = value.ok_or(MbidFromSqlError {})?.as_ref();
        Ok(Mbid::from_str(str_value)?)
    }
}

impl<DB: Backend<RawValue = String>> ToSql<Text, DB> for Mbid {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error + Send + Sync>>
    {
        write!(out, "{}", self)?;
        Ok(IsNull::No)
    }
}
