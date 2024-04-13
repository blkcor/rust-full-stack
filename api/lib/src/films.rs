use actix_web::{
    delete, get, post, put,
    web::{scope, Path, ServiceConfig},
    HttpResponse, Responder,
};

//get film list
#[get("/films")]
pub async fn get_films() -> impl Responder {
    HttpResponse::Ok().body("get films")
}

//get film by id
#[get("/films/{id}")]
pub async fn get_film(id: Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("get film by id: {}", id))
}

//create film
#[post("/films")]
pub async fn create_film() -> impl Responder {
    HttpResponse::Ok().body("create film")
}

//update film
#[put("/films")]
pub async fn update_film() -> impl Responder {
    HttpResponse::Ok().body("update film")
}

//delete film
#[delete("/films/{id}")]
pub async fn delete_film(id: Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("delete film by id: {}", id))
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
