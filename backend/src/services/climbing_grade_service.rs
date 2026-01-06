use sea_orm::DatabaseConnection;

use crate::{errors::errors::RepositoryError, repositories::climbing_grade_repo::ClimbingGradeRepo, structs::climbing_grade::ClimbingGrade};

pub async fn get_all_climbing_grades() -> Result<Vec<ClimbingGrade>, RepositoryError>{
    let db : DatabaseConnection = sea_orm::Database::connect("postgres://user:pass@localhost/db").await?;
    let climbing_grade_repo: ClimbingGradeRepo = ClimbingGradeRepo::new(db);
    let climbing_grades_option = climbing_grade_repo.find_all().await?;
    climbing_grades_option.ok_or(RepositoryError::NotFound)
}