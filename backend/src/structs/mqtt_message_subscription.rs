use rocket::tokio::sync::broadcast::{self};

#[derive(Debug, Clone)]
pub struct MqttMessageSubscription {
    pub topic: String,
    pub newest_message: broadcast::Sender<String>,
}

impl MqttMessageSubscription {
    pub fn new(topic: String, first_value: String) -> Self {
        let (tx, _rx) = broadcast::channel(16);
        tx.send(first_value.clone()).unwrap();
        Self {
            topic,
            newest_message: tx,
        }
    }

    // pub fn register_observer(&mut self, observer: Record){
    //     self.observers.push(observer);
    // }

    // pub fn unregister_observer(&mut self, observer: Record){
    //     self.observers.retain(|x| x.id != observer.id);
    // }

    // pub fn notify_observers(&mut self) {
    //     let new_value = self.get_newest_value();

    //     for observer in self.observers.iter_mut() {
    //         observer.update(new_value.clone());
    //     }
    // }

    // fn get_newest_value(&self) -> MeasurementPoint {
    //     self.measurement_points.first().unwrap().clone()
    // }
}
