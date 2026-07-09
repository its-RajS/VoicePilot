use crate::models::{
    error::VoicePilotError,
    ipc::{IpcRequest, IpcResponse},
};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;

pub struct IpcClient {
    inner: Mutex<IpcProcess>,
}

struct IpcProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_request_id: u64,
}

impl IpcClient {
    pub fn spawn() -> Result<Self, VoicePilotError> {
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

        let client = Self {
            inner: Mutex::new(IpcProcess {
                child,
                stdin,
                stdout: BufReader::new(stdout),
                next_request_id: 1,
            }),
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
            IpcResponse::HealthStatus { status, .. } => Ok(status),
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

    pub fn cleanup_transcript(&self, transcript: &str, mode: &str) -> Result<String, VoicePilotError> {
        let request_id = self.next_request_id()?;
        let response = self.send(IpcRequest::CleanupRequest {
            request_id,
            transcript: transcript.to_owned(),
            mode: mode.to_owned(),
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

        if let Some(status) = process
            .child
            .try_wait()
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?
        {
            return Err(VoicePilotError::InferenceUnavailable(format!(
                "inference bridge exited unexpectedly: {status}"
            )));
        }

        let payload = serde_json::to_string(&request)
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?;
        process
            .stdin
            .write_all(payload.as_bytes())
            .and_then(|_| process.stdin.write_all(b"\n"))
            .and_then(|_| process.stdin.flush())
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?;

        let mut line = String::new();
        let read = process
            .stdout
            .read_line(&mut line)
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?;

        if read == 0 {
            return Err(VoicePilotError::InferenceUnavailable(
                "inference bridge closed stdout".into(),
            ));
        }

        let response: IpcResponse = serde_json::from_str(line.trim())
            .map_err(|error| VoicePilotError::InferenceUnavailable(error.to_string()))?;

        if response.request_id() != request_id {
            return Err(VoicePilotError::InferenceUnavailable(format!(
                "mismatched response id: expected {request_id}, got {}",
                response.request_id()
            )));
        }

        Ok(response)
    }
}

impl Drop for IpcProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
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

    #[test]
    fn bridge_starts_and_answers_health() {
        let client = IpcClient::spawn().expect("bridge should start");
        let status = client.health_check().expect("health should respond");
        assert_eq!(status, "ok");
    }
}
