# Architecture Guide

`serverless-llm-relay` bridges stateful messaging webhooks with stateless serverless LLM inference backends in pure Rust.

## Design Goals

1. **Zero-Idle Compute Cost:** Offload GPU inference to on-demand serverless endpoints (Runpod Serverless vLLM or Cohere v2) while keeping the relay gateway lightweight.
2. **Low-Latency State Assembly:** Maintain per-user conversation history in embedded SQLite (`rusqlite` bundled) without external database round-trips.
3. **Provider Isolation:** Decouple transport webhooks and prompt assembly from upstream LLM HTTP schemas through clean request/response types.

## End-to-End Request Lifecycle

```text
Telegram Update JSON
  --> POST /webhook/telegram (Axum Router)
  --> Telegram Update Parser (extract chat_id, sender name, text)
  --> RelayPipeline::process
        1. Fetch ordered user history from MemoryStore (SQLite)
        2. Build system prompt via persona::format_prompt
        3. Assemble ChatRequest [system, ...history, user]
        4. Dispatch async completion to LLM Provider (Cohere v2 / Runpod)
        5. Persist user turn and assistant reply in MemoryStore
  --> Format outgoing Telegram message payload (chat_id, text)
```

## Module Architecture

| Module | Path | Responsibility |
| :--- | :--- | :--- |
| `config` | `src/config.rs` | Parses `PORT`, `DEFAULT_MODEL`, and provider credentials from environment variables. |
| `provider` | `src/provider.rs` | Defines `ChatMessage`, `ChatRequest`, `ChatResponse`, and provider traits/adapters. |
| `cohere` | `src/cohere.rs` | Sends chat completion requests to Cohere API v2 (`/v2/chat`). |
| `runpod` | `src/runpod.rs` | Dispatches synchronous vLLM jobs to Runpod Serverless (`/v2/{id}/runsync`). |
| `memory` | `src/memory/` | Manages SQLite conversation persistence (`MemoryStore`), `EmotionalArc`, and `KeyMoment`. |
| `persona` | `src/persona.rs` | Renders companion system prompts personalized with the active user name. |
| `telegram` | `src/telegram.rs` | Parses inbound Telegram `Update` payloads and formats outbound `sendMessage` bodies. |
| `pipeline` | `src/pipeline.rs` | Orchestrates memory lookup, persona injection, LLM invocation, and history persistence. |
| `gateway` | `src/gateway.rs` | Exposes Axum HTTP routes (`GET /health` and `POST /webhook/telegram`). |
