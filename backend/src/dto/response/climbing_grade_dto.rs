use rocket_autodocu::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::climbing_grade::ClimbingGrade;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClimbingGradeDto{
    id: i32,
    name: String,
    grade_context: String,
    numerical_value: i32,
}

impl From<ClimbingGrade> for ClimbingGradeDto {
    fn from(grade: ClimbingGrade) -> Self {
        ClimbingGradeDto {            
            id: grade.id, 
            name: grade.name, 
            grade_context: grade.grade_context, 
            numerical_value: grade.numerical_value,
        }
    }
}
