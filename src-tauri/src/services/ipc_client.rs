use crate::models::{
    error::VoicePilotError,
    ipc::{IpcRequest, IpcResponse},
};
use std::env;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;

const MAX_FRAME_LEN: u32 = 64 * 1024 * 1024;

const PROTOCOL_VERSION: u32 = 1;

pub struct IpcClient {
    inner: Mutex<IpcProcess>,
}

struct IpcProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
    next_request_id: u64,
}

impl IpcClient {
    pub fn spawn() -> Result<Self, VoicePilotError> {
        let process = spawn_process()?;
        let client = Self {
            inner: Mutex::new(process),
        };

        let status = client.health_check()?;
        if status != "ok" {
            return Err(VoicePilotError::InferenceUnavailable(format!(
                "unexpected inference health status: {status}"
            )));
        }

        Ok(client)
    }

    pub fn health_check(&self) -> Result<String, VoicePilotError> {
        let request_id = self.next_request_id()?;
        let response = self.send(IpcRequest::HealthCheck { request_id })?;

        match response {
            IpcResponse::HealthStatus {
                status,
                protocol_version,
                ..
            } => {
                if protocol_version != PROTOCOL_VERSION {
                    return Err(VoicePilotError::InferenceUnavailable(format!(
                        "protocol version mismatch: rust={PROTOCOL_VERSION}, python={protocol_version}"
                    )));
                }
                Ok(status)
            }
            IpcResponse::Error { message, .. } => {
                Err(VoicePilotError::InferenceUnavailable(message))
            }
            other => Err(VoicePilotError::InferenceUnavailable(format!(
                "unexpected health response: {other:?}"
            ))),
        }
    }

    pub fn start_recording(&self, sample_rate_hz: u32) -> Result<(), VoicePilotError> {
        let request_id = self.next_request_id()?;
        let response = self.send(IpcRequest::StartRecording {
            request_id,
            sample_rate_hz,
        })?;

        match response {
            IpcResponse::RecordingStarted { .. } => Ok(()),
            IpcResponse::Error { message, .. } => Err(VoicePilotError::InferenceUnavailable(message)),
            other => Err(VoicePilotError::InferenceUnavailable(format!(
                "unexpected recording response: {other:?}"
            ))),
        }
    }

    pub fn send_audio_chunk(&self, pcm_s16le: Vec<u8>) -> Result<Option<String>, VoicePilotError> {
        let request_id = self.next_request_id()?;
        let response = self.send(IpcRequest::AudioChunk {
            request_id,
            pcm_s16le,
        })?;

        match response {
            IpcResponse::PartialTranscript { text, .. } => Ok(Some(text)),
            IpcResponse::FinalTranscript { text, .. } => Ok(Some(text)),
            IpcResponse::Error { message, .. } => Err(VoicePilotError::InferenceUnavailable(message)),
            other => Err(VoicePilotError::InferenceUnavailable(format!(
                "unexpected audio response: {other:?}"
            ))),
        }
    }

    pub fn cleanup_transcript(
        &self,
        transcript: &str,
        mode: &str,
        model: &str,
    ) -> Result<String, VoicePilotError> {
        let request_id = self.next_request_id()?;
        let response = self.send(IpcRequest::CleanupRequest {
            request_id,
            transcript: transcript.to_owned(),
            mode: mode.to_owned(),
            model: model.to_owned(),
        })?;

        match response {
            IpcResponse::CleanupResponse { cleaned_prompt, .. } => Ok(cleaned_prompt),
            IpcResponse::Error { message, .. } => Err(VoicePilotError::InferenceUnavailable(message)),
            other => Err(VoicePilotError::InferenceUnavailable(format!(
                "unexpected cleanup response: {other:?}"
            ))),
        }
    }

    fn next_request_id(&self) -> Result<u64, VoicePilotError> {
        let mut process = self
            .inner
            .lock()
            .map_err(|_| VoicePilotError::InferenceUnavailable("ipc client lock poisoned".into()))?;
        let request_id = process.next_request_id;
        process.next_request_id += 1;
        Ok(request_id)
    }

    fn send(&self, request: IpcRequest) -> Result<IpcResponse, VoicePilotError> {
        let request_id = request.request_id();
        let mut process = self
            .inner
            .lock()
            .map_err(|_| VoicePilotError::InferenceUnavailable("ipc client lock poisoned".into()))?;

        if process
            .child
            .try_wait()
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?
            .is_some()
        {
            // ponytail: one restart attempt per crash, no backoff/retry budget.
            // Add exponential backoff + crash-loop detection if restarts get noisy.
            *process = spawn_process()?;
        }

        let payload = serde_json::to_vec(&request)
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?;
        write_frame(&mut process.stdin, &payload)
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?;

        let frame = read_frame(&mut process.stdout)
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?;

        let response: IpcResponse = serde_json::from_slice(&frame)
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?;

        if response.request_id() != request_id {
            return Err(VoicePilotError::InferenceUnavailable(format!(
                "mismatched response id: expected {request_id}, got {}",
                response.request_id()
            )));
        }

        Ok(response)
    }

    #[cfg(test)]
    fn kill_for_test(&self) {
        let mut process = self.inner.lock().expect("lock should not be poisoned");
        let _ = process.child.kill();
        let _ = process.child.wait();
    }

    #[cfg(test)]
    fn child_pid(&self) -> u32 {
        self.inner.lock().expect("lock should not be poisoned").child.id()
    }
}

impl Drop for IpcProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn spawn_process() -> Result<IpcProcess, VoicePilotError> {
    let repo_root = repository_root();
    let python = python_executable(&repo_root);
    let python_path = repo_root.join("inference/src");

    let mut child = Command::new(&python)
        .arg("-m")
        .arg("voicepilot_inference.ipc_server")
        .current_dir(&repo_root)
        .env("PYTHONPATH", merged_python_path(&python_path))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?;

    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| VoicePilotError::InferenceUnavailable("missing child stdin".into()))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| VoicePilotError::InferenceUnavailable("missing child stdout".into()))?;

    Ok(IpcProcess {
        child,
        stdin,
        stdout,
        next_request_id: 1,
    })
}

fn write_frame(stdin: &mut ChildStdin, payload: &[u8]) -> std::io::Result<()> {
    let len = u32::try_from(payload.len()).expect("ipc payload should fit u32 length prefix");
    stdin.write_all(&len.to_be_bytes())?;
    stdin.write_all(payload)?;
    stdin.flush()
}

fn read_frame(stdout: &mut ChildStdout) -> std::io::Result<Vec<u8>> {
    let mut len_bytes = [0u8; 4];
    stdout.read_exact(&mut len_bytes)?;
    let len = u32::from_be_bytes(len_bytes);
    if len > MAX_FRAME_LEN {
        return Err(std::io::Error::other(format!(
            "ipc frame length {len} exceeds max {MAX_FRAME_LEN}"
        )));
    }
    let mut payload = vec![0u8; len as usize];
    stdout.read_exact(&mut payload)?;
    Ok(payload)
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("src-tauri should have a repo root")
        .to_path_buf()
}

fn python_executable(repo_root: &Path) -> PathBuf {
    let candidates = [
        repo_root.join("inference/.venv/bin/python3"),
        repo_root.join("inference/.venv/bin/python"),
        PathBuf::from("python3"),
    ];

    candidates
        .into_iter()
        .find(|candidate| candidate.is_absolute() || command_exists(candidate))
        .unwrap_or_else(|| PathBuf::from("python3"))
}

fn command_exists(command: &Path) -> bool {
    if command.components().count() > 1 {
        return command.exists();
    }

    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).any(|path| path.join(command).exists()))
        .unwrap_or(false)
}

fn merged_python_path(extra_path: &Path) -> String {
    match env::var_os("PYTHONPATH") {
        Some(existing) if !existing.is_empty() => {
            let mut paths = vec![extra_path.to_path_buf()];
            paths.extend(env::split_paths(&existing));
            env::join_paths(paths)
                .expect("python path join should succeed")
                .to_string_lossy()
                .into_owned()
        }
        _ => extra_path.to_string_lossy().into_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::IpcClient;
    use std::time::Instant;

    #[test]
    fn bridge_starts_and_answers_health() {
        let client = IpcClient::spawn().expect("bridge should start");
        let status = client.health_check().expect("health should respond");
        assert_eq!(status, "ok");
    }

    #[test]
    fn audio_chunk_round_trip_under_50ms() {
        let client = IpcClient::spawn().expect("bridge should start");
        let pcm = vec![0u8; 3200]; // 100ms of 16kHz mono 16-bit silence

        let start = Instant::now();
        let text = client
            .send_audio_chunk(pcm)
            .expect("audio chunk should round-trip");
        let elapsed = start.elapsed();

        assert!(text.is_some());
        assert!(
            elapsed.as_millis() < 50,
            "audio chunk round trip took {elapsed:?}"
        );
    }

    #[test]
    fn drop_kills_child_process() {
        let client = IpcClient::spawn().expect("bridge should start");
        let pid = client.child_pid();
        drop(client);

        assert!(
            !std::path::Path::new(&format!("/proc/{pid}")).exists(),
            "child process {pid} should be reaped after Drop"
        );
    }

    #[test]
    fn bridge_restarts_after_crash() {
        let client = IpcClient::spawn().expect("bridge should start");
        client.kill_for_test();

        let status = client
            .health_check()
            .expect("client should transparently restart the bridge");
        assert_eq!(status, "ok");
    }
}
