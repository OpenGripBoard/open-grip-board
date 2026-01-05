use rocket::{get, serde::json::Json};
use rocket_autodocu::openapi;

use crate::{dto::response::gym_dto::GymDto, services::gym_service, structs::gym::Gym};

#[openapi]
#[get("/gym/<gym_id>")]
pub async fn get_gym(gym_id: i32) -> Option<Json<GymDto>>{
    let gym: Gym = gym_service::get_gym(gym_id).await.ok()?;
    let dto = GymDto::from(gym);
    Some(Json(dto))
}