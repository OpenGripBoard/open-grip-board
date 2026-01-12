use sea_orm::prelude::DateTime;

use crate::structs::grip_type::GripType;

#[allow(dead_code)]
pub struct ExerciseTemplate {
    id: i32,
    active_duration: DateTime,
    active_force: i32,
    grip_type: GripType,
    rest_duration: DateTime,
    rest_force: i32,
}
