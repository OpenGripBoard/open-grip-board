use crate::dto::request::new_climbing_grade_dto::NewClimbingGradeDto;

pub struct CreateClimbingGrade {
    pub name: String,
    pub grade_context: String,
    pub numerical_value: i32,
}

impl From<NewClimbingGradeDto> for CreateClimbingGrade {
    fn from(grade: NewClimbingGradeDto) -> Self {
        CreateClimbingGrade {
            name: grade.name,
            grade_context: grade.grade_context,
            numerical_value: grade.numerical_value,
        }
    }
}
