use actix_web::{web::Data, HttpResponse};
use error::Result;

use super::super::contract::StatisticsContract;

#[utoipa::path(
    get,
    path = "/api/v1/statistics/languages-count",
    responses(
        (
            status = 200,
            description = "Crypto currency data with all full data for repositories",
            body = [LanguageCount]
        )
    )
)]
pub async fn get_language_counts<S: StatisticsContract>(service: Data<S>) -> Result<HttpResponse> {
    let languages = service.get_language_counts().await?;
    Ok(HttpResponse::Ok().json(languages))
}
