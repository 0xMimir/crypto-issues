use std::sync::Arc;

use actix_web::web::{self, Data, ServiceConfig};
use sea_orm::DatabaseConnection;

use self::{domain::Projects, handlers::*, infrastructure::PgRepository};

mod contract;
mod data;
pub(super) mod docs;
mod domain;
mod handlers;
mod infrastructure;

pub fn setup(conn: Arc<DatabaseConnection>, config: &mut ServiceConfig) {
    let state = Projects::new(PgRepository::new(conn));

    config.app_data(Data::new(state));

    config.service(
        web::resource("/api/{version}/projects")
            .route(web::get().to(get_search::<Projects<PgRepository>>)),
    );
}
