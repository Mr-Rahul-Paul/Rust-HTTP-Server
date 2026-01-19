# Rust HTTP Server

High-performance, async HTTP server using **Axum**, **Tokio**, and **MongoDB**. Features built-in **Prometheus** metrics. Requires a preconfig'd Mongo URI with `appName`.

## Setup

```bash
# 1. Setup Env (Ensure appName is included)
echo 'mongodb_uri="mongodb://localhost:27017/?appName=<NameAnythingYouWant>"' > .env

# 2. Run
cargo run
```

## Endpoints

- `GET  /` - Health Check
- `GET  /metrics` - Prometheus Metrics
- `GET  /api/users` - List Users
- `POST /api/users` - Create User
- `GET  /download` - File Download

## Stack

- **Runtime**: Tokio
- **Web**: Axum
- **DB**: MongoDB
- **Observability**: Prometheus (`/metrics`)
