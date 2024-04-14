pub const API_VERSION: &str = "v0.1.0";

use actix_web::{
    web::{get, ServiceConfig},
    HttpResponse,
};

use crate::film_repository::FilmRepository;
pub struct AppState<R: FilmRepository> {
    pub film_repository: R,
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(health));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn health_check_work() {
        let result = health().await;
        assert!(result.status().is_success());
        assert_eq!(result.status(), StatusCode::OK);

        let data = result
            .headers()
            .get("version")
            .and_then(|h| h.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }
}
