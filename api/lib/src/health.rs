use actix_web::{get, HttpResponse, Responder};

pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}
