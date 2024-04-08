use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use phf::phf_map;

pub mod account;
pub mod event;
pub mod misc;
pub mod schema;
pub mod utils;

pub type Conn = deadpool::managed::Object<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub static CONSTRAINTS_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "accounts_name_key" => "name",
    "accounts_sustech_id_key" => "sustech_id",

    "events_venue_id_fkey" => "venue_id",
};
