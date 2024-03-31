use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub mod account;
pub mod schema;

pub type Conn = deadpool::managed::Object<AsyncDieselConnectionManager<AsyncPgConnection>>;
