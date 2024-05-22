use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use diesel::pg::{Pg, PgValue};
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::expression::AsExpression;
use serde::{Deserialize, Serialize};

use super::sql_types::Point as SqlPoint;

/// Point is represented in Postgres as a tuple of 64 bit floating point values (x, y).  This
/// struct is a dumb wrapper type, meant only to indicate the tuple's meaning.
#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = SqlPoint)]
pub struct Point(pub f64, pub f64);

impl FromSql<SqlPoint, Pg> for Point {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        let mut bytes = value.as_bytes();
        let x = bytes.read_f64::<NetworkEndian>()?;
        let y = bytes.read_f64::<NetworkEndian>()?;
        Ok(Point(x, y))
    }
}

impl ToSql<SqlPoint, Pg> for Point {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_f64::<NetworkEndian>(self.0)?;
        out.write_f64::<NetworkEndian>(self.1)?;
        Ok(IsNull::No)
    }
}
