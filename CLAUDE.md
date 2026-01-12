# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Open Grip Board is a grip strength measuring board system with a Rust backend API. The system integrates with physical hangboard hardware via MQTT for real-time measurements.

## Development Setup

1. Copy `.env.example` to `.env` and configure values
2. Set up MQTT password file:
   ```bash
   ./scripts/setup-mqtt-passwd.sh <username> <password>
   ```
3. Start all services:
   ```bash
   docker compose up -d
   ```

The backend runs on port 8000 with Swagger UI at http://127.0.0.1:8000/swagger-ui

## Common Commands

### Backend (from `backend/` directory)

Build:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Run a single test:
```bash
cargo test <test_name>
```

Format code:
```bash
cargo fmt
```

Lint:
```bash
cargo clippy
```

### Database Migrations (from `backend/migration/` directory)

Apply migrations:
```bash
cargo run
```

Generate new migration:
```bash
cargo run -- generate MIGRATION_NAME
```

Rollback last migration:
```bash
cargo run -- down
```

Reset and reapply all migrations:
```bash
cargo run -- fresh
```

### Entity Generation (from `backend/` directory)

Regenerate SeaORM entities from database schema:
```bash
sea-orm-cli generate entity --output-dir ./entity/src -l
```

## Architecture

### Backend Structure (`backend/src/`)

The Rust backend uses Rocket web framework with SeaORM for database access:

- **controllers/** - HTTP endpoint handlers (climber, gym, climbing_grade, hangboard)
- **services/** - Business logic layer injected as Rocket managed state
- **repositories/** - Database access layer using SeaORM
- **guards/** - Rocket request guards for auth and rate limiting
- **commands/** - Command objects for create operations
- **structs/** - Domain models
- **dto/** - Data transfer objects with validation (using `validator` crate)
- **utilities/** - Helper functions (password hashing, JWT)
- **entity/** - SeaORM entity definitions (auto-generated from DB schema)
- **migration/** - Database migrations using sea-orm-migration

### Authentication & Authorization

- JWT-based authentication via `Authorization: Bearer <token>` header
- Tokens issued on successful login (`POST /climber/login`)
- Protected routes use `AuthenticatedUser` request guard
- Authorization checks prevent users from modifying other users' data

### Request Guards

- `AuthenticatedUser` - Validates JWT and extracts user claims
- `RateLimited` - Global rate limiting (10 req/sec)

### Service Layer Pattern

Services are initialized in `main.rs` during Rocket ignition and managed as state:
- `ClimberService` - User/climber management with Argon2 password hashing
- `GymService` - Gym CRUD operations
- `ClimbingGradeService` - Climbing grade management
- `MqttService` - Real-time hangboard data via MQTT subscriptions

### Infrastructure

- **PostgreSQL** - Primary database (port 5432)
- **Mosquitto** - MQTT broker for hangboard sensor data (port 1883, WebSocket 8083)
- **Swagger/OpenAPI** - Auto-generated API documentation via rocket-autodocu

### Key Dependencies

- `rocket` - Web framework
- `rocket_cors` - CORS support
- `sea-orm` - Async ORM with PostgreSQL support
- `rumqttc` - MQTT client for hangboard integration
- `argon2` - Password hashing
- `jsonwebtoken` - JWT authentication
- `validator` - Input validation
- `tracing` - Structured logging
- `governor` - Rate limiting
- `rocket-autodocu` - OpenAPI/Swagger generation

## Environment Variables

See `.env.example` for all required configuration. Key variables:
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret for signing JWTs (min 32 chars)
- `MQTT_HOST`, `MQTT_PORT`, `MQTT_USERNAME`, `MQTT_PASSWORD` - MQTT broker config
- `ALLOWED_ORIGINS` - Comma-separated CORS origins
- `RUST_LOG` - Log level (trace/debug/info/warn/error)
