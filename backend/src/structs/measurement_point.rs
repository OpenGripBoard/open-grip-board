use sea_orm::prelude::DateTime;

pub struct MeasurementPoint{
    pub timestamp: DateTime,
    pub value: i32,
}