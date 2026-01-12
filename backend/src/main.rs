use std::env;

use rocket::{fairing::AdHoc, fs::FileServer, response::Redirect, serde::json::Json};
use rocket_autodocu::{openapi_get_routes, swagger_ui::*};
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde::Serialize;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use backend::{
    controllers::{
        climber::*, climbing_grade_controller::*, gym::*,
        hangboard_controller::get_hangboard_live_data,
    },
    services::{
        climber_service::ClimberService, climbing_grade_service::ClimbingGradeService,
        gym_service::GymService, mqtt_service::MqttService,
    },
};

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

static BASE_URI: &str = "http://127.0.0.1:8000";

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
}

#[get("/")]
fn get_root() -> Redirect {
    Redirect::to(format!("{}/swagger-ui/", BASE_URI))
}

#[get("/health")]
fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn configure_cors() -> rocket_cors::Cors {
    let allowed_origins = env::var("ALLOWED_ORIGINS")
        .map(|origins| {
            origins
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|_| vec!["http://localhost:3000".to_string()]);

    CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&allowed_origins),
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Patch,
            rocket::http::Method::Delete,
            rocket::http::Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::some(&[
            "Authorization",
            "Content-Type",
            "Accept",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration error")
}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    init_tracing();

    tracing::info!("Starting Open Grip Board backend");

    let cors = configure_cors();

    rocket::build()
        .attach(cors)
        .attach(AdHoc::try_on_ignite("Database", |rocket| async {
            let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            tracing::info!("Connecting to database...");

            let db = sea_orm::Database::connect(&db_url)
                .await
                .expect("Failed to connect to DB");

            tracing::info!("Database connected successfully");

            let climbing_grade_service = ClimbingGradeService::new(db.clone());
            let climber_service = ClimberService::new(db.clone());
            let gym_service = GymService::new(db.clone());
            let mqtt_service = MqttService::new();

            Ok(rocket
                .manage(climbing_grade_service)
                .manage(climber_service)
                .manage(gym_service)
                .manage(mqtt_service))
        }))
        .mount("/", routes![get_root, health_check])
        .mount(
            "/climber",
            routes![
                get_climber,
                post_new_climber,
                post_climber_login,
                patch_climber_favourite_gyms
            ],
        )
        .mount("/gym", routes![get_gym, get_gyms, post_new_gym])
        .mount(
            "/climbing-grade",
            routes![
                get_climbing_grades,
                create_climbing_grade,
                delete_climbing_grade
            ],
        )
        .mount("/hangboard", routes![get_hangboard_live_data])
        .mount(
            "/",
            openapi_get_routes![
                get_climber,
                post_new_climber,
                post_climber_login,
                patch_climber_favourite_gyms,
                get_gym,
                get_gyms,
                post_new_gym,
                get_climbing_grades,
                create_climbing_grade,
                delete_climbing_grade
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount("/assets", FileServer::from("./assets"))
}
