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

## Memory Store and Companion Persona

`MemoryStore` wraps an embedded SQLite connection (`rusqlite` with `bundled` feature) and initializes the `history` table automatically:

```sql
CREATE TABLE IF NOT EXISTS history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL
);
```

- **User Isolation:** Every query scopes rows strictly by `user_id` (mapped from the webhook chat or user identifier).
- **Chronological Context Window:** `get_history(user_id, limit)` selects the most recent `limit` rows using `ORDER BY id DESC LIMIT ?2` and reverses the slice in memory so the LLM receives turns in chronological order.
- **Companion Metadata Models:** `EmotionalArc` (`start_tone`, `current_tone`, `intimacy_level`) and `KeyMoment` (`timestamp_epoch_s`, `summary`, `emotional_significance`) model longitudinal relationship state.
- **System Prompt Formatting:** `persona::format_prompt(persona, user_name)` combines the base persona template (`ALARA_DEFAULT_PERSONA` or custom override) with user-specific addressing directives as the leading `system` message.

## LLM Provider Adapters

All inference backends accept a normalized `ChatRequest` (`messages`, `model`, `temperature`, `max_tokens`) and return a `ChatResponse` (`content`, `model`).

### Cohere API v2 Adapter

- **Endpoint:** `POST https://api.cohere.com/v2/chat`
- **Authentication:** `Authorization: Bearer <COHERE_API_KEY>`
- **Mapping:** Serializes `ChatRequest` messages and generation parameters (`model`, `temperature`, `max_tokens`) and extracts the assistant text blocks from the v2 response message content array.

### Runpod Serverless vLLM Adapter

- **Endpoint:** `POST https://api.runpod.ai/v2/<RUNPOD_ENDPOINT_ID>/runsync`
- **Authentication:** `Authorization: Bearer <RUNPOD_API_KEY>`
- **Mapping:** Wraps OpenAI-compatible chat fields inside the Runpod `{ "input": { ... } }` envelope and parses generated output tokens from the synchronous worker response payload.
