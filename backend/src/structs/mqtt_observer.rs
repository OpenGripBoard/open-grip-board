use rocket::tokio::sync::broadcast;

use crate::{structs::measurement_point::MeasurementPoint, traits::observer::Observer};

pub struct MqttObserver {
    pub current_point: broadcast::Sender<String>,
}

impl Observer for MqttObserver {
    fn update(&mut self, measurement_point: MeasurementPoint) {
        self.current_point.send(measurement_point.value.to_string());
    }
}
