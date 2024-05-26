use chrono::prelude::*;
use diesel::dsl::{self, AsSelect, Eq, Filter, InnerJoin, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::account::AccountCard;
use super::misc::Place;
use super::schema::*;
use super::utils::types::Point;
use super::utils::{Bracket, BracketDsl, CountReferencesDsl, CountReferencesIn, Update};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::orm::utils::sql_types::Eventtype"]
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
    pub description: &'a str,
    pub cover: Option<Uuid>,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: i32,
    pub location: Point,
    pub organizer_id: i32,
    pub tickets: Option<i32>,
    pub registration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Selectable, Identifiable, Queryable)]
#[diesel(belongs_to(Place))]
#[diesel(belongs_to(Account))]
#[diesel(table_name = events)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub kind: EventType,
    pub description: String,
    pub cover: Uuid,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub venue_id: i32,
    pub location: Point,
    pub organizer_id: i32,
    pub tickets: Option<i32>,
    pub registration_deadline: Option<NaiveDateTime>,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = events)]
pub struct EventDisplay {
    pub id: i32,
    pub name: String,
    pub kind: EventType,
    pub description: String,
    pub cover: Uuid,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    #[diesel(embed)]
    pub venue: Place,
    pub location: Point,
    #[diesel(embed)]
    pub organizer: AccountCard,
    pub tickets: Option<i32>,
    pub registration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = events)]
pub struct EventSummary {
    pub id: i32,
    pub name: String,
    pub kind: EventType,
    pub description: String,
    pub cover: Uuid,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    #[diesel(embed)]
    pub venue: Place,
    pub location: Point,
    #[diesel(embed)]
    pub organizer: AccountCard,
    pub tickets: Option<i32>,
    pub registration_deadline: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Queryable)]
pub struct EventWithParticipation<T: Serialize> {
    pub participation_count: i64,
    #[serde(flatten)]
    pub inner: T,
}

#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = events)]
pub struct EventChangeset {
    pub is_deleted: Option<bool>,
}

type Table = events::table;

type RawAll = Select<Table, AsSelect<Event, Pg>>;
type All = Filter<RawAll, Eq<events::is_deleted, bool>>;
type UpdateAll = Update<Table>;
type WithId = Eq<events::id, i32>;
type Find = dsl::Find<All, i32>;
type FindJoin = InnerJoin<InnerJoin<Find, accounts::table>, places::table>;
type ParticipationCount = Bracket<CountReferencesIn<events::id, participation::event_id>>;
type FindWithParticipationAs<T> = Select<FindJoin, (ParticipationCount, AsSelect<T, Pg>)>;
type Join = InnerJoin<InnerJoin<All, accounts::table>, places::table>;
type JoinWithParticipation<T> = Select<Join, (ParticipationCount, AsSelect<T, Pg>)>;
type UpdateId = Filter<UpdateAll, WithId>;

impl Event {
    pub fn all() -> All {
        events::table
            .select(Event::as_select())
            .filter(events::is_deleted.eq(false))
    }

    pub fn joined() -> Join {
        Self::all()
            .inner_join(accounts::table)
            .inner_join(places::table)
    }

    pub fn all_summary_with_participation() -> JoinWithParticipation<EventSummary> {
        Self::joined().select((
            events::id
                .count_references_in(participation::event_id)
                .bracket(),
            EventSummary::as_select(),
        ))
    }

    pub fn find(id: i32) -> Find {
        Self::all().find(id)
    }

    pub fn find_joined(id: i32) -> FindJoin {
        Self::find(id)
            .inner_join(accounts::table)
            .inner_join(places::table)
    }

    pub fn find_as_display_with_participation(id: i32) -> FindWithParticipationAs<EventDisplay> {
        Self::find_joined(id).select((
            events::id
                .count_references_in(participation::event_id)
                .bracket(),
            EventDisplay::as_select(),
        ))
    }

    pub fn find_as_summary_with_participation(id: i32) -> FindWithParticipationAs<EventSummary> {
        Self::find_joined(id).select((
            events::id
                .count_references_in(participation::event_id)
                .bracket(),
            EventSummary::as_select(),
        ))
    }

    pub fn update(id: i32) -> UpdateId {
        diesel::update(events::table).filter(events::id.eq(id))
    }
}

impl<'a> NewEvent<'a> {
    pub fn as_insert(&self) -> InsertStatement<Table, <&Self as Insertable<Table>>::Values> {
        diesel::insert_into(events::table).values(self)
    }
}
