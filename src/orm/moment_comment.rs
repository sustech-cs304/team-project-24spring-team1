use chrono::prelude::*;
use diesel::dsl::{AsSelect, Eq, Filter, Find, InnerJoin, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use serde::Serialize;

use super::account::AccountCard;
use super::schema::*;
use super::utils::Update;

#[derive(Debug, Insertable)]
#[diesel(table_name = moment_comments)]
pub struct NewMomentComment<'a> {
    pub account_id: i32,
    pub moment_id: i32,
    pub content: &'a str,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(belongs_to(Account))]
#[diesel(belongs_to(Moment))]
#[diesel(table_name = moment_comments)]
pub struct MomentComment {
    pub id: i32,
    pub account_id: i32,
    pub moment_id: i32,
    pub content: String,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = moment_comments)]
pub struct MomentCommentDisplay {
    pub id: i32,
    #[diesel(embed)]
    pub account: AccountCard,
    pub content: String,
    pub created_at: NaiveDateTime,
}

type Table = moment_comments::table;

type RawAll = Select<Table, AsSelect<MomentComment, Pg>>;
type All = Filter<RawAll, Eq<moment_comments::is_deleted, bool>>;
type JoinAccount = InnerJoin<All, accounts::table>;
type UpdateAll = Update<Table>;
type WithId = Eq<moment_comments::id, i32>;
type WithMomentId = Eq<moment_comments::moment_id, i32>;
type FindId = Find<All, i32>;

type ByMomentId = Filter<All, WithMomentId>;
type ByMomentIdDisplay = Filter<JoinAccount, WithMomentId>;
type UpdateId = Filter<UpdateAll, WithId>;

impl MomentComment {
    pub fn all() -> All {
        moment_comments::table
            .select(MomentComment::as_select())
            .filter(moment_comments::is_deleted.eq(false))
    }

    pub fn by_moment_id(moment_id: i32) -> ByMomentId {
        Self::all().filter(moment_comments::moment_id.eq(moment_id))
    }

    pub fn by_moment_id_as_display(moment_id: i32) -> ByMomentIdDisplay {
        Self::all()
            .inner_join(accounts::table)
            .filter(moment_comments::moment_id.eq(moment_id))
    }

    pub fn find(id: i32) -> FindId {
        Self::all().find(id)
    }

    pub fn update(id: i32) -> UpdateId {
        diesel::update(moment_comments::table).filter(moment_comments::id.eq(id))
    }
}

impl<'a> NewMomentComment<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(moment_comments::table).values(self)
    }
}
