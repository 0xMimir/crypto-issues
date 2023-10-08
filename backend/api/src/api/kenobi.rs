use actix_web::{get, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items", body = String)
    )
)]
#[get("/hello-there")]
///
/// Route to know that api has started up
///
pub async fn hello_there() -> HttpResponse {
    HttpResponse::Ok().body("General Kenobi")
}
