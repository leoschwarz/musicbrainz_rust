use diesel::Queryable;
use diesel::backend::Backend;
use diesel::expression::bound::Bound;
use diesel::expression::AsExpression;
use diesel::row::Row;
use diesel::types::{FromSql, FromSqlRow, HasSqlType, IsNull, Nullable, Text, ToSql};
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

impl<DB: Backend<RawValue = [u8]>> FromSql<Text, DB> for Mbid {
    fn from_sql(value: Option<&[u8]>) -> Result<Self, Box<Error + Send + Sync>>
    {
        let string_value: String = FromSql::<Text, DB>::from_sql(value)?;
        Ok(Mbid::from_str(string_value.as_ref())?)
    }
}

impl<DB: Backend<RawValue = Text>> ToSql<Nullable<Text>, DB> for Mbid {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error + Send + Sync>>
    {
        write!(out, "{}", self)?;
        Ok(IsNull::No)
    }
}

impl AsExpression<Text> for Mbid {
    type Expression = Bound<Text, Self>;

    fn as_expression(self) -> Self::Expression
    {
        Bound::new(self)
    }
}

impl<'expr> AsExpression<Text> for &'expr Mbid {
    type Expression = Bound<Text, Self>;

    fn as_expression(self) -> Self::Expression
    {
        Bound::new(self)
    }
}

impl AsExpression<Nullable<Text>> for Mbid {
    type Expression = Bound<Nullable<Text>, Self>;

    fn as_expression(self) -> Self::Expression
    {
        Bound::new(self)
    }
}

impl<'expr> AsExpression<Nullable<Text>> for &'expr Mbid {
    type Expression = Bound<Nullable<Text>, Self>;

    fn as_expression(self) -> Self::Expression
    {
        Bound::new(self)
    }
}

impl<ST, DB> FromSqlRow<ST, DB> for Mbid
    where DB: Backend + HasSqlType<ST>,
          Mbid: Queryable<ST, DB>
{
    fn build_from_row<T: Row<DB>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>>
    {
        let row = <<Mbid as Queryable<ST, DB>>::Row as FromSqlRow<ST, DB>>::build_from_row(row)?;
        Ok(Mbid::build(row))
    }
}
