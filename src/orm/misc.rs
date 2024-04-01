use diesel::dsl::{AsSelect, Eq, Filter, Find, Select};
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::Serialize;

use super::schema::*;

#[derive(Serialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = places)]
pub struct Place {
    pub id: i32,
    pub name: String,
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
