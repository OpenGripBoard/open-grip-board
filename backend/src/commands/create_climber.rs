use crate::dto::request::new_climber_dto::NewClimberDto;

pub struct CreateClimber{
    pub email: String,
    pub username: String,
}

impl From<NewClimberDto> for CreateClimber {
    fn from(climber: NewClimberDto) -> Self {
        CreateClimber { 
            email: climber.email, 
            username: climber.username,
        }
    }
}
