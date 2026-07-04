# Development Plan

## Phase 0

1. Run the scaffold with `npm run tauri:dev`.
2. Implement SQLite-backed config service.
3. Start the Python FastAPI service from the Tauri lifecycle.
4. Add a health check from Rust to Python.
5. Add CI for Rust, TypeScript, and Python lint/test.

## Phase 1

1. Implement global hotkey service.
2. Implement audio capture and stream chunks to Python.
3. Add VAD, STT, and Ollama cleanup adapters.
4. Send transcript events to the React overlay.
5. Implement X11 typing via xdotool.
