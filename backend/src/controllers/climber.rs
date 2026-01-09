use rocket::{State, get, http::Status, post, serde::json::Json};
use rocket_autodocu::openapi;

use crate::{commands::create_climber::CreateClimber, dto::{request::new_climber_dto::NewClimberDto, response::climber_dto::ClimberDto}, services::climber_service::ClimberService, structs::climber::Climber};

#[openapi]
#[get("/climber/<climber_id>")]
pub async fn get_climber(service: &State<ClimberService>, climber_id: i32) -> Option<Json<ClimberDto>>{
    let climber: Climber = service.get_climber(climber_id).await.ok()?;
    let dto = ClimberDto::from(climber);
    Some(Json(dto))
}

#[openapi]
#[post("/climber", format="json", data = "<new_climber_dto>")]
pub async fn post_new_climber(service: &State<ClimberService>, new_climber_dto: Json<NewClimberDto>) -> Status {
    let new_climber = CreateClimber::from(new_climber_dto.into_inner());
    match service.create_climber(new_climber).await {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}
