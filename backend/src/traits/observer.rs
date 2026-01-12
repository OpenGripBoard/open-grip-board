use crate::structs::measurement_point::MeasurementPoint;

#[allow(dead_code)]
pub trait Observer {
    fn update(&mut self, measurement_point: MeasurementPoint);
}
