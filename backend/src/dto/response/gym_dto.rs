use rocket_autodocu::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::gym::Gym;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GymDto{
    name: String,
    longitude: f32,
    latitude: f32,
    admin_climber_id: i32,
}

impl From<Gym> for GymDto {
    fn from(gym: Gym) -> Self {
        GymDto {
            name: gym.name,
            longitude: gym.location.longitude,
            latitude: gym.location.latitude,
            admin_climber_id: gym.admin.id
        }
    }
}
