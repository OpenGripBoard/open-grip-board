use sea_orm::prelude::DateTime;

pub struct MeasurementPoint{
    timestamp: DateTime,
    value: i32,
}