use chrono::prelude::*;
use diesel::dsl::{AliasedFields, AsSelect, CountStar, Desc, Eq, Filter, InnerJoin, Order, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel::query_source::Alias;
use serde::Serialize;

use crate::orm::account::AccountCard;
use crate::orm::chat::Chat;
use crate::orm::schema::*;

use super::{ByIsGroup, ChatCard};

#[derive(Debug, Insertable)]
#[diesel(table_name = chat_members)]
pub struct NewChatMember {
    pub chat_id: i32,
    pub account_id: i32,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(belongs_to(Account))]
#[diesel(belongs_to(Chat))]
#[diesel(table_name = chat_members)]
#[diesel(primary_key(chat_id, account_id))]
pub struct ChatMember {
    pub chat_id: i32,
    pub account_id: i32,
    pub last_read: NaiveDateTime,
}

#[derive(Debug, Selectable, Serialize, Queryable)]
pub struct ChatMemberDisplay {
    #[diesel(embed)]
    #[serde(flatten)]
    pub account: AccountCard,
}

#[derive(Debug, Selectable, Serialize, Queryable)]
pub struct ChatItemDisplay {
    #[diesel(embed)]
    #[serde(flatten)]
    pub chat: ChatCard,
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
type ByAccountIdDisplay = Select<
    InnerJoin<Order<ByAccountId, Desc<chat_members::last_read>>, chats::table>,
    AsSelect<ChatItemDisplay, Pg>,
>;
type ByChatId = Filter<All, Eq<chat_members::chat_id, i32>>;
type ByChatIdDisplay =
    Select<InnerJoin<ByChatId, accounts::table>, AsSelect<ChatMemberDisplay, Pg>>;
type CheckInsideChat = Select<Filter<ByAccountId, Eq<chat_members::chat_id, i32>>, CountStar>;
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

    pub fn by_account_id_display(account_id: i32) -> ByAccountIdDisplay {
        Self::by_account_id(account_id)
            .inner_join(chats::table)
            .order(chat_members::last_read.desc())
            .select(ChatItemDisplay::as_select())
    }

    pub fn by_chat_id(chat_id: i32) -> ByChatId {
        Self::all().filter(chat_members::chat_id.eq(chat_id))
    }

    pub fn by_chat_id_display(chat_id: i32) -> ByChatIdDisplay {
        Self::by_chat_id(chat_id)
            .inner_join(accounts::table)
            .select(ChatMemberDisplay::as_select())
    }

    pub fn check_in_chat(account_id: i32, chat_id: i32) -> CheckInsideChat {
        Self::by_account_id(account_id)
            .filter(chat_members::chat_id.eq(chat_id))
            .count()
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
