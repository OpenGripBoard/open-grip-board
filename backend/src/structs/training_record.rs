use rocket::time::Time;

use crate::structs::{training_template::TrainingTemplate, exercise_record::ExerciseRecord};

pub struct TrainingRecord{
    id: i32,
    training_template: TrainingTemplate,
    exercise_records: Vec<ExerciseRecord>,
    total_duration: Time,
}