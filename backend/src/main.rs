use rocket::fairing::AdHoc;
use rocket_autodocu::{openapi, openapi_get_routes, swagger_ui::*};
use backend::{controllers::{climber::*, climbing_grade_controller::*, gym::*}, services::{climber_service::ClimberService, climbing_grade_service::ClimbingGradeService, gym_service::GymService}};

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "ру")]
    Russian
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[openapi]
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

// Try visiting:
//   http://127.0.0.1:8000/hello/мир
#[get("/мир")]
fn mir() -> &'static str {
    "Привет, мир!"
}

// Try visiting:
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<age>", rank = 2)]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

// Note: without the `..` in `opt..`, we'd need to pass `opt.emoji`, `opt.name`.
//
// Try visiting:
//   http://127.0.0.1:8000/?emoji
//   http://127.0.0.1:8000/?name=Rocketeer
//   http://127.0.0.1:8000/?lang=ру
//   http://127.0.0.1:8000/?lang=ру&emoji
//   http://127.0.0.1:8000/?emoji&lang=en
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en
//   http://127.0.0.1:8000/?emoji&name=Rocketeer
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en&emoji
//   http://127.0.0.1:8000/?lang=ru&emoji&name=Rocketeer
#[get("/?<lang>&<opt..>")]
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("👋 ");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Database", |rocket| async {
                let db = sea_orm::Database::connect(
                    "postgresql://username:password@127.0.0.1:5432/default_database"
                ).await.expect("Failed to connect to DB");
                let climbing_grade_service = ClimbingGradeService::new(db.clone());
                let climber_service = ClimberService::new(db.clone());
                let gym_service = GymService::new(db.clone());
                Ok(rocket
                    .manage(climbing_grade_service)
                    .manage(climber_service)
                    .manage(gym_service))
            }))
        .mount("/", routes![hello])
        .mount("/hello", routes![world, mir])
        .mount("/wave", routes![wave])
        .mount("/climber", routes![get_climber, post_new_climber])
        .mount("/gym", routes![get_gym])
        .mount("/climbing-grade", routes![get_climbing_grades])
        .mount("/", openapi_get_routes![world, get_climber, post_new_climber, get_gym, get_climbing_grades],)
        .mount("/swagger-ui/", make_swagger_ui(&SwaggerUIConfig {url: "../openapi.json".to_owned(), ..Default::default()}),)
}
