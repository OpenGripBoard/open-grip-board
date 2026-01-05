use crate::structs::{climber::Climber, location::Location};

pub struct Gym{
    pub id: i32,
    pub name: String,
    pub location: Location,
    pub admin: Climber,
}
