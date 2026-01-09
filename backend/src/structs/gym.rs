use crate::structs::{climber::Climber, location::Location, hangboard::Hangboard};

pub struct Gym{
    pub id: i32,
    pub name: String,
    pub location: Location,
    pub admin: Climber,
    pub hangboards: Option<Vec<Hangboard>>,
}
