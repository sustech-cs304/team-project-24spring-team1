use chrono::prelude::*;
use diesel::dsl::{AsSelect, Asc, Eq, Filter, Find, InnerJoin, Order, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use serde::Serialize;

use super::account::AccountCard;
use super::schema::*;
use super::utils::Update;

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
type AllOrdered = Order<All, Asc<comments::id>>;
type JoinAccount = InnerJoin<AllOrdered, accounts::table>;
type UpdateAll = Update<Table>;
type WithId = Eq<comments::id, i32>;
type WithEventId = Eq<comments::event_id, i32>;
type FindId = Find<All, i32>;

type ByEventId = Filter<AllOrdered, WithEventId>;
type ByEventIdDisplay = Filter<JoinAccount, WithEventId>;
type UpdateId = Filter<UpdateAll, WithId>;

impl Comment {
    pub fn all() -> All {
        comments::table
            .select(Comment::as_select())
            .filter(comments::is_deleted.eq(false))
    }

    pub fn all_ordered() -> AllOrdered {
        comments::table
            .select(Comment::as_select())
            .filter(comments::is_deleted.eq(false))
            .order(comments::id.asc())
    }

    pub fn by_event_id(event_id: i32) -> ByEventId {
        Self::all_ordered().filter(comments::event_id.eq(event_id))
    }

    pub fn by_event_id_as_display(event_id: i32) -> ByEventIdDisplay {
        Self::all_ordered()
            .inner_join(accounts::table)
            .filter(comments::event_id.eq(event_id))
    }

    pub fn find(id: i32) -> FindId {
        Self::all().find(id)
    }

    pub fn update(id: i32) -> UpdateId {
        diesel::update(comments::table).filter(comments::id.eq(id))
    }
}

impl<'a> NewComment<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(comments::table).values(self)
    }
}
