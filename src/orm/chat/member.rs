use chrono::prelude::*;
use diesel::dsl::{AliasedFields, AsSelect, Eq, Filter, InnerJoin, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel::query_source::Alias;

use crate::orm::chat::Chat;
use crate::orm::schema::*;

use super::ByIsGroup;

#[derive(Debug, Insertable)]
#[diesel(table_name = chat_members)]
pub struct NewChatMember {
    pub chat_id: i32,
    pub account_id: i32,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(table_name = chat_members)]
#[diesel(primary_key(chat_id, account_id))]
pub struct ChatMember {
    pub chat_id: i32,
    pub account_id: i32,
    pub last_read: NaiveDateTime,
}

#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = chat_members)]
pub struct ChatMemberChangeset {
    pub last_read: Option<NaiveDateTime>,
}

type Table = chat_members::table;

diesel::alias!(
    chat_members as member1: ChatMemberAlias1,
    chat_members as member2: ChatMemberAlias2,
);

type All = Select<Table, AsSelect<ChatMember, Pg>>;
type ByAccountId = Filter<All, Eq<chat_members::account_id, i32>>;
type ByChatId = Filter<All, Eq<chat_members::chat_id, i32>>;
type ByFriendChat = Select<
    Filter<
        Filter<
            InnerJoin<InnerJoin<ByIsGroup, Alias<ChatMemberAlias1>>, Alias<ChatMemberAlias2>>,
            Eq<AliasedFields<ChatMemberAlias1, chat_members::account_id>, i32>,
        >,
        Eq<AliasedFields<ChatMemberAlias2, chat_members::account_id>, i32>,
    >,
    chats::id,
>;

impl ChatMember {
    pub fn all() -> All {
        chat_members::table.select(ChatMember::as_select())
    }

    pub fn by_account_id(account_id: i32) -> ByAccountId {
        Self::all().filter(chat_members::account_id.eq(account_id))
    }

    pub fn by_chat_id(chat_id: i32) -> ByChatId {
        Self::all().filter(chat_members::chat_id.eq(chat_id))
    }

    pub fn by_friend_chat(members: &[i32; 2]) -> ByFriendChat {
        Chat::by_is_group(false)
            .inner_join(member1)
            .inner_join(member2)
            .filter(member1.field(chat_members::account_id).eq(members[0]))
            .filter(member2.field(chat_members::account_id).eq(members[1]))
            .select(chats::id)
    }
}

impl NewChatMember {
    pub fn new(chat_id: i32, account_id: i32) -> Self {
        Self {
            chat_id,
            account_id,
        }
    }

    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(chat_members::table).values(self)
    }
}
