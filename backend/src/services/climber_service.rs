use crate::{commands::create_climber::CreateClimber, errors::errors::RepositoryError, repositories::climber_repo, structs::climber::Climber};

pub async fn create_climber(cmd: CreateClimber) -> Result<Climber,RepositoryError>{
    climber_repo::create_climber(cmd).await
}

pub async fn get_climber(climber_id: i32) -> Result<Climber, RepositoryError>{
    climber_repo::get_climber(climber_id).await
}