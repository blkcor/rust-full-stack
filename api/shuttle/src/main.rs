use actix_web::{
    get,
    web::{self, ServiceConfig},
};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

struct AppState {
    pool: sqlx::PgPool,
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(app_state: web::Data<AppState>) -> String {
    tracing::info!("Checking database version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(&app_state.pool)
        .await;
    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Create the database schema
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    //initialize the app state
    let app_state = web::Data::new(AppState { pool });
    // Configure the Actix Web service
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(app_state)
            .service(hello_world)
            .service(version);
    };
    Ok(config.into())
}
