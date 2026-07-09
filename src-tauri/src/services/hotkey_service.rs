use crate::models::config::AppConfig;
use crate::services::audio_service::{AudioService, RecordingSession};
use crate::services::ipc_client::IpcClient;
use rdev::{Event, EventType, Key};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

/// System-prompt mode used when cleaning up a hotkey-driven transcript. The
/// inference bridge maps this to the engineering prompt family.
const CLEANUP_MODE: &str = "engineering";

pub struct HotkeyService;

impl HotkeyService {
    /// Start the global hotkey listener on its own thread.
    ///
    /// Fails gracefully: on Linux, `rdev::listen` needs read access to
    /// `/dev/input/event*`. If the listener can't start, the app keeps running
    /// (Ollama panel, playground, etc. still work) and a `voicepilot:error`
    /// event is emitted with setup guidance.
    pub fn start(app: AppHandle, ipc: Arc<IpcClient>, config: AppConfig) {
        let state = Arc::new(HotkeyState {
            modifiers: parse_modifiers(&config.hotkey.modifiers),
            trigger: parse_key(&config.hotkey.key),
            session: Mutex::new(None),
            held_modifiers: Mutex::new(Vec::new()),
            audio: AudioService::new(ipc.clone()),
            ipc,
            app: app.clone(),
            typing_mode: config.typing.mode.clone(),
            speed_cps: config.typing.speed_cps,
            llm_model: config.models.llm_model.clone(),
        });

        if state.modifiers.is_empty() {
            // Without a modifier we can't safely detect the combo; a bare-key
            // trigger would fire on every ordinary keypress. Bail with guidance.
            let _ = state.app.emit(
                "voicepilot:error",
                "hotkey has no modifiers configured; refusing to bind a bare key globally",
            );
            return;
        }

        let state_for_cb = state.clone();
        // ponytail: one long-lived listener thread, no restart/backoff. If rdev
        // errors, emit guidance and let the app run without hotkeys until restart.
        if std::thread::Builder::new()
            .name("voicepilot-hotkey".into())
            .spawn(move || {
                if let Err(e) = rdev::listen(move |event| handle_event(event, &state_for_cb)) {
                    let msg = format!(
                        "hotkey listener failed: {e:?}. On Linux, rdev needs read access to \
                         /dev/input/event* — add your user to the 'input' group \
                         (`sudo usermod -aG input $USER`, then re-login) or run with permissions."
                    );
                    eprintln!("{msg}");
                    let _ = state.app.emit("voicepilot:error", &msg);
                }
            })
            .is_err()
        {
            eprintln!("failed to spawn hotkey listener thread");
        }
    }
}

struct HotkeyState {
    modifiers: Vec<Key>,
    trigger: Key,
    session: Mutex<Option<RecordingSession>>,
    held_modifiers: Mutex<Vec<Key>>,
    audio: AudioService,
    ipc: Arc<IpcClient>,
    app: AppHandle,
    typing_mode: String,
    speed_cps: u32,
    llm_model: String,
}

fn handle_event(event: Event, state: &Arc<HotkeyState>) {
    let (key, pressed) = match event.event_type {
        EventType::KeyPress(k) => (k, true),
        EventType::KeyRelease(k) => (k, false),
        _ => return,
    };

    // Track held modifiers so the combo is only considered active when all are down.
    if state.modifiers.contains(&key) {
        let mut held = state.held_modifiers.lock().expect("held_modifiers lock");
        if pressed {
            // Ignore autorepeat: a held modifier keeps firing KeyPress events.
            if !held.contains(&key) {
                held.push(key);
            }
        } else {
            held.retain(|k| *k != key);
        }
        return;
    }

    if key != state.trigger {
        return;
    }

    let combo_held = {
        let held = state.held_modifiers.lock().expect("held_modifiers lock");
        state.modifiers.iter().all(|m| held.contains(m))
    };

    if pressed && combo_held {
        // Push-to-talk: start a capture session (ignore autorepeat repeats).
        let mut slot = state.session.lock().expect("session lock");
        if slot.is_none() {
            match state.audio.start_recording() {
                Ok(session) => {
                    *slot = Some(session);
                    let _ = state.app.emit("voicepilot:status", "listening");
                }
                Err(e) => {
                    eprintln!("start recording failed: {e}");
                    let _ = state.app.emit("voicepilot:error", &e);
                }
            }
        }
    } else if !pressed {
        // Trigger released: stop capture, then run cleanup + type off the
        // listener thread so global input stays responsive.
        let session = state.session.lock().expect("session lock").take();
        if let Some(session) = session {
            let _ = state.app.emit("voicepilot:status", "processing");
            let state_arc = state.clone();
            std::thread::spawn(move || finish_session(session, &state_arc));
        }
    }
}

fn finish_session(session: RecordingSession, state: &Arc<HotkeyState>) {
    let transcript = match session.stop() {
        Ok(t) => t,
        Err(e) => {
            let _ = state.app.emit("voicepilot:error", &e);
            let _ = state.app.emit("voicepilot:status", "idle");
            return;
        }
    };
    let _ = state.app.emit("voicepilot:final", &transcript);

    if transcript.trim().is_empty() {
        let _ = state.app.emit("voicepilot:status", "idle");
        return;
    }

    match state
        .ipc
        .cleanup_transcript(&transcript, CLEANUP_MODE, &state.llm_model)
    {
        Ok(cleaned) => {
            let _ = state.app.emit("voicepilot:cleaned", &cleaned);
            let _ = state.app.emit("voicepilot:status", "typing");
            if let Err(e) = type_text(&cleaned, &state.typing_mode, state.speed_cps) {
                eprintln!("type failed: {e}");
                let _ = state.app.emit("voicepilot:error", &e);
            }
            let _ = state.app.emit("voicepilot:status", "complete");
        }
        Err(e) => {
            let msg = e.to_string();
            let _ = state.app.emit("voicepilot:error", &msg);
            let _ = state.app.emit("voicepilot:status", "idle");
        }
    }
}

/// Type the cleaned prompt into the currently focused window.
///
/// Only `auto_type` (xdotool) is implemented. `clipboard` mode is a future
/// boundary; until then it falls back to xdotool so the flow still completes.
fn type_text(text: &str, mode: &str, speed_cps: u32) -> Result<(), String> {
    let _ = mode;
    // chars/sec → ms between keystrokes. 0 means "as fast as possible".
    let delay_ms = if speed_cps > 0 { 1000 / speed_cps } else { 0 };
    let status = Command::new("xdotool")
        .arg("type")
        .arg("--clearmodifiers")
        .arg("--delay")
        .arg(delay_ms.to_string())
        .arg(text)
        .stdin(Stdio::null())
        .status()
        .map_err(|e| format!("xdotool type failed to start: {e}"))?;
    if !status.success() {
        return Err(format!("xdotool type exited with {status}"));
    }
    Ok(())
}

fn parse_modifiers(modifiers: &[String]) -> Vec<Key> {
    modifiers
        .iter()
        .filter_map(|m| match m.to_ascii_lowercase().as_str() {
            "ctrl" | "control" => Some(Key::ControlLeft),
            "shift" => Some(Key::ShiftLeft),
            "alt" | "option" => Some(Key::Alt),
            "super" | "meta" | "cmd" | "win" => Some(Key::MetaLeft),
            _ => None,
        })
        .collect()
}

fn parse_key(key: &str) -> Key {
    match key {
        "Space" => Key::Space,
        "Return" | "Enter" => Key::Return,
        "Tab" => Key::Tab,
        "Escape" => Key::Escape,
        single if single.len() == 1 => {
            let c = single.chars().next().unwrap().to_ascii_uppercase();
            match c {
                'A' => Key::KeyA,
                'B' => Key::KeyB,
                'C' => Key::KeyC,
                'D' => Key::KeyD,
                'E' => Key::KeyE,
                'F' => Key::KeyF,
                'G' => Key::KeyG,
                'H' => Key::KeyH,
                'I' => Key::KeyI,
                'J' => Key::KeyJ,
                'K' => Key::KeyK,
                'L' => Key::KeyL,
                'M' => Key::KeyM,
                'N' => Key::KeyN,
                'O' => Key::KeyO,
                'P' => Key::KeyP,
                'Q' => Key::KeyQ,
                'R' => Key::KeyR,
                'S' => Key::KeyS,
                'T' => Key::KeyT,
                'U' => Key::KeyU,
                'V' => Key::KeyV,
                'W' => Key::KeyW,
                'X' => Key::KeyX,
                'Y' => Key::KeyY,
                'Z' => Key::KeyZ,
                _ => Key::Unknown(c as u32),
            }
        }
        _ => Key::Unknown(0),
    }
}
