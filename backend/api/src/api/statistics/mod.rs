use std::sync::Arc;

use actix_web::web::{ServiceConfig, self, resource, Data};
use sea_orm::DatabaseConnection;

use self::{domain::Statistics, infrastructure::PgRepository, handlers::*};

mod handlers;
mod infrastructure;
mod contract;
pub mod docs;
mod domain;

pub fn setup(conn: Arc<DatabaseConnection>, config: &mut ServiceConfig) {
    let state = Statistics {
        repository: PgRepository::new(conn),
    };

    config.app_data(Data::new(state));

    config.service(
        resource("/api/{version}/statistics/languages-count")
            .route(web::get().to(get_language_counts::<Statistics<PgRepository>>)),
    );
}