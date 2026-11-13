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
