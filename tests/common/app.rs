use actix_http::Request;
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, App, Error};
use deadpool::managed::{Hook, HookResult, Metrics, Pool};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use std::sync::{Arc, Mutex};

use backend::utils::auth::CRAProvider;
use backend::AppBuilder;

use crate::common::setup;

async fn connection_post_create<E>(
    object: &mut AsyncPgConnection,
    _metrics: &Metrics,
) -> HookResult<E> {
    object.begin_test_transaction().await.unwrap();
    Ok(())
}

pub trait TestApp = Service<Request, Response = ServiceResponse<BoxBody>, Error = Error>;

pub async fn create_app() -> impl TestApp {
    setup().await;

    let connection_count = Arc::new(Mutex::new(0));

    let config = crate::common::db::get_manager();
    let pool = Pool::builder(config)
        .post_create(Hook::async_fn(move |object, metrics| {
            {
                let mut count = connection_count.lock().unwrap();
                assert_eq!(*count, 0, "connection_post_create called multiple times");
                *count += 1;
            }
            Box::pin(connection_post_create(object, metrics))
        }))
        .max_size(1)
        .build()
        .unwrap();

    let configurator = AppBuilder::new()
        .with_pool(pool.clone())
        .with_auth_provider(Box::new(CRAProvider::new()))
        .with_cas_endpoint("")
        .into_configurator();

    test::init_service(App::new().configure(configurator.as_function())).await
}
