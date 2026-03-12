use thiserror::Error;
#[cfg(feature = "server")]
pub mod database;
#[cfg(feature = "server")]
pub mod webscraper;


#[derive(Debug, Error)]
pub enum DbJsonError {
    #[cfg(feature = "server")]
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}