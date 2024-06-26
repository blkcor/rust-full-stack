use async_trait::async_trait;
use shared::models::{CreateFilm, Film};
use sqlx::types::Uuid;
pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

pub mod postgres_film_repository;
pub use postgres_film_repository::PostgresFilmRepository;

#[async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_films(&self) -> FilmResult<Vec<Film>>;
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    async fn create_film(&self, id: &CreateFilm) -> FilmResult<Film>;
    async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}
