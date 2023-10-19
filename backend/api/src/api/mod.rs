use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{dev::Server, web::ServiceConfig, App, HttpServer};
use sea_orm::DatabaseConnection;

use self::kenobi::hello_there;

mod cryptocurrencies;
mod kenobi;
mod openapi;
mod repository;
mod statistics;
mod projects;

pub fn create_api(conn: Arc<DatabaseConnection>) -> Server {
    let workers = match config::get_default("IS_DEV", "false").as_str() == "true" {
        true => 1,
        false => config::get_default("ACTIX_WORKERS", "8")
            .parse()
            .unwrap_or(8),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(setup_cors())
            .configure(|config| configure_routes(conn.clone(), config))
    })
    .workers(workers)
    .bind(("0.0.0.0", 1111))
    .expect("Unable to start sever")
    .run()
}

fn configure_routes(conn: Arc<DatabaseConnection>, config: &mut ServiceConfig) {
    openapi::OpenApiDocsFactory::configure_routes(config);
    cryptocurrencies::setup(conn.clone(), config);
    repository::setup(conn.clone(), config);
    statistics::setup(conn, config);

    config.service(hello_there);
}

fn setup_cors() -> Cors {
    Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT"])
        .allowed_origin("http://localhost:8080")
        .allow_any_origin()
        .allow_any_header()
}
