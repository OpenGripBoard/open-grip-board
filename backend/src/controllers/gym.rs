use rocket::{State, get, serde::json::Json};
use rocket_autodocu::openapi;

use crate::{dto::response::gym_dto::GymDto, services::gym_service::GymService, structs::gym::Gym};

#[openapi]
#[get("/gym/<gym_id>")]
pub async fn get_gym(service: &State<GymService>, gym_id: i32) -> Option<Json<GymDto>>{
    let gym: Gym = service.get_gym(gym_id).await.ok()?;
    let dto = GymDto::from(gym);
    Some(Json(dto))
}