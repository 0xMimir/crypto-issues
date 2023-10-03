use std::sync::Arc;

use actix_web::{dev::Server, web::ServiceConfig, App, HttpServer};
use sea_orm::DatabaseConnection;

use self::kenobi::hello_there;

mod cryptocurrencies;
mod kenobi;
mod openapi;

pub fn create_api(conn: Arc<DatabaseConnection>) -> Server {
    let workers = match config::get_default("IS_DEV", "false").as_str() == "true" {
        true => 1,
        false => config::get_default("ACTIX_WORKERS", "8")
            .parse()
            .unwrap_or(8),
    };

    HttpServer::new(move || App::new().configure(|config| configure_routes(conn.clone(), config)))
        .workers(workers)
        .bind(("localhost", 1111))
        .expect("Unable to start sever")
        .run()
}

fn configure_routes(conn: Arc<DatabaseConnection>, config: &mut ServiceConfig) {
    openapi::OpenApiDocsFactory::configure_routes(config);
    cryptocurrencies::setup(conn, config);

    config.service(hello_there);
}
