use std::sync::Arc;

use actix_web::web::{self, resource, Data, ServiceConfig};
use sea_orm::DatabaseConnection;

use self::{domain::Statistics, handlers::*, infrastructure::PgRepository};

mod contract;
pub mod docs;
mod domain;
mod handlers;
mod infrastructure;

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
