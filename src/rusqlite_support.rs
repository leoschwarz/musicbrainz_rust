use super::entities::{Date, Mbid};
use rusqlite::Error as RusqliteError;
use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, Value, ValueRef};

impl FromSql for Date {
    fn column_result(value: ValueRef) -> Result<Self, FromSqlError>
    {
        match value {
            ValueRef::Text(s) => s.parse().map_err(|e| FromSqlError::Other(From::from(e))),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Date {
    fn to_sql(&self) -> Result<ToSqlOutput, RusqliteError>
    {
        let s = self.to_string();
        Ok(ToSqlOutput::Owned(Value::Text(s)))
    }
}

impl FromSql for Mbid {
    fn column_result(value: ValueRef) -> Result<Self, FromSqlError>
    {
        match value {
            ValueRef::Text(s) => s.parse().map_err(|e| FromSqlError::Other(From::from(e))),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Mbid {
    fn to_sql(&self) -> Result<ToSqlOutput, RusqliteError>
    {
        let s = self.to_string();
        Ok(ToSqlOutput::Owned(Value::Text(s)))
    }
}
