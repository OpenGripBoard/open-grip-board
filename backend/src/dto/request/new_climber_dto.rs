use schemars::JsonSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, JsonSchema, Validate)]
pub struct NewClimberDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 2, max = 50, message = "Username must be 2-50 characters"))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}
