use entity::measurement_points;
use sea_orm::prelude::DateTime;

pub struct MeasurementPoint{
    pub timestamp: DateTime,
    pub value: i32,
}

impl From<measurement_points::Model> for MeasurementPoint {
    fn from(point: measurement_points::Model) -> Self {
        MeasurementPoint { 
            timestamp: point.measurement_point_timestamp,
            value: point.value,
        }
    }
}