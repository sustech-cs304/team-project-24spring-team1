use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::SimpleAsyncConnection;
use diesel_async_migrations::{embed_migrations, EmbeddedMigrations};
use std::sync::Once;
use tokio::sync::OnceCell;

#[allow(clippy::declare_interior_mutable_const)]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

static INIT: Once = Once::new();
static INIT_ASYNC: OnceCell<()> = OnceCell::const_new();

pub async fn setup() {
    INIT.call_once(|| {
        if let Err(e) = dotenvy::dotenv() {
            eprintln!("Warning: failed to load .env file: {e}");
        }

        pretty_env_logger::init();

        log::info!("Test environment initialized.");
    });

    INIT_ASYNC
        .get_or_init(|| async {
            let config = crate::common::db::get_manager();
            let pool = Pool::builder(config).build().unwrap();
            let mut conn = pool.get().await.unwrap();

            log::debug!("Recreating testing schema...");
            conn.batch_execute(
                r#"
                    DROP SCHEMA public CASCADE;
                    CREATE SCHEMA public;
                "#,
            )
            .await
            .unwrap();

            log::debug!("Running migrations...");
            #[allow(clippy::borrow_interior_mutable_const)]
            MIGRATIONS.run_pending_migrations(&mut conn).await.unwrap();

            log::debug!("Inserting default places...");
            conn.batch_execute(
                "INSERT INTO places(name) VALUES ('Place 1'), ('Place 2'), ('Place 3');",
            )
            .await
            .unwrap();

            log::info!("Database for testing initialized");
        })
        .await;
}
