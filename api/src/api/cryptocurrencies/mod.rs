use std::sync::Arc;

use actix_web::web::{self, resource, Data, ServiceConfig};
use sea_orm::DatabaseConnection;

use self::{domain::Cryptocurrencies, handlers::*, repository::PgRepository};

mod contract;
mod domain;
mod handlers;
mod repository;

pub fn setup(conn: Arc<DatabaseConnection>, config: &mut ServiceConfig) {
    let state = Cryptocurrencies {
        repository: PgRepository::new(conn),
    };

    config.app_data(Data::new(state));

    config.service(
        resource("/api/{version}/crypto")
            .route(web::get().to(get_cryptocurrencies::<Cryptocurrencies<PgRepository>>)),
    );

    config.service(
        resource("/api/{version}/crypto/{id}")
            .route(web::get().to(get_cryptocurrency_by_id::<Cryptocurrencies<PgRepository>>)),
    );

    config.service(
        resource("/api/{version}/repository/{id}/issues")
            .route(web::get().to(get_issues::<Cryptocurrencies<PgRepository>>)),
    );
}
