use entity::climbing_grades::Model;
use entity::climbing_grades;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::errors::errors::RepositoryError;
use crate::structs::climbing_grade::ClimbingGrade;

pub struct ClimbingGradeRepo{
    db: DatabaseConnection
}

impl ClimbingGradeRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self)-> Result<Option<Vec<ClimbingGrade>>, RepositoryError>{
        let grades_model: Vec<Model> = climbing_grades::Entity::find().all(&self.db).await?;
        let grades: Vec<ClimbingGrade> = grades_model.into_iter().map(ClimbingGrade::from).collect();
        Ok(Some(grades))
    }
}
