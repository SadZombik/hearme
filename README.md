# HEARME

`HEARME` is a local-first AI companion for emotional support. Users can choose between two personas — Anya or Arthur — and talk by text or voice in Russian or English, receiving calm, respectful, non-judgmental replies with a built-in crisis safety guard. HEARME is not a psychologist or emergency service; by default it runs locally with Ollama, with optional cloud providers via configuration.

## What Covered

- `Website/AI companion concept`: emotional-support web app with safe, respectful dialogue.
- `Clear UI + main screen`: HEARME welcome, short description, “How it works”, avatar selection, Start button.
- `Character choice`: girl/boy personas, distinct prompts and visuals.
- `Text chat`: streaming SSE replies, message order, pending state, error bubble.
- `Voice input/output`: browser Speech Recognition + Speech Synthesis, voice picker, mute toggle, stop button.
- `Session controls`: create session, clear history, switch companion.
- `AI integration`: local Ollama default, cloud OpenAI-compatible fallback, mock demo mode.
- `Server-side API keys`: cloud key stays in backend env, never in browser.
- `Supportive AI style`: calm, warm, no judgment, ≤1 question, small safe step, short replies.
- `No diagnosis/dangerous advice`: explicitly in system prompts (prompts/safety.*.txt).
- `Crisis reaction`: deterministic keyword guard + static RU/EN reply with 112/trusted person.
- `Honest AI limitation notice`: disclaimer in UI, cloud/local mode notices.
- `Guest mode`: no registration, minimal session data in memory.
- `Docker setup`: Dockerfile, Compose, Ollama service, env example.

## Partially covered

- `Voice selection`: picker and auto-best scoring exist, but no “listen to sample then confirm” screen; quality still depends on installed OS/browser voices.
- `Voice setting persistence`: stored only in JS memory per character, not in profile/localStorage.
- `Voice passed to AI`: character is passed into prompts; voice is browser-only and not sent to dialogue settings.
- `Languages`: RU + EN implemented; Kazakh from the dev plan example is not.
- `Privacy policy`: notices/disclaimer exist, but no separate privacy policy page or consent screen.
- `HTTPS/encryption`: local HTTP demo only; no production HTTPS, encryption at rest, or secure hosting configured.
- `Data retention`: manual history clear exists; no expiry/auto-deletion policy.
- `Settings page`: controls are in the topbar/chat, not a dedicated settings page.
- `Safety`: keyword crisis guard and prompt rules exist, but no local crisis resources, minors-specific flow, or systematic safety-dialogue tests.
- `Testing`: unit/smoke checks pass, but no cross-browser, device, slow-network, real-user, or security testing.
- `Deployment`: Compose works locally; no domain, hosting, backups, monitoring, or pilot feedback loop.

## Not covered

- `Mobile/adaptive layout and touch testing` — excluded by your instruction.
- `Practical self-help tools`: breathing, grounding, emotion journal, self-analysis questions.
- `Multiple appearance variants or extra characters` — only two fixed personas.
- `Retry/resend of failed messages` — errors are shown but not retryable.
- `Rate limiting / abuse protection`.
- `Accounts, access control, account deletion, staff-access controls`.
- `Kazakhstan legal compliance and minors-specific legal safeguards`.
- `Privacy policy document, user consent flow, retention rules`.
- `Systematic quality checks` for stress, loneliness, conflict, anxiety, study, and relationship scenarios.
- `Working hosted link/domain and access handover`.

Biggest remaining gaps for the docs’ MVP: privacy policy/consent, HTTPS/hosting, mobile layout, retry/rate limiting, and formal safety/quality test records.

## Tech stack

- `Backend`: Rust (edition 2024, MSRV 1.85), see `Cargo.toml`.
- `Frontend`: dependency-free static HTML/CSS/JS served by the backend. Chat uses Server-Sent Events; voice uses the browser Web Speech API with per-character voice selection.
- `AI`: local Ollama by default (qwen3:4b) through the OpenAI-compatible /v1/chat/completions API; optional cloud OpenAI-compatible provider (Gemini/Groq/OpenRouter) with keys stored server-side; mock provider for offline demos.
- `Safety`: Russian/English system prompts, separate personas for Anya and Arthur, and a deterministic crisis-keyword guard that bypasses the model with a 112/trusted-person fallback.
- `Deployment`: multi-stage Dockerfile, Docker Compose with app + Ollama + model setup, and .env.example for runtime configuration.

Most of this codebase was generated with the help of Big Pickle - OpenCode AI assistant, with the concept, requirements, UI direction, review, and decisions provided by the HEARME team.

## Deployment

### Required tools

- Docker Engine 24+ or Docker Desktop with Docker Compose v2 (docker compose). This is the only required runtime for the normal launch; no separate Rust, Ollama, Node, or Python installation is needed.
- A modern browser with microphone permission for voice input; Chrome, Edge, or Safari are recommended. Text chat works in any modern browser.
- 8 GB RAM recommended for the default qwen3:4b model, 16 GB for larger models, and about 6 GB free disk for Docker images and model weights.
- Internet access on the first launch to pull Docker images and the Ollama model. After the model is cached, local mode works offline.

### Launch

```
cp .env.example .env   # optional: edit provider/model/API key
docker compose up --build
```

Open http://localhost:3000.

The stack exposes:

- 3000 — HEARME web app
- 11434 — Ollama API

On the first run, the model-setup service pulls qwen3:4b; wait for it to finish before starting a chat:

```
docker compose logs -f model-setup
```

Stop with docker compose down. Reset the cached model volume with `docker compose down -v`.

### Cloud mode

Set in .env:

```
DEFAULT_PROVIDER=cloud
CLOUD_BASE_URL=https://generativelanguage.googleapis.com/v1beta/openai
CLOUD_MODEL=gemini-2.5-flash
CLOUD_API_KEY=your-key-here
```

The API key stays server-side and is never exposed to the browser.

### Local development without Docker

Optional: install Rust 1.85+ and Ollama, then:

```
ollama pull qwen3:4b
cargo run --release
```

The current Compose setup is intended for a local demo; production use would additionally need HTTPS, a domain, backups, monitoring, and a privacy policy page.
