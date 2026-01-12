use sea_orm::DatabaseConnection;

use crate::{
    commands::create_gym::CreateGym,
    errors::errors::RepositoryError,
    repositories::{crud_repo::CrudRepo, gym_repo::GymRepo},
    structs::gym::Gym,
};

pub struct GymService {
    repo: GymRepo,
}

impl GymService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repo: GymRepo::new(db),
        }
    }

    pub async fn get_gym(&self, gym_id: i32) -> Result<Gym, RepositoryError> {
        Ok(self.repo.find_by_id(gym_id).await?)
    }

    pub async fn get_all_gyms(&self) -> Result<Vec<Gym>, RepositoryError> {
        let gyms_option = self.repo.find_all().await?;
        gyms_option.ok_or(RepositoryError::NotFound)
    }

    pub async fn create_gym(&self, new_gym: CreateGym) -> Result<Gym, RepositoryError> {
        self.repo.insert(new_gym).await
    }
}
