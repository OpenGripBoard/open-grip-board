use std::env;
use std::error::Error;
use std::time::Duration;

use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};

use crate::errors::MqttError;
use crate::structs::mqtt_message_subscription::MqttMessageSubscription;

pub struct MqttService {
    message_subscriptions: Vec<MqttMessageSubscription>,
    mqtt_host: String,
    mqtt_port: u16,
    mqtt_username: Option<String>,
    mqtt_password: Option<String>,
}

impl Default for MqttService {
    fn default() -> Self {
        Self::new()
    }
}

impl MqttService {
    pub fn new() -> Self {
        let mqtt_host = env::var("MQTT_HOST").unwrap_or_else(|_| "broker".to_string());
        let mqtt_port: u16 = env::var("MQTT_PORT")
            .unwrap_or_else(|_| "1883".to_string())
            .parse()
            .unwrap_or(1883);
        let mqtt_username = env::var("MQTT_USERNAME").ok();
        let mqtt_password = env::var("MQTT_PASSWORD").ok();

        Self {
            message_subscriptions: vec![],
            mqtt_host,
            mqtt_port,
            mqtt_username,
            mqtt_password,
        }
    }

    pub fn get_subscription_by_id(&self, hangboard_id: i32) -> MqttMessageSubscription {
        self.message_subscriptions
            .iter()
            .find(|subscription| subscription.topic == hangboard_id.to_string())
            .unwrap()
            .clone()
    }

    fn store_message(&self, topic: String, value: String) -> Result<(), MqttError> {
        let subscription = self.get_subscription_by_id(topic.parse().unwrap());
        match subscription.newest_message.send(value) {
            Ok(_) => Ok(()),
            Err(_) => Err(MqttError::NotFound),
        }
    }

    fn create_subscription(&mut self, topic: String, first_value: String) {
        let new_container = MqttMessageSubscription::new(topic, first_value);
        self.message_subscriptions.push(new_container);
    }

    pub async fn subscribe_to_topic(&mut self, topic: String) -> Result<(), Box<dyn Error>> {
        self.create_subscription(topic.clone(), "first".to_string());
        let mqtt_options = self.initialize_mqtt_client();
        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
        client.subscribe(topic.clone(), QoS::AtMostOnce).await?;
        while let Ok(event) = eventloop.poll().await {
            if let Event::Incoming(Incoming::Publish(publish)) = event {
                let payload = String::from_utf8_lossy(&publish.payload).to_string();
                let _ = self.store_message(topic.to_string(), payload.parse().unwrap());
            }
        }
        Ok(())
    }

    fn initialize_mqtt_client(&self) -> MqttOptions {
        let mut mqttoptions =
            MqttOptions::new("gripboard-backend", &self.mqtt_host, self.mqtt_port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        if let (Some(username), Some(password)) = (&self.mqtt_username, &self.mqtt_password) {
            mqttoptions.set_credentials(username, password);
        }

        mqttoptions
    }
}
