use actix_web::web::{self, ServiceConfig};
use api_lib::health::{service, AppState};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;
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
        cfg.app_data(app_state).configure(service);
    };
    Ok(config.into())
}
