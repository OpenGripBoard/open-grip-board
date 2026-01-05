#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DbError(#[from] sea_orm::DbErr),

    #[error("Entity not found")]
    NotFound,
}
