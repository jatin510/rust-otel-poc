# Rust OpenTelemetry with Jaeger

This project demonstrates a basic Rust application with OpenTelemetry instrumentation and Jaeger tracing.

## Features

- Health check endpoint (`/health`)
- User information API (GET `/users`, GET `/users/{id}`, POST `/users`)
- JSON-based data storage
- OpenTelemetry instrumentation with Jaeger exporter
- Dockerized setup with Jaeger and the Rust application

## Prerequisites

- Rust (1.60+)
- Docker and Docker Compose (for running with Jaeger)

## Getting Started

### Running Locally

1. Clone the repository
2. Set up a Jaeger instance:
   ```
   docker run -d --name jaeger \
     -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 \
     -e COLLECTOR_OTLP_ENABLED=true \
     -p 6831:6831/udp \
     -p 6832:6832/udp \
     -p 5778:5778 \
     -p 16686:16686 \
     -p 4317:4317 \
     -p 4318:4318 \
     -p 14250:14250 \
     -p 14268:14268 \
     -p 14269:14269 \
     -p 9411:9411 \
     jaegertracing/all-in-one:latest
   ```
3. Start the application:
   ```
   RUST_LOG=info cargo run
   ```

### Running with Docker Compose

1. Build and start the services:
   ```
   docker-compose up -d
   ```

## API Usage

### Health Check

```
GET /health
```

Response:
```json
{
  "status": "healthy"
}
```

### List Users

```
GET /users
```

Response:
```json
[
  {
    "id": "1",
    "name": "John Doe",
    "email": "john@example.com"
  }
]
```

### Get User by ID

```
GET /users/{id}
```

Response:
```json
{
  "id": "1",
  "name": "John Doe",
  "email": "john@example.com"
}
```

### Create User

```
POST /users
Content-Type: application/json

{
  "id": "2",
  "name": "Jane Smith",
  "email": "jane@example.com"
}
```

Response:
```json
{
  "id": "2",
  "name": "Jane Smith",
  "email": "jane@example.com"
}
```

## Viewing Traces

Access the Jaeger UI at `http://localhost:16686` to view traces from your application.

## Project Structure

- `src/main.rs` - Main application code
- `Cargo.toml` - Rust dependencies
- `docker-compose.yml` - Docker Compose configuration
- `Dockerfile` - Docker configuration for the Rust application
- `users.json` - JSON file for storing user data 