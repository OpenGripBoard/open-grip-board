use rocket::{State, get, http::Status, patch, post, serde::json::Json};
use rocket_autodocu::openapi;
use validator::Validate;

use crate::{
    commands::create_climber::CreateClimber,
    dto::{
        request::{login_request_dto::LoginRequestDto, new_climber_dto::NewClimberDto, patch_climber_dto::PatchClimberFavouriteGymDto},
        response::{auth_response_dto::AuthResponseDto, climber_dto::ClimberDto},
    },
    guards::{auth_guard::AuthenticatedUser, rate_limit_guard::RateLimited},
    services::{climber_service::ClimberService, gym_service::GymService},
    structs::climber::Climber,
    utilities::jwt_util,
};

#[openapi]
#[get("/climber/<climber_id>")]
pub async fn get_climber(
    _auth: AuthenticatedUser,
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
) -> Result<Status, Status> {
    let new_climber_dto = new_climber_dto.into_inner();

    if let Err(e) = new_climber_dto.validate() {
        tracing::warn!("Validation failed for new climber: {:?}", e);
        return Err(Status::BadRequest);
    }

    let new_climber = CreateClimber::from(new_climber_dto);
    match service.create_climber(new_climber).await {
        Ok(_) => Ok(Status::Created),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[openapi]
#[post("/climber/login", format = "json", data = "<login_request_dto>")]
pub async fn post_climber_login(
    _rate_limit: RateLimited,
    service: &State<ClimberService>,
    login_request_dto: Json<LoginRequestDto>,
) -> Result<Json<AuthResponseDto>, Status> {
    let login_request = login_request_dto.into_inner();

    if let Err(e) = login_request.validate() {
        tracing::warn!("Validation failed for login: {:?}", e);
        return Err(Status::BadRequest);
    }

    let climber = service
        .authenticate_climber(login_request)
        .await
        .map_err(|_| Status::Forbidden)?;

    let token = jwt_util::create_token(climber.id, &climber.email)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(AuthResponseDto {
        token,
        climber_id: climber.id,
        username: climber.username,
    }))
}

#[openapi]
#[patch("/climber/<climber_id>/favourite-gyms", format = "json", data = "<climber_patches>")]
pub async fn patch_climber_favourite_gyms(
    auth: AuthenticatedUser,
    service: &State<ClimberService>,
    climber_id: i32,
    climber_patches: Json<PatchClimberFavouriteGymDto>,
    gym_service: &State<GymService>,
) -> Status {
    // Authorization check: users can only modify their own favourites
    if auth.climber_id != climber_id {
        return Status::Forbidden;
    }

    match service
        .patch_climber_favourite_gyms(climber_id, climber_patches.into_inner(), gym_service)
        .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}