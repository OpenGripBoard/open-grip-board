use entity::climbing_grades;

pub struct ClimbingGrade {
    pub id: i32,
    pub name: String,
    pub grade_context: String,
    pub numerical_value: i32,
}

impl From<climbing_grades::Model> for ClimbingGrade {
    fn from(grade: climbing_grades::Model) -> Self {
        ClimbingGrade {
            id: grade.climbing_grade_id,
            name: grade.name,
            grade_context: grade.grade_context,
            numerical_value: grade.numverical_value,
        }
    }
}
