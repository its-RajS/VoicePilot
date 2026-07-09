use crate::services::ipc_client::IpcClient;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Send a chunk to the inference bridge roughly every this many milliseconds.
const CHUNK_MS: usize = 100;

/// Captures microphone audio and forwards S16LE PCM chunks to the inference
/// bridge. Each [`RecordingSession`] owns one short-lived capture thread so the
/// blocking cpal/IPC work never touches the Tauri async runtime or the hotkey
/// listener thread.
pub struct AudioService {
    ipc: Arc<IpcClient>,
}

/// A live capture session. Drop by calling [`RecordingSession::stop`], which
/// joins the capture thread and returns whatever partial transcript the
/// bridge produced.
pub struct RecordingSession {
    stop_flag: Arc<AtomicBool>,
    handle: Option<JoinHandle<Result<String, String>>>,
}

impl AudioService {
    pub fn new(ipc: Arc<IpcClient>) -> Self {
        Self { ipc }
    }

    /// Begin capturing from the default input device. Fails fast (before
    /// spawning a thread) if no microphone or readable config is available.
    pub fn start_recording(&self) -> Result<RecordingSession, String> {
        Self::check_device()?;

        let ipc = self.ipc.clone();
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_for_thread = stop_flag.clone();

        let handle = thread::Builder::new()
            .name("voicepilot-audio".into())
            .spawn(move || -> Result<String, String> {
                // ponytail: open the device on THIS thread. cpal's ALSA Device is
                // !Send (it holds raw snd_pcm handles), so it must not cross threads.
                let host = cpal::default_host();
                let device = host
                    .default_input_device()
                    .ok_or_else(|| "no microphone input device available".to_string())?;
                let supported = device
                    .default_input_config()
                    .map_err(|e| format!("default input config error: {e}"))?;
                let sample_rate = supported.sample_rate().0;
                let channels = supported.channels();
                let stream_config = supported.config();
                let format = supported.sample_format();
                let step = channels.max(1) as usize;

                ipc.start_recording(sample_rate).map_err(|e| e.to_string())?;

                let (chunk_tx, chunk_rx) = mpsc::sync_channel::<Vec<i16>>(64);
                let stream = open_stream(&device, &stream_config, format, step, chunk_tx)?;
                stream.play().map_err(|e| format!("stream play: {e}"))?;

                let target_frames = ((sample_rate as usize) * CHUNK_MS / 1000).max(1);
                let mut buffer: Vec<i16> = Vec::with_capacity(target_frames * 2);
                let mut transcript = String::new();

                while !stop_for_thread.load(Ordering::Relaxed) {
                    match chunk_rx.recv_timeout(Duration::from_millis(50)) {
                        Ok(samples) => {
                            buffer.extend(samples);
                            while buffer.len() >= target_frames {
                                let chunk: Vec<i16> = buffer.drain(..target_frames).collect();
                                let bytes = encode_s16le(&chunk);
                                // ponytail: the inference bridge echoes a partial
                                // transcript per chunk today (real STT is a later
                                // ticket). Until then, the latest non-empty partial
                                // wins — echoes carry byte counts, not speech.
                                if let Ok(Some(text)) = ipc.send_audio_chunk(bytes) {
                                    if !text.is_empty() {
                                        transcript = text;
                                    }
                                }
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => continue,
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                }

                drop(stream);
                Ok(transcript)
            })
            .map_err(|e| format!("spawn audio thread: {e}"))?;

        Ok(RecordingSession {
            stop_flag,
            handle: Some(handle),
        })
    }

    fn check_device() -> Result<(), String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| "no microphone input device available".to_string())?;
        device
            .default_input_config()
            .map_err(|e| format!("default input config error: {e}"))?;
        Ok(())
    }
}

impl RecordingSession {
    /// Stop capturing and return the accumulated partial transcript.
    pub fn stop(mut self) -> Result<String, String> {
        self.stop_flag.store(true, Ordering::Relaxed);
        match self.handle.take() {
            Some(handle) => handle
                .join()
                .map_err(|_| "audio thread panicked".to_string())?,
            None => Ok(String::new()),
        }
    }
}

fn open_stream(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    format: SampleFormat,
    step: usize,
    chunk_tx: mpsc::SyncSender<Vec<i16>>,
) -> Result<cpal::Stream, String> {
    match format {
        SampleFormat::F32 => build::<f32>(device, config, step, chunk_tx),
        SampleFormat::I16 => build::<i16>(device, config, step, chunk_tx),
        SampleFormat::I32 => build::<i32>(device, config, step, chunk_tx),
        SampleFormat::U8 => build::<u8>(device, config, step, chunk_tx),
        other => Err(format!("unsupported sample format: {other}")),
    }
}

fn build<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    step: usize,
    chunk_tx: mpsc::SyncSender<Vec<i16>>,
) -> Result<cpal::Stream, String>
where
    T: SizedSample,
    i16: FromSample<T>,
{
    device
        .build_input_stream::<T, _, _>(
            config,
            move |data: &[T], _| {
                // Down-mix to mono by taking the first channel, then convert to i16.
                let mono: Vec<i16> = data
                    .iter()
                    .step_by(step)
                    .map(|s| cpal::Sample::to_sample::<i16>(*s))
                    .collect();
                // ponytail: never block the audio thread — drop chunks if the
                // consumer falls behind rather than stalling capture.
                let _ = chunk_tx.try_send(mono);
            },
            |err| eprintln!("voicepilot audio stream error: {err}"),
            None,
        )
        .map_err(|e| e.to_string())
}

fn encode_s16le(samples: &[i16]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 2);
    for &sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    bytes
}
