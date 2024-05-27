use chrono::prelude::*;
use diesel::dsl::{AsSelect, Eq, Filter, InnerJoin, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use serde::Serialize;

use super::account::AccountCard;
use super::schema::*;

#[derive(Debug, Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment<'a> {
    pub account_id: i32,
    pub event_id: i32,
    pub content: &'a str,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(belongs_to(Account))]
#[diesel(belongs_to(Event))]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32,
    pub account_id: i32,
    pub event_id: i32,
    pub content: String,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = comments)]
pub struct CommentDisplay {
    pub id: i32,
    #[diesel(embed)]
    pub account: AccountCard,
    pub content: String,
    pub created_at: NaiveDateTime,
}

type Table = comments::table;

type RawAll = Select<Table, AsSelect<Comment, Pg>>;
type All = Filter<RawAll, Eq<comments::is_deleted, bool>>;
type JoinAccount = InnerJoin<All, accounts::table>;
type WithEventId = Eq<comments::event_id, i32>;

type ByEventId = Filter<All, WithEventId>;
type ByEventIdDisplay = Filter<JoinAccount, WithEventId>;

impl Comment {
    pub fn all() -> All {
        comments::table
            .select(Comment::as_select())
            .filter(comments::is_deleted.eq(false))
    }

    pub fn by_event_id(event_id: i32) -> ByEventId {
        Self::all().filter(comments::event_id.eq(event_id))
    }

    pub fn by_event_id_as_display(event_id: i32) -> ByEventIdDisplay {
        Self::all()
            .inner_join(accounts::table)
            .filter(comments::event_id.eq(event_id))
    }
}

impl<'a> NewComment<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(comments::table).values(self)
    }
}
