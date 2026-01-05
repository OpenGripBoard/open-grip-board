use crate::entity::src::climbers;

pub struct Climber{
    pub id: i32,
    email: String,
    pub username: String,
}

impl From<climbers::Model> for Climber {
    fn from(climber: climbers::Model) -> Self {
        Climber {
            id: climber.climber_id,
            email: climber.email,
            username: climber.username,
        }
    }
}