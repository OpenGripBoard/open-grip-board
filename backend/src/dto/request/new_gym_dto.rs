use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct NewGymDto {
    pub name: String,
    pub location_x: f32,
    pub location_y: f32,
    pub admin_id: i32,
}
