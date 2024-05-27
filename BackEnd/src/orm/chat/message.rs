use chrono::prelude::*;
use diesel::dsl::{AsSelect, Eq, Filter, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use serde::Serialize;

use crate::orm::schema::*;

#[derive(Debug, Insertable)]
#[diesel(table_name = chat_messages)]
pub struct NewChatMessage<'a> {
    pub chat_id: i32,
    pub account_id: i32,
    pub content: &'a str,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(belongs_to(Chat))]
#[diesel(belongs_to(Account))]
#[diesel(table_name = chat_messages)]
pub struct ChatMessage {
    pub id: i32,
    pub chat_id: i32,
    pub account_id: i32,
    pub content: String,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Selectable, Serialize, Queryable)]
#[diesel(table_name = chat_messages)]
pub struct ChatMessageDisplay {
    pub id: i32,
    pub account_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

type Table = chat_messages::table;

type RawAll = Select<Table, AsSelect<ChatMessage, Pg>>;
type All = Filter<RawAll, Eq<chat_messages::is_deleted, bool>>;
type ByChatId = Filter<All, Eq<chat_messages::chat_id, i32>>;
type ByChatIdDisplay = Select<ByChatId, AsSelect<ChatMessageDisplay, Pg>>;

impl ChatMessage {
    pub fn all() -> All {
        chat_messages::table
            .select(ChatMessage::as_select())
            .filter(chat_messages::is_deleted.eq(false))
    }

    pub fn by_chat_id(chat_id: i32) -> ByChatId {
        Self::all().filter(chat_messages::chat_id.eq(chat_id))
    }

    pub fn by_chat_id_display(chat_id: i32) -> ByChatIdDisplay {
        Self::by_chat_id(chat_id).select(ChatMessageDisplay::as_select())
    }
}

impl<'a> NewChatMessage<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(chat_messages::table).values(self)
    }
}
