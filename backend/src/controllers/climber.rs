use rocket::{get, serde::json::Json};
use rocket_autodocu::openapi;

use crate::{dto::response::climber_dto::ClimberDto, services::climber_service, structs::climber::Climber};

#[openapi]
#[get("/climber/<climber_id>")]
pub async fn get_climber(climber_id: i32) -> Option<Json<ClimberDto>>{
    let climber: Climber = climber_service::get_climber(climber_id).await.ok()?;
    let dto = ClimberDto::from(climber);
    Some(Json(dto))
}
