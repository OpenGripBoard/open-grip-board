use sea_orm::DatabaseConnection;

use crate::{errors::errors::RepositoryError, repositories::{climber_repo::ClimberRepo, crud_repo::CrudRepo}, structs::climber::Climber};

pub async fn get_climber(climber_id: i32) -> Result<Climber, RepositoryError>{
    let db : DatabaseConnection = sea_orm::Database::connect("postgres://user:pass@localhost/db").await?;
    let climber_repo: ClimberRepo = ClimberRepo::new(db);
    let climber_option = climber_repo.find_by_id(climber_id).await?;
    climber_option.ok_or(RepositoryError::NotFound)
}