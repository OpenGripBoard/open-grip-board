use rocket_autodocu::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponseDto {
    pub token: String,
    pub climber_id: i32,
    pub username: String,
}
