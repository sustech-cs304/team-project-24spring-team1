use diesel::query_builder::QueryId;
use diesel::sql_types::SqlType;

#[derive(QueryId, SqlType)]
#[diesel(postgres_type(name = "eventtype"))]
pub struct Eventtype;

#[derive(QueryId, SqlType)]
#[diesel(postgres_type(name = "role"))]
pub struct Role;

#[derive(QueryId, SqlType)]
#[diesel(postgres_type(oid = 600, array_oid = 1017))]
pub struct Point;
