use rocket::time::Time;

use crate::structs::{exercise_record::ExerciseRecord, training_template::TrainingTemplate};

#[allow(dead_code)]
pub struct TrainingRecord {
    id: i32,
    training_template: TrainingTemplate,
    exercise_records: Vec<ExerciseRecord>,
    total_duration: Time,
}
