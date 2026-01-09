use sea_orm::DatabaseConnection;

use crate::{errors::errors::RepositoryError, repositories::{gym_repo::GymRepo, crud_repo::CrudRepo}, structs::gym::Gym};

pub struct GymService{
    repo: GymRepo,
}

impl GymService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { 
            repo: GymRepo::new(db),
        }
    }

    pub async fn get_gym(&self, gym_id: i32) -> Result<Gym, RepositoryError>{
        let gym_option = self.repo.find_by_id(gym_id).await?;
        gym_option.ok_or(RepositoryError::NotFound)
    }
}
