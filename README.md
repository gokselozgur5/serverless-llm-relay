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
