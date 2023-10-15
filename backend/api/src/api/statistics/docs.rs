use super::handlers::*;
use store::objects::LanguageCount;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_language_counts), components(schemas(LanguageCount)))]
pub struct StatisticsDocs;
