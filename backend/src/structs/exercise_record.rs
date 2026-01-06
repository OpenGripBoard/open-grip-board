use crate::structs::{exercise_template::ExerciseTemplate, record::Record};

pub struct ExerciseRecord{
    id: i32,
    exercise_template: ExerciseTemplate,
    was_successful: bool,
    records: Vec<Record>,
}