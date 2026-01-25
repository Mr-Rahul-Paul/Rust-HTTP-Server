# Rust HTTP Server

High-performance, async HTTP server using **Axum**, **Tokio**, and **MongoDB**. Features built-in **Prometheus** metrics. Requires a preconfig'd Mongo URI with `appName`.

The entire server containerised , packed with observabilty and metrics. can be run with a single command. 

## Setup

```bash
docker compose up --build

# http server:        http://localhost:3000
# Prometheus: http://localhost:9090
```

## capabilities : 
    - get and parse json  
    - post and parse json  
    - serve images  
    - metrics collection 
    
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
- **Observability**: Prometheus

---

> [!NOTE]
> **Security**: The `/metrics` endpoint (port 9090) exposes internal server metrics. When deploying to production, configure your cloud provider's firewall/security groups to restrict port 9090 to private/internal traffic only, allowing access solely from your Prometheus server.

or we can add a reverse proxy , that ony accepts our computers ip. gi
