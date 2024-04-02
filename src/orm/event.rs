use chrono::prelude::*;
use diesel::dsl::{self, AsSelect, InnerJoin, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use super::account::AccountProfile;
use super::misc::Place;
use super::schema::*;

#[derive(Debug, Serialize, Deserialize, DbEnum)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::orm::schema::sql_types::Eventtype"]
pub enum EventType {
    Show,
    Lecture,
    Competition,
    Other,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub kind: EventType,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: i32,
    pub description: &'a str,
    pub organizer_id: i32,
    pub tickets: Option<i32>,
    pub registeration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(belongs_to(Place))]
#[diesel(belongs_to(Account))]
#[diesel(table_name = events)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub kind: EventType,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: i32,
    pub description: String,
    pub organizer_id: i32,
    pub tickets: Option<i32>,
    pub registeration_deadline: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = events)]
pub struct EventDisplay {
    pub id: i32,
    pub name: String,
    pub kind: EventType,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    #[diesel(embed)]
    pub venue: Place,
    pub description: String,
    #[diesel(embed)]
    pub organizer: AccountProfile,
    pub tickets: Option<i32>,
    pub registeration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = events)]
pub struct EventSummary {
    pub id: i32,
    pub name: String,
    pub kind: EventType,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    #[diesel(embed)]
    pub venue: Place,
    pub description: String,
    #[diesel(embed)]
    pub organizer: AccountProfile,
    pub tickets: Option<i32>,
    pub registeration_deadline: Option<NaiveDateTime>,
}

type Table = events::table;

type All = Select<Table, AsSelect<Event, Pg>>;
type Find = dsl::Find<All, i32>;
type FindAsDisplay =
    Select<InnerJoin<InnerJoin<Find, accounts::table>, places::table>, AsSelect<EventDisplay, Pg>>;
type FindAsSummary =
    Select<InnerJoin<InnerJoin<Find, accounts::table>, places::table>, AsSelect<EventSummary, Pg>>;

impl Event {
    pub fn all() -> All {
        events::table.select(Event::as_select())
    }

    pub fn find(id: i32) -> Find {
        Self::all().find(id)
    }

    pub fn find_as_display(id: i32) -> FindAsDisplay {
        Self::find(id)
            .inner_join(accounts::table)
            .inner_join(places::table)
            .select(EventDisplay::as_select())
    }

    pub fn find_as_summary(id: i32) -> FindAsSummary {
        Self::find(id)
            .inner_join(accounts::table)
            .inner_join(places::table)
            .select(EventSummary::as_select())
    }

    pub fn venue(&self) -> super::misc::PlaceFindName {
        Place::find(self.venue_id)
    }
}

impl<'a> NewEvent<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(events::table).values(self)
    }
}
