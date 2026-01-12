use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String,
}
