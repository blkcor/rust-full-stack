use actix_web::{get, web};

pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[get("/")]
pub async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
pub async fn version(app_state: web::Data<AppState>) -> String {
    tracing::info!("Checking database version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(&app_state.pool)
        .await;
    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}
