use rocket::time::Time;

use crate::structs::{climber::Climber,climbing_grade::ClimbingGrade, exercise_template::ExerciseTemplate};

pub struct TrainingTemplate{
    id: i32,
    name: String,
    creator: Climber,
    grade: ClimbingGrade,
    predicted_duration: Time,
    exercises: Option<Vec<ExerciseTemplate>>
}