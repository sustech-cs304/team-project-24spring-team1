use chrono::prelude::*;
use diesel::dsl::{AsSelect, Eq, Filter, Find, InnerJoin, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use serde::Serialize;

use super::account::AccountCard;
use super::schema::*;
use super::utils::Update;

// CREATE TABLE moments (
//     id SERIAL PRIMARY KEY NOT NULL,
//     account_id INT NOT NULL REFERENCES accounts(id),
//     content TEXT NOT NULL,
//     is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
//     created_at TIMESTAMP NOT NULL DEFAULT NOW()
// );

#[derive(Debug, Insertable)]
#[diesel(table_name = moments)]
pub struct NewMoment<'a> {
    pub account_id: i32,
    pub content: &'a str,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = moments)]
pub struct Moment {
    pub id: i32,
    pub account_id: i32,
    pub content: String,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = moments)]
pub struct MomentDisplay {
    pub id: i32,
    #[diesel(embed)]
    pub account: AccountCard,
    pub content: String,
    pub created_at: NaiveDateTime,
}

type Table = moments::table;

type RawAll = Select<Table, AsSelect<Moment, Pg>>;
type All = Filter<RawAll, Eq<moments::is_deleted, bool>>;
type JoinAccount = InnerJoin<All, accounts::table>;
type AllDisplay = Select<JoinAccount, AsSelect<MomentDisplay, Pg>>;
type UpdateAll = Update<Table>;
type WithId = Eq<moments::id, i32>;
type FindId = Find<All, i32>;
type FindJoin = InnerJoin<FindId, accounts::table>;
type FindIdDisplay = Select<FindJoin, AsSelect<MomentDisplay, Pg>>;

type UpdateId = Filter<UpdateAll, WithId>;

impl Moment {
    pub fn all() -> All {
        moments::table
            .select(Moment::as_select())
            .filter(moments::is_deleted.eq(false))
    }

    pub fn all_display() -> AllDisplay {
        Self::all()
            .inner_join(accounts::table)
            .select(MomentDisplay::as_select())
    }

    pub fn find(id: i32) -> FindId {
        Self::all().find(id)
    }

    pub fn find_display(id: i32) -> FindIdDisplay {
        Self::find(id)
            .inner_join(accounts::table)
            .select(MomentDisplay::as_select())
    }

    pub fn update(id: i32) -> UpdateId {
        diesel::update(moments::table).filter(moments::id.eq(id))
    }
}

impl<'a> NewMoment<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(moments::table).values(self)
    }
}
