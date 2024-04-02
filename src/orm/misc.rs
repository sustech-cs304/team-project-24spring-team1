use diesel::dsl::{AsSelect, Eq, Filter, Find, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use serde::Serialize;

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
}
