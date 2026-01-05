use entity::{climbers, gyms};
use rocket::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::repositories::crud_repo::CrudRepo;
use crate::structs::climber::Climber;
use crate::structs::gym::Gym;
use crate::structs::location::Location;
use crate::errors::errors::RepositoryError;

pub struct GymRepo {
    db: DatabaseConnection
}

impl GymRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CrudRepo<Gym, i32> for GymRepo{
    async fn find_by_id(&self, id: i32)-> Result<Option<Gym>, RepositoryError>{
        let gym_model = gyms::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;
        let admin_model = climbers::Entity::find_by_id(gym_model.admin_id)
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;
        Ok(Some(Gym{
            id: gym_model.gym_id,
            name: gym_model.name,
            location: Location{ longitude: gym_model.location_x, latitude: gym_model.location_y },
            admin: Climber::from(admin_model),
        }))
    }
}
