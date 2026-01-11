use entity::{climbers, favourite_gyms};
use rocket::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, Set, ColumnTrait, QueryFilter};
use crate::commands::create_climber::CreateClimber;
use crate::repositories::crud_repo::CrudRepo;
use crate::structs::climber::Climber;
use crate::errors::errors::{AuthentificationError, RepositoryError};
use crate::utilities::hash_util;

pub struct ClimberRepo {
    db: DatabaseConnection
}

impl ClimberRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl ClimberRepo{
    pub async fn authenticate_climber_password(&self, email: String, password: String) -> Result<Option<Climber>, AuthentificationError>{
        let password_hash: String = match self.get_password_hash(email.clone()).await?{
            Some(hash) => {hash},
            None => {return Ok(None)}
        };

        let is_verified = match hash_util::veryfiy_password(password_hash, &password){
            Ok(result) => {result},
            Err(_e)=> {return Err(AuthentificationError::InternalError)}
        };

        if is_verified{
            let climber = self.find_by_email(email).await?;
            return Ok(Some(climber))
        }else{
            return Ok(None)
        }
    }

    async fn get_password_hash(&self, email: String) -> Result<Option<String>, RepositoryError>{
        let climber_model = climbers::Entity::find()
            .filter(climbers::Column::Email.eq(email))
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;
        return Ok(climber_model.password_hash);
    }

    async fn find_by_email(&self, email: String) -> Result<Climber,RepositoryError>{
        let climber_model = climbers::Entity::find()
            .filter(climbers::Column::Email.eq(email))
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;
        Ok(Climber::from(climber_model)
        )
    }

    pub async fn insert_favourite_gyms_relation(&self, climber_id: i32, gym_id: i32)-> Result<(),RepositoryError>{
        let favourite_gyms = favourite_gyms::ActiveModel { climber_id: Set(climber_id), gym_id: Set(gym_id) };
        favourite_gyms.insert(&self.db).await?;
        Ok(())
    }

    pub async fn delete_favourite_gyms_relation(&self, climber_id: i32, gym_id: i32)-> Result<(),RepositoryError>{
        let favourite_gyms = favourite_gyms::ActiveModel { climber_id: Set(climber_id), gym_id: Set(gym_id) };
        favourite_gyms.delete(&self.db).await?;
        Ok(())
    }
}

#[async_trait]
impl CrudRepo<Climber, CreateClimber, i32> for ClimberRepo{
    async fn find_by_id(&self, id: i32)-> Result<Climber, RepositoryError>{
        let climber_model = climbers::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;
        Ok(Climber::from(climber_model))
    }

    async fn delete_by_id(&self, id: i32) -> Result<(), RepositoryError>{
        let res: DeleteResult = climbers::Entity::delete_by_id(id).exec(&self.db).await?;
        if res.rows_affected == 1 {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    async fn insert(&self, new_climber: CreateClimber) -> Result<Climber, RepositoryError>{
        let password_hash = hash_util::hash_password(&new_climber.password)?;
        let climber = climbers::ActiveModel {
            username: Set(new_climber.username),
            email: Set(new_climber.email),
            password_hash: Set(Some(password_hash)),
            profile_pic_id: Set(1),
            ..Default::default()
        };
        let climber_model: climbers::Model = climber.insert(&self.db).await?;
        Ok(Climber::from(climber_model))
    }
}
