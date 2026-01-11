use rocket::{fairing::AdHoc, fs::FileServer, response::Redirect};
use rocket_autodocu::{openapi_get_routes, swagger_ui::*};
use backend::{controllers::{climber::*, climbing_grade_controller::*, gym::*}, services::{climber_service::ClimberService, climbing_grade_service::ClimbingGradeService, gym_service::GymService}};

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

static BASE_URI: &str = "http://127.0.0.1:8000";

#[get("/")]
fn get_root()-> Redirect{
    Redirect::to(format!("{}/swagger-ui", BASE_URI,))
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
        .mount("/", routes![get_root])
        .mount("/climber", routes![get_climber, post_new_climber, post_climber_login])
        .mount("/gym", routes![get_gym, get_gyms, post_new_gym])
        .mount("/climbing-grade", routes![get_climbing_grades, create_climbing_grade, delete_climbing_grade])
        .mount("/", openapi_get_routes![get_climber, post_new_climber, post_climber_login, get_gym, get_gyms, post_new_gym, get_climbing_grades, create_climbing_grade, delete_climbing_grade],)
        .mount("/swagger-ui/", make_swagger_ui(&SwaggerUIConfig {url: "../openapi.json".to_owned(), ..Default::default()}),)
        .mount("/assets", FileServer::from("./assets"))
}
