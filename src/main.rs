#![feature(trait_alias)]
#![feature(int_roundings)]
#![feature(let_chains)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod controllers;
mod error;
mod orm;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    controllers::run().await
}
