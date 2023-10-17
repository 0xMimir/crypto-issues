use std::sync::Arc;

use actix_web::web::{self, resource, Data, ServiceConfig};
use sea_orm::DatabaseConnection;

use self::{domain::Repository, handlers::*, repository::PgRepository};

mod contract;
mod data;
pub(super) mod docs;
mod domain;
mod handlers;
mod repository;

pub fn setup(conn: Arc<DatabaseConnection>, config: &mut ServiceConfig) {
    let state = Repository {
        repository: PgRepository::new(conn),
    };

    config.app_data(Data::new(state));

    config.service(
        resource("/api/{version}/repository/search")
            .route(web::get().to(get_search::<Repository<PgRepository>>)),
    );

    config.service(
        resource("/api/{version}/repository/{id}")
            .route(web::get().to(get_repository_by_id::<Repository<PgRepository>>)),
    );

    config.service(
        resource("/api/{version}/repository/{id}/issues")
            .route(web::get().to(get_issues::<Repository<PgRepository>>)),
    );
}
