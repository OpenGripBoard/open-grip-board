use entity::climbers;

use crate::structs::{
    gym::Gym, training_record::TrainingRecord, training_template::TrainingTemplate,
};

pub struct Climber {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub training_records: Option<Vec<TrainingRecord>>,
    pub favourite_trainings: Option<Vec<TrainingTemplate>>,
    pub favourite_gyms: Option<Vec<Gym>>,
}

impl From<climbers::Model> for Climber {
    fn from(climber: climbers::Model) -> Self {
        Climber {
            id: climber.climber_id,
            email: climber.email,
            username: climber.username,
            training_records: None,
            favourite_gyms: None,
            favourite_trainings: None,
        }
    }
}
