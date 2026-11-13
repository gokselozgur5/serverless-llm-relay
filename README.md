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

- Zero-idle compute cost via serverless backends (Runpod / vLLM / Cohere).
- Stateful conversational context and user memory store.
- Asynchronous streaming relay with backpressure.
- Clean webhook adapters for Telegram, Discord, and HTTP clients.
