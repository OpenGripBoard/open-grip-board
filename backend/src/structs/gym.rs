use entity::{climbers, gyms, hangboards};

use crate::structs::{climber::Climber, hangboard::Hangboard, location::Location};

pub struct Gym {
    pub id: i32,
    pub name: String,
    pub location: Location,
    pub admin: Climber,
    pub hangboards: Option<Vec<Hangboard>>,
}

impl From<(gyms::Model, Option<climbers::Model>, Vec<hangboards::Model>)> for Gym {
    fn from(tuple: (gyms::Model, Option<climbers::Model>, Vec<hangboards::Model>)) -> Self {
        let (gym, admin, hangboards) = tuple;
        let climber: Option<Climber> = admin.map(Climber::from);
        let hangboards: Option<Vec<Hangboard>> = {
            let vec: Vec<Hangboard> = hangboards.into_iter().map(Hangboard::from).collect();
            if vec.is_empty() { None } else { Some(vec) }
        };
        Gym {
            id: gym.gym_id,
            name: gym.name,
            location: Location {
                longitude: gym.location_x,
                latitude: gym.location_y,
            },
            admin: climber.unwrap(),
            hangboards,
        }
    }
}
