use sea_orm::DatabaseConnection;

use crate::{errors::errors::RepositoryError, repositories::climbing_grade_repo::ClimbingGradeRepo, structs::climbing_grade::ClimbingGrade};

pub struct ClimbingGradeService {
    repo: ClimbingGradeRepo,
}

impl ClimbingGradeService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { 
            repo: ClimbingGradeRepo::new(db),
        }
    }

    pub async fn get_all_climbing_grades(&self) -> Result<Vec<ClimbingGrade>, RepositoryError>{
        let climbing_grades_option = self.repo.find_all().await?;
        climbing_grades_option.ok_or(RepositoryError::NotFound)
    }
}
