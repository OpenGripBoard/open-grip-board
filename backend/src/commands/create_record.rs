use sea_orm::prelude::Time;

use crate::structs::{grip_type::GripType, measurement_point::MeasurementPoint};

pub struct CreateRecord {
    pub used_grip_type: GripType,
    pub measurement_points: Vec<MeasurementPoint>,
    pub duration: Time,
}
