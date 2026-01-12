use rocket_autodocu::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::climber::Climber;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClimberDto {
    id: i32,
    name: String,
}

impl From<Climber> for ClimberDto {
    fn from(climber: Climber) -> Self {
        ClimberDto {
            id: climber.id,
            name: climber.username,
        }
    }
}
