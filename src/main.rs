#![feature(trait_alias)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod controllers;
mod error;
mod orm;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    dotenvy::dotenv().unwrap();
    pretty_env_logger::init();

    controllers::run().await
}
