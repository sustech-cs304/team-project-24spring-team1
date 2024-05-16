use chrono::prelude::*;
use diesel::dsl::{AsSelect, Eq, Filter, Find, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::*;
use super::utils::Update;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, DbEnum)]
#[serde(rename_all = "snake_case")]
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
    pub email: &'a str,
    pub password: Option<&'a str>,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i32,
    pub sustech_id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar: Uuid,
    pub role: Role,
    pub bio: String,
    pub registered_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Selectable, Queryable)]
#[diesel(table_name = accounts)]
pub struct AccountCredential {
    pub id: i32,
    pub password: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = accounts)]
pub struct AccountCard {
    pub id: i32,
    pub name: String,
    pub avatar: Uuid,
    pub role: Role,
}

#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = accounts)]
pub struct AccountProfile {
    pub id: i32,
    pub sustech_id: i32,
    pub name: String,
    pub email: String,
    pub avatar: Uuid,
    pub role: Role,
    pub bio: String,
}

#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountProfileChangeset<'a> {
    pub email: Option<&'a str>,
    pub bio: Option<&'a str>,
    pub avatar: Option<Uuid>,
}

type Table = accounts::table;

type All = Select<Table, AsSelect<Account, Pg>>;
type UpdateAll = Update<Table>;
type WithId = Eq<accounts::id, i32>;
type WithName<'a> = Eq<accounts::name, &'a str>;
type WithStudentId = Eq<accounts::sustech_id, i32>;
type FindId = Find<All, i32>;

type ByName<'a> = Filter<All, WithName<'a>>;
type ByStudentId = Filter<All, WithStudentId>;
type UpdateId = Filter<UpdateAll, WithId>;

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

    pub fn update(id: i32) -> UpdateId {
        diesel::update(accounts::table).filter(accounts::id.eq(id))
    }
}

impl<'a> NewAccount<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(accounts::table).values(self)
    }
}

impl Role {
    pub fn has_access_to(self, role: Role) -> bool {
        self <= role
    }
}
