use chrono::prelude::*;
use diesel::dsl::{AsSelect, Eq, Filter, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use serde::Serialize;

pub mod member;
pub mod message;

use super::schema::*;

#[derive(Debug, Insertable)]
#[diesel(table_name = chats)]
pub struct NewChat<'a> {
    pub is_group: bool,
    pub name: &'a str,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(table_name = chats)]
pub struct Chat {
    pub id: i32,
    pub name: String,
    pub is_group: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Selectable, Queryable, Serialize)]
#[diesel(table_name = chats)]
pub struct ChatCard {
    pub id: i32,
    pub name: String,
    pub is_group: bool,
}

type Table = chats::table;

type All = Select<Table, AsSelect<Chat, Pg>>;
type ByIsGroup = Filter<All, Eq<chats::is_group, bool>>;

impl Chat {
    pub fn all() -> All {
        chats::table.select(Chat::as_select())
    }

    pub fn by_is_group(is_group: bool) -> ByIsGroup {
        Self::all().filter(chats::is_group.eq(is_group))
    }
}

impl<'a> NewChat<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(chats::table).values(self)
    }
}
