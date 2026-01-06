use rocket::{get, serde::json::Json};
use rocket_autodocu::openapi;

use crate::{dto::response::climbing_grade_dto::ClimbingGradeDto, services::climbing_grade_service, structs::climbing_grade::ClimbingGrade};

#[openapi]
#[get("/climbing-grade")]
pub async fn get_climbing_grades() -> Option<Json<Vec<ClimbingGradeDto>>>{
    let climbing_grades: Vec<ClimbingGrade> = climbing_grade_service::get_all_climbing_grades().await.ok()?;
    let dto : Vec<ClimbingGradeDto> = climbing_grades.into_iter().map(ClimbingGradeDto::from).collect();
    Some(Json(dto))
}
