use actix_web::{
    delete, get, post, put,
    web::{scope, Data, Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::health::AppState;

//get film list
#[get("/films")]
pub async fn get_films(app_state: Data<AppState>) -> impl Responder {
    match app_state.film_repository.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

//get film by id
#[get("/films/{id}")]
pub async fn get_film(app_state: Data<AppState>, id: Path<Uuid>) -> impl Responder {
    match app_state.film_repository.get_film(&id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

//create film
#[post("/films")]
pub async fn create_film(
    app_state: Data<AppState>,
    create_film: Json<CreateFilm>,
) -> impl Responder {
    match app_state.film_repository.create_film(&create_film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

//update film
#[put("/films")]
pub async fn update_film(app_state: Data<AppState>, film: Json<Film>) -> impl Responder {
    match app_state.film_repository.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

//delete film
#[delete("/films/{id}")]
pub async fn delete_film(app_state: Data<AppState>, id: Path<Uuid>) -> impl Responder {
    match app_state.film_repository.delete_film(&id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/v1")
            .service(get_films)
            .service(get_film)
            .service(create_film)
            .service(update_film)
            .service(delete_film),
    );
}
