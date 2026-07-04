CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER DEFAULT (unixepoch())
);

CREATE TABLE IF NOT EXISTS hotkey (
    id INTEGER PRIMARY KEY,
    modifiers TEXT NOT NULL,
    key TEXT NOT NULL,
    mode TEXT DEFAULT 'push_to_talk'
);

CREATE TABLE IF NOT EXISTS audio (
    id INTEGER PRIMARY KEY,
    device_id TEXT,
    sample_rate INTEGER DEFAULT 16000,
    buffer_size INTEGER DEFAULT 512,
    vad_sensitivity REAL DEFAULT 0.5,
    agc_enabled INTEGER DEFAULT 1
);

CREATE TABLE IF NOT EXISTS models (
    id INTEGER PRIMARY KEY,
    stt_model TEXT DEFAULT 'nvidia/parakeet-tdt-0.6b',
    llm_model TEXT DEFAULT 'qwen3:8b',
    stt_quantization TEXT DEFAULT 'none',
    llm_temperature REAL DEFAULT 0.3
);
