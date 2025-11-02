# Rust HTTP Server

A high-performance asynchronous HTTP server built with Rust, providing RESTful API endpoints for user management with MongoDB integration.

## Overview

HTTP server built with Axum web framework and Tokio runtime. Provides REST API for managing user data with MongoDB database connectivity.

### Features

- High performance with Rust's async/await and zero-cost abstractions
- Type-safe request/response handling at compile time
- MongoDB integration with async database operations
- RESTful API endpoints following HTTP standards
- Concurrent request processing via Tokio runtime
- Environment-based configuration via .env file
- Health check endpoint for service monitoring

## Architecture

### Tech Stack

- Axum - Web framework for async HTTP services
- Tokio - Async runtime
- MongoDB - NoSQL document database
- Serde - JSON serialization/deserialization
- dotenv - Environment variable management

### Project Structure

```
Rust-HTTP-Server/
├── src/
│   └── main.rs          # Main application logic and route handlers
├── Cargo.toml           # Project dependencies and metadata
├── Cargo.lock           # Dependency lock file
└── .env                 # Environment variables (create this file)
```
## Getting Started

### Prerequisites

- Rust (latest stable version)
- MongoDB (local or Atlas cloud instance)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd Rust-HTTP-Server
```

2. Create `.env` file in project root:
```env
MONGODB_URI=mongodb+srv://username:password@cluster.mongodb.net/?retryWrites=true&w=majority
```

For local MongoDB:
```env
MONGODB_URI=mongodb://localhost:27017
```

3. Configure MongoDB: Whitelist your IP in Atlas Network Access settings if using cloud, or ensure local MongoDB is running

4. Build and run:
```bash
cargo build
cargo run
```

Server starts on `http://127.0.0.1:3000`

## API Endpoints

**GET /health** - Health check endpoint
```bash
curl http://127.0.0.1:3000/health
```

**GET /api/users** - Retrieve all users
```bash
curl http://127.0.0.1:3000/api/users
```

**POST /api/users** - Create new user
```bash
curl -X POST http://127.0.0.1:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Jane Doe", "email": "jane@example.com", "password": "password"}'
```
## Future Scope

**File Transfer**
- Multipart file upload endpoints
- File storage integration (local filesystem or cloud storage)
- File metadata storage in MongoDB
- Secure file download with access control
- File validation (size limits, type checking)
- Chunked uploads for large files

**Video Transfer**
- Video file upload with progress tracking
- HTTP video streaming (MP4, HLS, DASH)
- FFmpeg integration for transcoding, thumbnails, compression
- Range request support for video seeking
- Video playlist management
- Background video transcoding pipeline

**Security & Authentication**
- JWT-based user authentication
- Role-based access control
- API key authentication
- OAuth integration

**Additional Features**
- Redis caching layer
- Structured logging and tracing
- Prometheus metrics
- OpenAPI/Swagger documentation
- Unit and integration tests
- Docker containerization
- WebSocket support

## Development

Build for production:
```bash
cargo build --release
```

Run tests:
```bash
cargo test
```

## Contributing

Contributions welcome. Fork the repository, create a feature branch, and submit a pull request.

## License

MIT License

---

Note: This server is in active development. Review and test thoroughly before production deployment.

