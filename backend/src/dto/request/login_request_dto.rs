use schemars::JsonSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, JsonSchema, Validate)]
pub struct LoginRequestDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}