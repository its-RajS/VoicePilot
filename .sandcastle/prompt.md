# Context

VoicePilot is a local-first Linux desktop voice-to-prompt assistant.

- Desktop shell: Tauri v2 + Rust in `src-tauri/`
- UI: React + Vite in `src/`
- Inference service: FastAPI in `inference/`
- Local-first defaults: NVIDIA Parakeet STT, Silero VAD, Ollama-hosted Qwen cleanup
- Safety rule: never log raw audio, raw transcripts, or cleaned prompts by default
- Target platform: Linux desktop with PipeWire or PulseAudio, `xdotool`, and global hotkey support
- Target hardware baseline: Intel i5-class CPU, Iris Xe-class integrated graphics, 20 GB RAM

Useful context commands:

- `!\`git status --short\``
- `!\`git branch --show-current\``
- `!\`cargo check --manifest-path src-tauri/Cargo.toml\``
- `!\`npm run build\``
- `!\`python3 -m pytest inference/tests -q\``

# Task

Use this sandbox to validate and automate VoicePilot development workflows.

- Prefer surgical fixes over broad refactors.
- Confirm assumptions against files already present in the repo.
- Preserve evidence in `.sandcastle/logs/` when a run fails or needs handoff.
- If a required secret such as `OPENAI_API_KEY` is missing, stop and report the blocker.

# Done

When the task is complete, output `<promise>COMPLETE</promise>`.
