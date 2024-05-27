use actix_web::{App, HttpServer};
use clap::Parser;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use backend::utils::auth::CRAProvider;
use backend::AppBuilder;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    /// The port to listen on
    #[clap(short, long, env, default_value = "8080")]
    port: u16,

    /// The address to bind to
    #[clap(long, env, default_value = "127.0.0.1")]
    host: String,

    /// The URL of the database to connect to
    #[clap(long, env)]
    database_url: String,

    /// The URL of the database to connect to
    #[clap(
        long,
        env,
        default_value = "https://sso.cra.ac.cn/realms/cra-service-realm/protocol/cas/serviceValidate"
    )]
    cas_endpoint: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    let opts = Opts::parse();

    let pool_config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&opts.database_url);
    let pool = Pool::builder(pool_config).build().unwrap();
    let configurator = AppBuilder::new()
        .with_pool(pool.clone())
        .with_auth_provider(Box::new(CRAProvider::new()))
        .with_cas_endpoint(&opts.cas_endpoint)
        .into_configurator();

    log::info!("Starting server on {}:{}", opts.host, opts.port);
    HttpServer::new(move || App::new().configure(configurator.clone().as_function()))
        .bind((opts.host, opts.port))?
        .workers(2)
        .run()
        .await
}
