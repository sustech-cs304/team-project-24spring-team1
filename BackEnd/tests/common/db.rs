use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub fn get_manager() -> AsyncDieselConnectionManager<AsyncPgConnection> {
    let url = std::env::var("TEST_DATABASE_URL").unwrap();
    AsyncDieselConnectionManager::<AsyncPgConnection>::new(url)
}
