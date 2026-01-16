use entity::{grip_types, measurement_points, records};
use sea_orm::prelude::Time;

use crate::{
    structs::{grip_type::GripType, measurement_point::MeasurementPoint},
    traits::observer::Observer,
};

pub struct Record {
    pub id: i32,
    used_grip_type: GripType,
    measurement_points: Vec<MeasurementPoint>,
    duration: Time,
}

impl
    From<(
        records::Model,
        Vec<measurement_points::Model>,
        grip_types::Model,
    )> for Record
{
    fn from(
        tuple: (
            records::Model,
            Vec<measurement_points::Model>,
            grip_types::Model,
        ),
    ) -> Self {
        let (record, points, grip_type) = tuple;
        let grip_type = GripType::from(grip_type);
        let measurement_points = points.into_iter().map(MeasurementPoint::from).collect();
        Record {
            id: record.record_id,
            used_grip_type: grip_type,
            measurement_points: measurement_points,
            duration: record.duration,
        }
    }
}

impl Observer for Record {
    fn update(&mut self, measurement_point: MeasurementPoint) {
        self.measurement_points.push(measurement_point);
    }
}
