use crate::{dto::request::new_gym_dto::NewGymDto, structs::{climber::Climber, location::Location}};

pub struct CreateGym{
    pub name: String,
    pub location: Location,
    pub admin: Climber,
}

impl From<(NewGymDto, Climber)> for CreateGym {
    fn from(tuple : (NewGymDto, Climber)) -> Self{
        let (gym, admin) = tuple;
        CreateGym{
            name: gym.name,
            location: Location { longitude: gym.location_x, latitude: gym.location_y },
            admin: admin,
        }
    }
}
