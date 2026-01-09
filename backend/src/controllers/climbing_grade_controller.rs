use rocket::{State, get, serde::json::Json};
use rocket_autodocu::openapi;

use crate::{dto::response::climbing_grade_dto::ClimbingGradeDto, services::climbing_grade_service::ClimbingGradeService, structs::climbing_grade::ClimbingGrade};

#[openapi]
#[get("/climbing-grade")]
pub async fn get_climbing_grades(service: &State<ClimbingGradeService>) -> Option<Json<Vec<ClimbingGradeDto>>>{
    let climbing_grades: Vec<ClimbingGrade> = service.get_all_climbing_grades().await.ok()?;
    println!("Handling index route!");
    let dto : Vec<ClimbingGradeDto> = climbing_grades.into_iter().map(ClimbingGradeDto::from).collect();
    Some(Json(dto))
}
