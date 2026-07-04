# Architecture

VoicePilot uses a hybrid local-first architecture:

- Tauri + Rust for desktop shell, hotkeys, audio capture, config, typing, and tray integration.
- React + TypeScript for settings, overlay, first-run wizard, and status display.
- Python + FastAPI for the AI inference layer.
- Ollama for local prompt cleanup.

No audio, transcript, or prompt content should be logged or sent to remote services by default.
