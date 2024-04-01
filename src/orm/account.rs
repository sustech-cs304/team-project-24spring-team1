use chrono::prelude::*;
use diesel::dsl::{AsSelect, Eq, Filter, Find, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel_derive_enum::DbEnum;

use super::schema::*;

#[derive(Debug, DbEnum)]
#[ExistingTypePath = "crate::orm::schema::sql_types::Role"]
pub enum Role {
    Admin,
    Staff,
    Student,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub sustech_id: i32,
    pub name: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub sustech_id: i32,
    pub name: String,
    pub password: String,
    pub role: Role,
    pub registered_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = accounts)]
pub struct AccountCredential {
    pub id: i32,
    pub password: String,
}

type Table = accounts::table;

type All = Select<Table, AsSelect<Account, Pg>>;
type WithName<'a> = Eq<accounts::name, &'a str>;
type ByName<'a> = Filter<All, WithName<'a>>;
type WithStudentId = Eq<accounts::sustech_id, i32>;
type ByStudentId = Filter<All, WithStudentId>;
type FindId = Find<All, i32>;

impl Account {
    pub fn all() -> All {
        accounts::table.select(Account::as_select())
    }

    pub fn by_name(name: &str) -> ByName<'_> {
        Self::all().filter(accounts::name.eq(name))
    }

    pub fn by_sustech_id(sustech_id: i32) -> ByStudentId {
        Self::all().filter(accounts::sustech_id.eq(sustech_id))
    }

    pub fn find(id: i32) -> FindId {
        Self::all().find(id)
    }
}

impl<'a> NewAccount<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(accounts::table).values(self)
    }
}
