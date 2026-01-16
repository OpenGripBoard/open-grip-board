use sea_orm::DatabaseConnection;

use crate::{
    commands::create_climbing_grade::CreateClimbingGrade,
    errors::errors::RepositoryError,
    repositories::{climbing_grade_repo::ClimbingGradeRepo, crud_repo::CrudRepo},
    structs::climbing_grade::ClimbingGrade,
};

pub struct ClimbingGradeService {
    repo: ClimbingGradeRepo,
}

impl ClimbingGradeService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repo: ClimbingGradeRepo::new(db),
        }
    }

    pub async fn get_all_climbing_grades(&self) -> Result<Vec<ClimbingGrade>, RepositoryError> {
        let climbing_grades_option = self.repo.find_all().await?;
        climbing_grades_option.ok_or(RepositoryError::NotFound)
    }

    pub async fn create_climbing_grade(
        &self,
        new_grade: CreateClimbingGrade,
    ) -> Result<ClimbingGrade, RepositoryError> {
        self.repo.insert(new_grade).await
    }

    pub async fn delete_climbing_grade(
        &self,
        climbing_grade: ClimbingGrade,
    ) -> Result<(), RepositoryError> {
        self.delete_climbing_grade_by_id(climbing_grade.id).await
    }

    pub async fn delete_climbing_grade_by_id(
        &self,
        climbing_grade_id: i32,
    ) -> Result<(), RepositoryError> {
        self.repo.delete_by_id(climbing_grade_id).await
    }
}
