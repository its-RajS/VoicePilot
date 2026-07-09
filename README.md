# VoicePilot

Open-source, local-first voice-to-prompt assistant for Linux developers.

VoicePilot lets you hold a global hotkey, speak naturally, clean the transcript with a local LLM, and type the final prompt into the currently focused application.

## MVP scope

- Tauri v2 desktop shell
- React + TypeScript settings and overlay UI
- Rust services for hotkeys, audio capture, tray, config, and typing
- Python FastAPI inference service for VAD, STT, and prompt cleanup orchestration
- Local-first defaults: NVIDIA Parakeet 0.6B STT, Silero VAD, Ollama-hosted Qwen 3 Instruct for cleanup

## Project layout

```text
src-tauri/   Rust/Tauri desktop layer
src/         React frontend
inference/   Python AI backend
config/      SQLite schema and default settings
scripts/     install/build/dev helpers
docs/        architecture and development notes
```

## Development baseline

- Linux desktop with PipeWire or PulseAudio
- `xdotool` available for typing simulation
- `pkg-config` and Linux desktop development headers installed for Tauri/Rust checks
- Target hardware baseline: Intel i5-class CPU, Iris Xe-class integrated graphics, 20 GB RAM

## Development bootstrap

```bash
./scripts/dev-setup.sh
npm install
cd inference && python -m venv .venv && . .venv/bin/activate && pip install -e .
cd ..
npm run tauri:dev
```

## First milestone

Phase 0 targets project scaffolding, CI, logging, config storage, a basic Tauri window/tray, a Python health endpoint, and Rust-to-Python connectivity.

See `docs/development.md` for the next implementation steps.
