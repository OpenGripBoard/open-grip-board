use crate::commands::create_climbing_grade::CreateClimbingGrade;
use crate::errors::RepositoryError;
use crate::repositories::crud_repo::CrudRepo;
use crate::structs::climbing_grade::ClimbingGrade;
use entity::climbing_grades;
use entity::climbing_grades::Model;
use rocket::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait};

pub struct ClimbingGradeRepo {
    db: DatabaseConnection,
}

impl ClimbingGradeRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Option<Vec<ClimbingGrade>>, RepositoryError> {
        let grades_model: Vec<Model> = climbing_grades::Entity::find().all(&self.db).await?;
        let grades: Vec<ClimbingGrade> =
            grades_model.into_iter().map(ClimbingGrade::from).collect();
        Ok(Some(grades))
    }
}

#[async_trait]
impl CrudRepo<ClimbingGrade, CreateClimbingGrade, i32> for ClimbingGradeRepo {
    async fn find_by_id(&self, id: i32) -> Result<ClimbingGrade, RepositoryError> {
        let grades_model: Model = climbing_grades::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(RepositoryError::NotFound)?;
        Ok(ClimbingGrade::from(grades_model))
    }

    async fn delete_by_id(&self, id: i32) -> Result<(), RepositoryError> {
        let res: DeleteResult = climbing_grades::Entity::delete_by_id(id)
            .exec(&self.db)
            .await?;
        if res.rows_affected == 1 {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    async fn insert(
        &self,
        new_climbing_grade: CreateClimbingGrade,
    ) -> Result<ClimbingGrade, RepositoryError> {
        let climbing_grade = climbing_grades::ActiveModel {
            name: Set(new_climbing_grade.name),
            grade_context: Set(new_climbing_grade.grade_context),
            numverical_value: Set(new_climbing_grade.numerical_value),
            ..Default::default()
        };
        let climbing_grade: climbing_grades::Model = climbing_grade.insert(&self.db).await?;
        Ok(ClimbingGrade::from(climbing_grade))
    }
}
