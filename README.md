# serverless-llm-relay

High-throughput serverless LLM relay, companion memory, and webhook bridge.

## Architecture

```
[Client / Webhook] ---> [Relay Gateway] ---> [Memory Store (SQLite / Vector)]
                               |
                               v
                     [Serverless vLLM / Runpod]
```

## Features

- Zero-idle compute cost via serverless backends (Runpod / vLLM / Cohere).
- Stateful conversational context and user memory store.
- Asynchronous streaming relay with backpressure.
- Clean webhook adapters for Telegram, Discord, and HTTP clients.
