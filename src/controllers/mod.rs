use actix_web::middleware::{Logger, NormalizePath};
use actix_web::web::{self, ServiceConfig};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;

mod account;
mod auth;
mod event;
mod metadata;

use crate::error::Error;

pub struct AppState {
    pool: Pool<AsyncPgConnection>,
}

#[derive(Default)]
pub struct AppBuilder {
    pool: Option<Pool<AsyncPgConnection>>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pool(mut self, pool: Pool<AsyncPgConnection>) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn into_configurator(self) -> impl FnOnce(&mut ServiceConfig) {
        let pool = self.pool.expect("pool must be set");

        move |cfg| {
            let json_cfg = web::JsonConfig::default()
                .limit(1024 * 32)
                .content_type_required(true)
                .content_type(|mime| mime == mime::APPLICATION_JSON)
                .error_handler(|err, _req| Error::BadRequest(err.to_string()).into());
            let path_cfg = web::PathConfig::default()
                .error_handler(|err, _req| Error::BadRequest(err.to_string()).into());
            let query_cfg = web::QueryConfig::default()
                .error_handler(|err, _req| Error::BadRequest(err.to_string()).into());

            cfg.app_data(web::Data::new(AppState { pool: pool.clone() }))
                .app_data(json_cfg)
                .app_data(path_cfg)
                .app_data(query_cfg)
                .service(
                    web::scope("/api")
                        .wrap(NormalizePath::trim())
                        .wrap(Logger::new(
                            r#"%t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
                        ))
                        .configure(configure),
                );
        }
    }
}

fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(auth::configure))
        .service(web::scope("/account").configure(account::configure))
        .service(web::scope("/metadata").configure(metadata::configure))
        .service(web::scope("/event").configure(event::configure));
}
