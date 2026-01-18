use rocket::{State, get, http::Status, patch, post, serde::json::Json};
use rocket_autodocu::openapi;

use crate::{
    commands::create_climber::CreateClimber,
    dto::{
        request::{
            login_request_dto::LoginRequestDto, new_climber_dto::NewClimberDto,
            patch_climber_dto::PatchClimberFavouriteGymDto,
        },
        response::climber_dto::ClimberDto,
    },
    guards::rate_limit_guard::RateLimited,
    services::{climber_service::ClimberService, gym_service::GymService},
    structs::climber::Climber,
};

#[openapi]
#[get("/climber/<climber_id>")]
pub async fn get_climber(
    _rate_limit: RateLimited,
    service: &State<ClimberService>,
    climber_id: i32,
) -> Option<Json<ClimberDto>> {
    let climber: Climber = service.get_climber(climber_id).await.ok()?;
    let dto = ClimberDto::from(climber);
    Some(Json(dto))
}

#[openapi]
#[post("/climber", format = "json", data = "<new_climber_dto>")]
pub async fn post_new_climber(
    _rate_limit: RateLimited,
    service: &State<ClimberService>,
    new_climber_dto: Json<NewClimberDto>,
) -> Status {
    let new_climber = CreateClimber::from(new_climber_dto.into_inner());
    match service.create_climber(new_climber).await {
        Ok(_) => Status::Created,
        Err(_) => Status::InternalServerError,
    }
}

#[openapi]
#[post("/climber/login", data = "<login_request_dto>")]
pub async fn post_climber_login(
    _rate_limit: RateLimited,
    service: &State<ClimberService>,
    login_request_dto: Json<LoginRequestDto>,
) -> Result<Json<ClimberDto>, Status> {
    let login_request = login_request_dto.into_inner();
    let climber = service
        .authenticate_climber(login_request)
        .await
        .map_err(|_| Status::Forbidden)?;
    let dto = ClimberDto::from(climber);
    Ok(Json(dto))
}

#[openapi]
#[patch(
    "/climber/<climber_id>/favourite-gyms",
    format = "json",
    data = "<climber_patches>"
)]
pub async fn patch_climber_favourite_gyms(
    _rate_limit: RateLimited,
    service: &State<ClimberService>,
    climber_id: i32,
    climber_patches: Json<PatchClimberFavouriteGymDto>,
    gym_service: &State<GymService>,
) -> Status {
    match service
        .patch_climber_favourite_gyms(climber_id, climber_patches.into_inner(), gym_service)
        .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
