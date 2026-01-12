use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_autodocu::Result;
use rocket_autodocu::r#gen::OpenApiGenerator;
use rocket_autodocu::request::{OpenApiFromRequest, RequestHeaderInput};

use crate::utilities::jwt_util::{self, Claims};

pub struct AuthenticatedUser {
    pub climber_id: i32,
    pub email: String,
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");

        match auth_header {
            Some(header) => {
                if !header.starts_with("Bearer ") {
                    return Outcome::Error((Status::Unauthorized, AuthError::InvalidToken));
                }

                let token = &header[7..];

                match jwt_util::validate_token(token) {
                    Ok(token_data) => {
                        let claims: Claims = token_data.claims;
                        Outcome::Success(AuthenticatedUser {
                            climber_id: claims.sub,
                            email: claims.email,
                        })
                    }
                    Err(_) => Outcome::Error((Status::Unauthorized, AuthError::InvalidToken)),
                }
            }
            None => Outcome::Error((Status::Unauthorized, AuthError::MissingToken)),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for AuthenticatedUser {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
