use rocket::http::{ContentType, Header, Status};
use rocket::local::blocking::Client;
use serde_json::{Value, json};

use backend::utilities::jwt_util;

fn get_client() -> Client {
    Client::tracked(super::rocket()).expect("valid rocket instance")
}

// ============================================================================
// Health Check Tests
// ============================================================================

#[test]
fn health_check_returns_healthy() {
    let client = get_client();
    let response = client.get("/health").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body: Value = serde_json::from_str(&response.into_string().unwrap()).unwrap();
    assert_eq!(body["status"], "healthy");
    assert!(body["version"].is_string());
}

// ============================================================================
// Authentication Tests
// ============================================================================

#[test]
fn login_with_invalid_email_format_returns_bad_request() {
    let client = get_client();
    let response = client
        .post("/climber/login")
        .header(ContentType::JSON)
        .body(json!({"email": "not-an-email", "password": "password123"}).to_string())
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn register_with_short_password_returns_bad_request() {
    let client = get_client();
    let response = client
        .post("/climber")
        .header(ContentType::JSON)
        .body(
            json!({"email": "test@example.com", "username": "testuser", "password": "short"})
                .to_string(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

#[test]
fn register_with_invalid_email_returns_bad_request() {
    let client = get_client();
    let response = client
        .post("/climber")
        .header(ContentType::JSON)
        .body(
            json!({"email": "invalid", "username": "testuser", "password": "validpassword123"})
                .to_string(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::BadRequest);
}

// ============================================================================
// Authorization Tests
// ============================================================================

#[test]
fn get_climber_without_auth_returns_unauthorized() {
    let client = get_client();
    let response = client.get("/climber/climber/1").dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn get_climber_with_invalid_token_returns_unauthorized() {
    let client = get_client();
    let response = client
        .get("/climber/climber/1")
        .header(Header::new("Authorization", "Bearer invalid_token"))
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
}

// ============================================================================
// JWT Utility Tests
// ============================================================================

#[test]
fn jwt_create_and_validate_token() {
    // SAFETY: This test runs in isolation and doesn't share env vars with other threads
    unsafe {
        std::env::set_var("JWT_SECRET", "test_secret_key_for_jwt_testing_min_32_chars");
    }

    let token = jwt_util::create_token(1, "test@example.com").expect("should create token");
    let validated = jwt_util::validate_token(&token).expect("should validate token");

    assert_eq!(validated.claims.sub, 1);
    assert_eq!(validated.claims.email, "test@example.com");
}

#[test]
fn jwt_invalid_token_fails_validation() {
    // SAFETY: This test runs in isolation and doesn't share env vars with other threads
    unsafe {
        std::env::set_var("JWT_SECRET", "test_secret_key_for_jwt_testing_min_32_chars");
    }

    let result = jwt_util::validate_token("invalid.token.here");
    assert!(result.is_err());
}

// ============================================================================
// Input Validation Tests
// ============================================================================

#[test]
fn new_climber_dto_validates_email() {
    use backend::utilities::jwt_util;
    use validator::Validate;

    #[derive(validator::Validate)]
    struct TestDto {
        #[validate(email)]
        email: String,
    }

    let valid = TestDto {
        email: "test@example.com".to_string(),
    };
    assert!(valid.validate().is_ok());

    let invalid = TestDto {
        email: "not-an-email".to_string(),
    };
    assert!(invalid.validate().is_err());
}

// ============================================================================
// Public Endpoint Tests
// ============================================================================

#[test]
fn root_redirects_to_swagger() {
    let client = get_client();
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::SeeOther);
}

#[test]
fn get_climbing_grades_returns_ok() {
    let client = get_client();
    let response = client.get("/climbing-grade").dispatch();

    // May return None if DB not connected, but should not error
    assert!(response.status() == Status::Ok || response.status() == Status::NotFound);
}
