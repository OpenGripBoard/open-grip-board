use rocket::async_trait;

use crate::errors::RepositoryError;

#[async_trait]
pub trait CrudRepo<T, U, Id> {
    async fn find_by_id(&self, id: Id) -> Result<T, RepositoryError>;
    async fn insert(&self, entity: U) -> Result<T, RepositoryError>;
    // async fn update(&self, entity: T) -> Result<T,RepositoryError>;
    async fn delete_by_id(&self, id: Id) -> Result<(), RepositoryError>;
}
