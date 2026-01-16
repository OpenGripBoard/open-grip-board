use rocket::{State, delete, get, http::Status, post, serde::json::Json};
use rocket_autodocu::openapi;

use crate::{
    commands::create_climbing_grade::CreateClimbingGrade,
    dto::{
        request::new_climbing_grade_dto::NewClimbingGradeDto,
        response::climbing_grade_dto::ClimbingGradeDto,
    },
    services::climbing_grade_service::ClimbingGradeService,
    structs::climbing_grade::ClimbingGrade,
};

#[openapi]
#[get("/climbing-grade")]
pub async fn get_climbing_grades(
    service: &State<ClimbingGradeService>,
) -> Option<Json<Vec<ClimbingGradeDto>>> {
    let climbing_grades: Vec<ClimbingGrade> = service.get_all_climbing_grades().await.ok()?;
    let dto: Vec<ClimbingGradeDto> = climbing_grades
        .into_iter()
        .map(ClimbingGradeDto::from)
        .collect();
    Some(Json(dto))
}

#[openapi]
#[post("/climbing-grade", format = "json", data = "<new_climbing_grade_dto>")]
pub async fn create_climbing_grade(
    service: &State<ClimbingGradeService>,
    new_climbing_grade_dto: Json<NewClimbingGradeDto>,
) -> Status {
    match service
        .create_climbing_grade(CreateClimbingGrade::from(
            new_climbing_grade_dto.into_inner(),
        ))
        .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[openapi]
#[delete("/climbing-grade/<grade_id>")]
pub async fn delete_climbing_grade(service: &State<ClimbingGradeService>, grade_id: i32) -> Status {
    match service.delete_climbing_grade_by_id(grade_id).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
