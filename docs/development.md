# Development Plan

Phase 1: Foundation & Infrastructure
These must be completed first. Everything else depends on them.
VOICE-001: Project Bootstrap & Build System
Initialize Tauri v2 project with React frontend
Set up Rust workspace in src-tauri/
Set up Python virtual environment in python/
Create Cargo.toml, package.json, and pyproject.toml/requirements.txt
Configure cross-compilation targets (Linux x64)
AC: cargo check, npm install, and pip install all succeed from clean clone
VOICE-002: SQLite Configuration Schema
Create config.db with tables: settings, hotkeys, models, history
Schema must store: global hotkey combo, selected ASR model path, selected LLM model, typing mode (X11/Wayland), audio device ID
AC: Config persists across app restarts; schema is versioned for migrations
VOICE-003: Rust ↔ Python IPC Bridge
Implement IPC channel (Unix domain sockets or stdio pipes)
Define protobuf/JSON message protocol: StartRecording, AudioChunk, PartialTranscript, FinalTranscript, CleanupRequest, CleanupResponse, Error
AC: Rust can send audio bytes to Python and receive transcript strings with <50ms latency
VOICE-004: Ollama Integration & Local LLM Discovery
Auto-detect Ollama installation (ollama --version)
List available local models via Ollama API (/api/tags)
Pull Qwen 3 Instruct if not present (with user confirmation)
AC: App can query Ollama status and model list at startup

Phase 2: Audio Pipeline
The core input system. Must be rock-solid before AI layers.
VOICE-005: Audio Capture Engine (Rust)
Capture microphone input via cpal or rodio
Support configurable audio device selection
Stream raw PCM audio to the IPC bridge
AC: 16kHz mono 16-bit PCM; <50ms buffer latency; no dropouts on reference hardware
VOICE-006: Voice Activity Detection (Silero VAD)
Integrate Silero VAD in the Python inference server
Detect speech start/stop with configurable sensitivity
Emit SpeechStart and SpeechEnd events to Rust
AC: <100ms detection latency; filters out keyboard typing noise and background hum
VOICE-007: Push-to-Talk Mode
Global hotkey press starts listening
Hotkey release stops listening
Visual indicator (system tray icon change or overlay) showing "Recording" state
AC: Hotkey response <50ms; indicator appears within 100ms of press
VOICE-008: Live Transcript Preview
Stream partial transcripts from ASR back to React UI in real-time
Display text in a floating/transient overlay or in-app panel
AC: Partial updates render within 300ms of speech; UI updates at 15fps minimum

Phase 3: Speech Recognition & AI
The brain of the system.
VOICE-009: Local ASR Integration (Parakeet TDT 0.6B)
Load Parakeet TDT 0.6B via ONNX Runtime or Transformers
Process audio chunks from VAD
Emit partial transcripts during speech
Emit final transcript after speech ends
AC: Final transcript available <1s after speech stops; WER <15% on technical vocabulary
VOICE-010: Automatic Punctuation
Post-process ASR output with punctuation restoration
Can be rule-based or a lightweight model
AC: Sentence boundaries are correctly placed; commas appear in lists
VOICE-011: Prompt Cleanup via Local LLM
Send raw transcript to Ollama (Qwen 3 Instruct)
System prompt: "Rewrite this spoken developer dictation into a clean, concise engineering prompt. Fix filler words, expand abbreviations, structure logically."
AC: Cleanup completes <2s on reference hardware; output is coherent and technically accurate
VOICE-012: Model Selection & Management
UI to select ASR model (dropdown with discovered models)
UI to select LLM model (Ollama model list)
UI to set model paths for custom/local weights
AC: Model change takes effect without app restart; invalid paths show error

Phase 4: Output & Integration
Getting the text into the user's active application.
VOICE-013: X11 Typing Simulation (xdotool)
Type final cleaned prompt into focused X11 window
Handle Unicode and special characters
AC: Text appears in target app within 100ms; no garbled characters
VOICE-014: Wayland Typing Simulation (ydotool)
Type final cleaned prompt into focused Wayland window
Graceful fallback if ydotool daemon is not running
AC: Same performance as X11; auto-detect display server at runtime
VOICE-015: Auto-Type vs. Clipboard Mode
Default: auto-type into focused window
Optional: copy to clipboard instead (user-configurable)
AC: Toggle in settings; clipboard mode uses wl-copy/xclip appropriately

Phase 5: UI & Configuration
The user-facing shell.
VOICE-016: System Tray Application
Minimize to system tray on startup
Tray icon shows state: Idle, Listening, Processing
Tray menu: Show/Hide, Settings, Quit
AC: App runs without main window; tray icon visible on Ubuntu/GNOME and KDE
VOICE-017: Global Hotkey Registration
Register system-wide hotkey (default: Ctrl+Space)
Configurable via settings UI
Handle conflicts gracefully (warn user if combo is taken)
AC: Hotkey works when app is not focused; persists across reboots
VOICE-018: Settings / Configuration Screen
React UI panel with tabs: General, Audio, Models, Hotkey, About
General: startup behavior, theme
Audio: input device, VAD sensitivity
Models: ASR model path, LLM model selection, Ollama URL
Hotkey: key combo recorder
AC: All settings save to SQLite; changes apply immediately where possible
VOICE-019: Model Download / Status UI
Show download progress for Parakeet weights (if bundled downloader)
Show Ollama model pull status
Disk usage indicator for model files
AC: User can see which models are ready and which need downloading

Phase 6: Polish & Release
Required for a shippable MVP.
VOICE-020: End-to-End Integration Test
Simulate full flow: hotkey → speak → transcript → cleanup → type
Test with sample audio files
AC: Complete E2E test passes in CI; <3s total pipeline time
VOICE-021: Error Handling & Recovery
Handle: microphone disconnected, model load failure, Ollama not running, IPC crash
Show user-friendly error toasts/notifications
Auto-recovery where possible (e.g., restart Python server)
AC: No silent failures; every error has a visible message and recovery action
VOICE-022: Logging & Diagnostics
Structured logging in Rust (tracing) and Python (structlog or standard logging)
Log levels: ERROR, WARN, INFO, DEBUG
Log file rotation (max 10MB, 3 backups)
AC: Logs written to ~/.local/share/voicepilot/logs/; DEBUG mode toggle in settings
VOICE-023: Packaging & Installation
.deb package for Ubuntu/Debian
.AppImage for universal Linux
Installation script (install.sh) for manual setup
AC: Clean install on fresh Ubuntu 24.04; uninstall removes all files except models and logs
