# VoicePilot Agent Instructions

## Purpose

VoicePilot is a local-first Linux desktop assistant that captures speech, cleans it with local models, and types the result into the active application.

## Working Rules

- Prefer the project-local harness under `.codex/`, `.agents/`, and `.sandcastle/`.
- Treat `.agents/skills/` as the repo's canonical project skill set when overlap exists with home-directory skills.
- Keep changes small and focused.
- Do not log raw audio, raw transcripts, or cleaned prompts by default.
- Do not commit local models, virtual environments, Node dependencies, secrets, or generated logs.

## Safety Gates

- Stop and surface blockers before feature work when setup or environment checks fail.
- Prefer `cargo check` over full Rust builds during development.
- Keep Sandcastle session artifacts under `.sandcastle/logs/` for resumable work.
- Use `chatgpt-5.4` for architecture, planning, and complex coding tasks.
- Use `chatgpt-5.3` for implementation, review, and general workflow tasks.

## Key Paths

- `src-tauri/` Rust/Tauri desktop runtime
- `src/` React/Vite frontend
- `inference/` Python inference service
- `.codex/` Codex CLI config and agent profiles
- `.agents/skills/` project skill library
- `.sandcastle/` sandbox orchestration files

## Environment

- Required secret: `OPENAI_API_KEY`
- Optional workflow secrets: `LINEAR_API_KEY`, `ATLASSIAN_API_TOKEN`, `ATLASSIAN_EMAIL`, `CONFLUENCE_BASE_URL`

## Validation Expectations

- Sandcastle prompt and env must be project-specific and present.
- Root `.gitignore` must exclude `models/`, `target/`, `node_modules/`, `.env`, `.sandcastle/.env`, and `inference/.venv/`.
- Document hardware assumptions and Linux desktop prerequisites in `docs/CONTEXT.md`.

## Agent skills

### Issue tracker

Not configured — no tracker in use. `to-issues`, `triage`, `to-prd`, and `qa` skills are unconfigured until this changes.

### Domain docs

Single-context: `docs/CONTEXT.md` (not repo root), no `docs/adr/` yet. See `docs/agents/domain.md`.
