use rocket::{State, get, http::Status, post, serde::json::Json};
use rocket_autodocu::openapi;

use crate::{commands::create_gym::CreateGym, dto::{request::new_gym_dto::NewGymDto, response::gym_dto::GymDto}, services::{climber_service::ClimberService, gym_service::GymService}, structs::{climber::Climber, gym::Gym}};

#[openapi]
#[get("/gym/<gym_id>")]
pub async fn get_gym(service: &State<GymService>, gym_id: i32) -> Option<Json<GymDto>>{
    let gym: Gym = service.get_gym(gym_id).await.ok()?;
    let dto = GymDto::from(gym);
    Some(Json(dto))
}

#[openapi]
#[get("/gym")]
pub async fn get_gyms(service: &State<GymService>) -> Option<Json<Vec<GymDto>>>{
    let gyms: Vec<Gym> = service.get_all_gyms().await.ok()?;
    let dto : Vec<GymDto> = gyms.into_iter().map(GymDto::from).collect();
    Some(Json(dto))
}

#[openapi]
#[post("/gym", format="json", data="<new_gym_dto>")]
pub async fn post_new_gym(service: &State<GymService>, new_gym_dto: Json<NewGymDto>, climber_service: &State<ClimberService>) -> Status {
    let new_gym_dto: NewGymDto = new_gym_dto.into_inner();
    let admin: Climber = match climber_service.get_climber(new_gym_dto.admin_id).await{
        Ok(climber) => climber,
        Err(_) => return Status::BadRequest,
    };
    let create_gym: CreateGym = CreateGym::from((new_gym_dto, admin));
    match service.create_gym(create_gym).await {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}
