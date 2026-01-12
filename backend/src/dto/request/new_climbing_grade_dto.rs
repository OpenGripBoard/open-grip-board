use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct NewClimbingGradeDto {
    pub name: String,
    pub grade_context: String,
    pub numerical_value: i32,
}
