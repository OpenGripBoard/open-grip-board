use sea_orm::DatabaseConnection;

use crate::{errors::errors::RepositoryError, repositories::{climber_repo::ClimberRepo, crud_repo::CrudRepo}, structs::climber::Climber};

pub struct ClimberService{
    repo: ClimberRepo,
}

impl ClimberService{
    pub fn new(db: DatabaseConnection) -> Self{
        Self {
            repo: ClimberRepo::new(db),
        }
    }

    pub async fn get_climber(&self, climber_id: i32) -> Result<Climber, RepositoryError>{
        let climber_option = self.repo.find_by_id(climber_id).await?;
        climber_option.ok_or(RepositoryError::NotFound)
    }
}