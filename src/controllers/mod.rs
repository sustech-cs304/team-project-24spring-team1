use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::env;

mod account;

pub struct AppState {
    pool: Pool<AsyncPgConnection>,
}

pub async fn run() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set, e.g. DATABASE_URL=postgres://<user>:<passwd>@127.0.0.1:5432/<db>");
    let pool_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(pool_config).build().unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .wrap(Logger::default())
            .service(web::scope("/api").configure(config))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/account").configure(account::config));
}
