use actix_web::{
    web::{self, resource, ServiceConfig},
    HttpResponse,
};
use utoipa::{openapi::OpenApi, OpenApi as Trait};

pub struct OpenApiDocsFactory;

impl OpenApiDocsFactory {
    fn generate() -> ApiDocsState {
        let mut openapi = OpenApi::default();

        let docs = [super::cryptocurrencies::docs::CryptocurrenciesDocs::openapi()];

        for doc in docs {
            openapi.merge(doc);
        }

        ApiDocsState { openapi }
    }

    pub fn configure_routes(config: &mut ServiceConfig) {
        config.app_data(web::Data::new(Self::generate()));
        config.service(resource("/openapi.json").route(web::get().to(handle_docs)));
    }
}

struct ApiDocsState {
    openapi: OpenApi,
}

async fn handle_docs(state: web::Data<ApiDocsState>) -> HttpResponse {
    HttpResponse::Ok().json(&state.openapi)
}
