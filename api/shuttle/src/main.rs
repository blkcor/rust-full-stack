use actix_web::web::{self, ServiceConfig};
use api_lib::{film_repository::PostgresFilmRepository, health::AppState};
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
    let film_repository = PostgresFilmRepository::new(pool);
    let film_repository = web::Data::new(AppState { film_repository });

    // Configure the Actix Web service
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(film_repository)
            .configure(api_lib::health::service)
            .configure(api_lib::films::service::<PostgresFilmRepository>);
    };
    Ok(config.into())
}
