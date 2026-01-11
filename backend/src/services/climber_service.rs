use sea_orm::DatabaseConnection;

use crate::{commands::create_climber::CreateClimber, dto::request::login_request_dto::LoginRequestDto, errors::errors::{AuthentificationError, RepositoryError}, repositories::{climber_repo::ClimberRepo, crud_repo::CrudRepo}, structs::climber::Climber};

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

    pub async fn create_climber(&self, new_climber: CreateClimber) -> Result<(),RepositoryError>{
        self.repo.insert(new_climber).await?;
        Ok(())
    }

    pub async fn authenticate_climber(&self, login_request_dto: LoginRequestDto) -> Result<Climber, AuthentificationError>{
        match self.repo.authenticate_climber_password(login_request_dto.email, login_request_dto.password).await?{
            Some(climber) => {Ok(climber)}
            None => {Err(AuthentificationError::AuthError)}
        }
    }
}