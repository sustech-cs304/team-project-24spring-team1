use actix_web::middleware::{Logger, NormalizePath};
use actix_web::web::{self, ServiceConfig};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;
use std::sync::Mutex;

mod account;
mod auth;
mod comment;
mod event;
mod metadata;
mod moment;

use crate::error::Error;
use crate::utils::auth::AuthProvider;

pub struct AppState {
    pool: Pool<AsyncPgConnection>,
    auth_provider: Mutex<Box<dyn AuthProvider + Send>>,
}

#[derive(Default)]
pub struct AppBuilder {
    pool: Option<Pool<AsyncPgConnection>>,
    auth_provider: Option<Mutex<Box<dyn AuthProvider + Send>>>,
}

#[derive(Clone)]
pub struct AppConfigurator {
    data: web::Data<AppState>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pool(mut self, pool: Pool<AsyncPgConnection>) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn with_auth_provider(mut self, auth_provider: Box<dyn AuthProvider + Send>) -> Self {
        self.auth_provider = Some(Mutex::new(auth_provider));
        self
    }

    pub fn into_configurator(self) -> AppConfigurator {
        let data = web::Data::new(AppState {
            pool: self.pool.expect("pool must be set"),
            auth_provider: self.auth_provider.expect("auth_provider must be set"),
        });

        AppConfigurator { data }
    }
}

impl AppConfigurator {
    pub fn as_function(&self) -> impl FnOnce(&mut ServiceConfig) + '_ {
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

            cfg.app_data(self.data.clone())
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
        .service(
            web::scope("/event")
                .configure(event::configure)
                .configure(comment::configure),
        )
        .service(web::scope("/moment").configure(moment::configure));
}
