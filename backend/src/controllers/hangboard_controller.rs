use rocket::{
    State, get,
    response::stream::{Event, EventStream},
};

use crate::services::mqtt_service::MqttService;

#[get("/hangboard/<hangboard_id>/live")]
pub fn get_hangboard_live_data(service: &State<MqttService>, hangboard_id: i32) -> EventStream![] {
    let mut rx = service
        .get_subscription_by_id(hangboard_id)
        .newest_message
        .subscribe();
    EventStream! {
        loop {
            let msg = match rx.recv().await{
                    Ok(msg) => msg,
                    Err(_) => break,
            };
            yield Event::data(msg);
        }
    }
}

// #[openapi]
// #[post("/hangboard/<hangboard_id>")]
// pub async fn  post_go_online(service: &State<Mutex<MqttService>>, hangboard_id: i32)  -> Status{
//         let mut service = service.lock().unwrap();
//     service.subscribe_to_topic(hangboard_id.to_string()).await;
//     Status::Ok
// }
