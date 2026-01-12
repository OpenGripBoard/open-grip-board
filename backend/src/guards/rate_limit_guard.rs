use std::num::NonZeroU32;
use std::sync::LazyLock;

use governor::clock::DefaultClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket_autodocu::r#gen::OpenApiGenerator;
use rocket_autodocu::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_autodocu::Result;

type GlobalLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

static RATE_LIMITER: LazyLock<GlobalLimiter> = LazyLock::new(|| {
    RateLimiter::direct(Quota::per_second(NonZeroU32::new(10).unwrap()))
});

pub struct RateLimited;

#[derive(Debug)]
pub struct RateLimitError;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RateLimited {
    type Error = RateLimitError;

    async fn from_request(_request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match RATE_LIMITER.check() {
            Ok(_) => Outcome::Success(RateLimited),
            Err(_) => Outcome::Error((Status::TooManyRequests, RateLimitError)),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for RateLimited {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
