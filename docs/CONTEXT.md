# VoicePilot Context

## Product Summary

VoicePilot is a local-first Linux desktop assistant that captures spoken input, refines it with local inference, and types the final prompt into the active application.

## Architecture

- `src-tauri/`: Rust/Tauri desktop host, global hotkeys, audio capture, tray, config, typing
- `src/`: React/Vite settings and overlay UI
- `inference/`: Python FastAPI service for VAD, STT, and cleanup orchestration
- `.codex/`: Codex CLI project config and SAFe agent profiles
- `.agents/skills/`: project-local shared skill library
- `.sandcastle/`: sandbox automation entrypoint, env, prompt, and logs

## Development Baseline

- OS: Linux desktop
- Audio: PipeWire or PulseAudio
- Typing automation: `xdotool`
- Hotkeys: X11 or Wayland session with required permissions
- Rust checks should use `cargo check` for the normal dev loop

## Hardware Assumptions

- CPU: Intel i5-class or equivalent
- GPU: Intel Iris Xe-class integrated graphics or equivalent
- Memory: 20 GB RAM baseline

## Model Assumptions

- Speech-to-text: NVIDIA Parakeet 0.6B class model
- Voice activity detection: Silero VAD
- Prompt cleanup: Qwen 3 Instruct class model served locally through Ollama
- Python dependencies should remain CPU-safe by default and must not require CUDA unless explicitly enabled

## Safety Constraints

- Do not log raw audio, raw transcripts, or cleaned prompts by default
- Do not commit local models, secrets, `node_modules/`, `target/`, or `inference/.venv/`
- Missing `OPENAI_API_KEY` is a setup blocker for Sandcastle and API-key Codex flows

## Workflow Notes

- Use `gpt-5` for planning, architecture, and complex coding tasks
- Use `gpt-5-mini` for implementation and review tasks
- Keep Sandcastle logs under `.sandcastle/logs/` for recovery and handoff
