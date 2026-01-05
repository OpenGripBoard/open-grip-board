use sea_orm::DatabaseConnection;

use crate::{errors::errors::RepositoryError, repositories::{gym_repo::GymRepo, crud_repo::CrudRepo}, structs::gym::Gym};

pub async fn get_gym(gym_id: i32) -> Result<Gym, RepositoryError>{
    let db : DatabaseConnection = sea_orm::Database::connect("postgres://user:pass@localhost/db").await?;
    let gym_repo: GymRepo = GymRepo::new(db);
    let gym_option = gym_repo.find_by_id(gym_id).await?;
    gym_option.ok_or(RepositoryError::NotFound)
}