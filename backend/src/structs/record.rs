use rocket::time::Time;

use crate::structs::{grip_type::GripType, measurement_point::MeasurementPoint};

pub struct Record{
    id:i32,
    used_grip_type: GripType,
    measurement_points: Vec<MeasurementPoint>,
    duration: Time,
}