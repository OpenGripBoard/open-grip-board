#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DbError(#[from] sea_orm::DbErr),

    #[error("Entity not found")]
    NotFound,

    #[error("Hashing error: {0}")]
    HashingError(String),
}

impl From<argon2::password_hash::Error> for RepositoryError {
    fn from(err: argon2::password_hash::Error) -> Self {
        RepositoryError::HashingError(err.to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthentificationError {
    #[error("Password not valid")]
    AuthError,

    #[error("Email not found")]
    NotFound(#[from] RepositoryError),

    #[error("Internal processing failed")]
    InternalError,
}
