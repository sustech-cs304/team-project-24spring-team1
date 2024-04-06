use actix_web::middleware::{Logger, NormalizePath};
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

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_port = env::var("BIND_PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8080);
    info!("Starting server on {}:{}", bind_address, bind_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .wrap(NormalizePath::trim())
            .wrap(Logger::new(
                r#"%t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .service(web::scope("/api").configure(configure))
    })
    .bind((bind_address, bind_port))?
    .workers(2)
    .run()
    .await
}

fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/account").configure(account::configure))
        .service(web::scope("/metadata").configure(metadata::configure))
        .service(web::scope("/event").configure(event::configure));
}
