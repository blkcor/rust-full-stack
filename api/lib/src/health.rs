use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};

pub struct AppState {
    pub pool: sqlx::PgPool,
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(health));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "v0.0.1"))
        .finish()
}
