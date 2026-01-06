use entity::climbers;
use rocket::async_trait;
use sea_orm::{DatabaseConnection, DeleteResult, EntityTrait};
use crate::repositories::crud_repo::CrudRepo;
use crate::structs::climber::Climber;
use crate::errors::errors::RepositoryError;

pub struct ClimberRepo {
    db: DatabaseConnection
}

impl ClimberRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CrudRepo<Climber, i32> for ClimberRepo{
    async fn find_by_id(&self, id: i32)-> Result<Option<Climber>, RepositoryError>{
        let climber_model = climbers::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;
        Ok(Some(Climber::from(climber_model)
        ))
    }

    async fn delete_by_id(&self, id: i32) -> Result<(), RepositoryError>{
        let res: DeleteResult = climbers::Entity::delete_by_id(id).exec(&self.db).await?;
        if res.rows_affected == 1 {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }
}
