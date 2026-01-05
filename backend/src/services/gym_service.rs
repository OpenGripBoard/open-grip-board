use crate::{errors::errors::RepositoryError, repositories::gym_repo, structs::gym::Gym};

pub async fn get_gym(gym_id: i32) -> Result<Gym, RepositoryError>{
    gym_repo::get_gym(gym_id).await
}