# serverless-llm-relay

High-throughput Rust LLM relay, companion memory store, and webhook bridge.

## Architecture

```text
[Telegram / HTTP Client] ---> [Axum Gateway] ---> [Relay Pipeline]
                                                       |
                       +-------------------------------+-------------------------------+
                       |                                                               |
                       v                                                               v
          [SQLite Memory & Persona]                                   [Cohere v2 / Runpod vLLM]
```

## Features

- Zero-idle GPU cost via Runpod Serverless (`/runsync`) and Cohere API v2 backends.
- Per-user conversational history, emotional arc, and key moment models backed by SQLite.
- Deterministic companion persona prompt formatting (`persona::format_prompt`).
- Pure Rust async HTTP gateway built on Tokio, Axum, and Reqwest.
- Detailed system design in [docs/architecture.md](docs/architecture.md).

## Configuration

Runtime configuration is loaded from environment variables via `Config::from_env()`.

| Variable | Required | Default | Description |
| :--- | :--- | :--- | :--- |
| `PORT` | No | `8080` | TCP port bound by the Axum HTTP server. |
| `DEFAULT_MODEL` | No | `command-r` | Model identifier sent in completion requests. |
| `COHERE_API_KEY` | Conditional | `None` | API token for the Cohere v2 chat adapter. |
| `RUNPOD_API_KEY` | Conditional | `None` | Bearer API key for Runpod Serverless requests. |
| `RUNPOD_ENDPOINT_ID` | Conditional | `None` | Target Runpod Serverless vLLM endpoint ID. |
| `RUST_LOG` | No | `info` | Structured log filter for `tracing-subscriber`. |

## HTTP Endpoints

| Method | Path | Purpose | Response |
| :--- | :--- | :--- | :--- |
| `GET` | `/health` | Service liveness check | `200 OK` JSON health payload |
| `POST` | `/webhook/telegram` | Telegram Update webhook | `200 OK` outgoing reply payload |

Example webhook check:

```bash
curl -s http://localhost:8080/health

curl -s -X POST http://localhost:8080/webhook/telegram \
  -H "Content-Type: application/json" \
  -d '{
    "update_id": 1001,
    "message": {
      "message_id": 42,
      "chat": { "id": 987654321 },
      "from": { "id": 987654321, "first_name": "Alex" },
      "text": "Hello from Telegram"
    }
  }'
```

## Build and Container Deployment

Run checks and unit tests locally with Cargo:

```bash
cargo check && cargo test
RUST_LOG=info PORT=8080 cargo run --release
```

Build and run the containerized release binary with Docker:

```bash
docker build -t serverless-llm-relay:latest .
docker run --rm -p 8080:8080 \
  -e DEFAULT_MODEL=command-r \
  -e COHERE_API_KEY="$COHERE_API_KEY" \
  serverless-llm-relay:latest
```
