use sea_orm::DatabaseConnection;

use crate::{
    commands::create_climber::CreateClimber,
    dto::request::{
        login_request_dto::LoginRequestDto, patch_climber_dto::PatchClimberFavouriteGymDto,
        patch_operation::PatchOperation,
    },
    errors::{AuthentificationError, RepositoryError},
    repositories::{climber_repo::ClimberRepo, crud_repo::CrudRepo},
    structs::climber::Climber,
};

use crate::{services::gym_service::GymService, structs::gym::Gym};

pub struct ClimberService {
    repo: ClimberRepo,
}

impl ClimberService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repo: ClimberRepo::new(db),
        }
    }

    pub async fn get_climber(&self, climber_id: i32) -> Result<Climber, RepositoryError> {
        self.repo.find_by_id(climber_id).await
    }

    pub async fn create_climber(&self, new_climber: CreateClimber) -> Result<(), RepositoryError> {
        self.repo.insert(new_climber).await?;
        Ok(())
    }

    pub async fn authenticate_climber(
        &self,
        login_request_dto: LoginRequestDto,
    ) -> Result<Climber, AuthentificationError> {
        match self
            .repo
            .authenticate_climber_password(login_request_dto.email, login_request_dto.password)
            .await?
        {
            Some(climber) => Ok(climber),
            None => Err(AuthentificationError::AuthError),
        }
    }

    pub async fn patch_climber_favourite_gyms(
        &self,
        climber_id: i32,
        patch_climber_dto: PatchClimberFavouriteGymDto,
        gym_service: &GymService,
    ) -> Result<(), RepositoryError> {
        let mut climber: Climber = self.repo.find_by_id(climber_id).await?;
        let operation: PatchOperation = patch_climber_dto.patch_operation;
        let gym: Gym = gym_service
            .get_gym(patch_climber_dto.favourite_gym_id)
            .await?;
        match operation {
            PatchOperation::Add => {
                let gym_id = gym.id;
                climber = self.add_favourite_gym(climber, gym);
                self.repo
                    .insert_favourite_gyms_relation(climber.id, gym_id)
                    .await?;
            }
            PatchOperation::Remove => {
                let gym_id = gym.id;
                climber = self.remove_favourite_gym(climber, gym);
                self.repo
                    .delete_favourite_gyms_relation(climber.id, gym_id)
                    .await?;
            }
        };
        Ok(())
    }

    fn add_favourite_gym(&self, climber: Climber, gym: Gym) -> Climber {
        let new_gyms = match climber.favourite_gyms {
            Some(mut gyms_vec) => {
                gyms_vec.push(gym);
                Some(gyms_vec)
            }
            None => Some(vec![gym]),
        };
        Climber {
            favourite_gyms: new_gyms,
            ..climber
        }
    }

    fn remove_favourite_gym(&self, climber: Climber, gym: Gym) -> Climber {
        let new_gyms = match climber.favourite_gyms {
            Some(gyms_vec) => {
                let mut gyms_vec = gyms_vec;
                gyms_vec.retain(|g| g.id != gym.id);
                if gyms_vec.is_empty() {
                    None
                } else {
                    Some(gyms_vec)
                }
            }
            None => None,
        };
        Climber {
            favourite_gyms: new_gyms,
            ..climber
        }
    }
}
