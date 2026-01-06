use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct NewClimberDto{
    pub email: String,
    pub username: String,
}