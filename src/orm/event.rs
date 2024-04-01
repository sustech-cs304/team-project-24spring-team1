use chrono::prelude::*;
use diesel::dsl::{AsSelect, Eq, Filter, Find, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel_derive_enum::DbEnum;
use serde::Deserialize;

use super::account::Account;
use super::misc::Place;
use super::schema::*;

#[derive(Debug, Deserialize, DbEnum)]
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
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

type Table = events::table;

type All = Select<Table, AsSelect<Event, Pg>>;

impl Event {
    pub fn all() -> All {
        events::table.select(Event::as_select())
    }

    pub fn venus(&self) -> super::misc::PlaceFindName {
        Place::find(self.venue_id)
    }
}

impl<'a> NewEvent<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(events::table).values(self)
    }
}
