use crate::commands::create_gym::CreateGym;
use crate::errors::RepositoryError;
use crate::repositories::crud_repo::CrudRepo;
use crate::structs::climber::Climber;
use crate::structs::gym::Gym;
use crate::structs::location::Location;
use entity::{climbers, gyms, hangboards};
use rocket::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, Set};

pub struct GymRepo {
    db: DatabaseConnection,
}

impl GymRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Option<Vec<Gym>>, RepositoryError> {
        let gyms_with_admin: Vec<(gyms::Model, Option<climbers::Model>)> = gyms::Entity::find()
            .find_also_related(climbers::Entity)
            .all(&self.db)
            .await?;

        let gyms_with_hangboards: Vec<(gyms::Model, Vec<hangboards::Model>)> = gyms::Entity::find()
            .find_with_related(hangboards::Entity)
            .all(&self.db)
            .await?;

        let gyms: Vec<Gym> = gyms_with_admin
            .into_iter()
            .map(|(gym_model, admin_model)| {
                let hangboards = gyms_with_hangboards
                    .iter()
                    .find(|(h_gym_model, _)| h_gym_model.gym_id == gym_model.gym_id)
                    .map(|(_, h)| h.clone())
                    .unwrap_or_default();

                Gym::from((gym_model, admin_model, hangboards))
            })
            .collect();

        Ok(Some(gyms))
    }
}

#[async_trait]
impl CrudRepo<Gym, CreateGym, i32> for GymRepo {
    async fn find_by_id(&self, id: i32) -> Result<Gym, RepositoryError> {
        let gym_model = gyms::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;
        let admin_model = climbers::Entity::find_by_id(gym_model.admin_id)
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;
        Ok(Gym {
            id: gym_model.gym_id,
            name: gym_model.name,
            location: Location {
                longitude: gym_model.location_x,
                latitude: gym_model.location_y,
            },
            admin: Climber::from(admin_model),
            hangboards: None,
        })
    }

    async fn delete_by_id(&self, id: i32) -> Result<(), RepositoryError> {
        let res: DeleteResult = gyms::Entity::delete_by_id(id).exec(&self.db).await?;
        if res.rows_affected == 1 {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    async fn insert(&self, new_gym: CreateGym) -> Result<Gym, RepositoryError> {
        let gym = gyms::ActiveModel {
            name: Set(new_gym.name),
            location_x: Set(new_gym.location.longitude),
            location_y: Set(new_gym.location.latitude),
            admin_id: Set(new_gym.admin.id),
            ..Default::default()
        };
        let gym_model: gyms::Model = gym.insert(&self.db).await?;
        Ok(self.find_by_id(gym_model.gym_id).await?)
    }
}
