use diesel::dsl::{AsSelect, Eq, Filter, Find, InnerJoin, InnerJoinOn, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::{DeleteStatement, InsertStatement, IntoUpdateTarget};
use serde::Serialize;

use super::event::EventSummary;
use super::schema::*;

#[derive(Debug, Serialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = places)]
pub struct Place {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Selectable, Identifiable, Insertable, Queryable)]
#[diesel(table_name = participation)]
#[diesel(primary_key(event_id, account_id))]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Account))]
pub struct Participation {
    pub event_id: i32,
    pub account_id: i32,
}

type PlaceAll = Select<places::table, AsSelect<Place, Pg>>;
type PlaceWithName<'a> = Eq<places::name, &'a str>;
type PlaceByName<'a> = Filter<PlaceAll, PlaceWithName<'a>>;
type PlaceIdByName<'a> = Select<PlaceByName<'a>, places::id>;
type PlaceFind = Find<PlaceAll, i32>;
pub type PlaceFindName = Select<PlaceFind, places::name>;

type ParticipationFind = Find<participation::table, (i32, i32)>;
type ParticipationByAccountId = Select<
    Filter<
        InnerJoinOn<
            InnerJoinOn<
                InnerJoin<participation::table, events::table>,
                places::table,
                Eq<places::id, events::venue_id>,
            >,
            accounts::table,
            Eq<accounts::id, events::organizer_id>,
        >,
        Eq<participation::account_id, i32>,
    >,
    AsSelect<EventSummary, Pg>,
>;

impl Place {
    pub fn all() -> PlaceAll {
        places::table.select(Place::as_select())
    }

    pub fn by_name(name: &str) -> PlaceIdByName<'_> {
        Self::all().filter(places::name.eq(name)).select(places::id)
    }

    pub fn find(id: i32) -> PlaceFindName {
        Self::all().find(id).select(places::name)
    }
}

impl Participation {
    pub fn new(event_id: i32, account_id: i32) -> Self {
        Self {
            event_id,
            account_id,
        }
    }

    pub fn as_insert(
        &self,
    ) -> InsertStatement<participation::table, <&Self as Insertable<participation::table>>::Values>
    {
        diesel::insert_into(participation::table).values(self)
    }

    pub fn as_delete(
        &self,
    ) -> DeleteStatement<participation::table, <ParticipationFind as IntoUpdateTarget>::WhereClause>
    {
        diesel::delete(participation::table.find((self.event_id, self.account_id)))
    }

    pub fn by_account_id(account_id: i32) -> ParticipationByAccountId {
        participation::table
            .inner_join(events::table)
            .inner_join(places::table.on(places::id.eq(events::venue_id)))
            .inner_join(accounts::table.on(accounts::id.eq(events::organizer_id)))
            .filter(participation::account_id.eq(account_id))
            .select(EventSummary::as_select())
    }
}
