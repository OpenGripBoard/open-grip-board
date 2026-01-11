use crate::structs::measurement_point::MeasurementPoint;

pub trait Observer{
    fn update (&mut self, measurement_point: MeasurementPoint);
}