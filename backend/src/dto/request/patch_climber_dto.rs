use schemars::JsonSchema;
use serde::Deserialize;

use crate::dto::request::patch_operation::PatchOperation;

#[derive(Deserialize, JsonSchema)]
pub struct PatchClimberFavouriteGymDto{
    pub patch_operation: PatchOperation,
    pub favourite_gym_id: i32,
}