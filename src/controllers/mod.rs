use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::env;

mod account;
mod event;
mod metadata;

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
            .service(web::scope("/api").configure(configure))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/account").configure(account::configure))
        .service(web::scope("/metadata").configure(metadata::configure))
        .service(web::scope("/event").configure(event::configure));
}
