use crate::structs::{climber::Climber, location::Location};

pub struct CreateGym{
    name: String,
    location: Location,
    admin: Climber,
}