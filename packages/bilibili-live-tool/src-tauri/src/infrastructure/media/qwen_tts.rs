//! Qwen3-TTS speech service adapter.
//!
//! The adapter speaks a small HTTP contract (see `services/qwen3-tts/server.py`)
//! and exposes it through [`SpeechSynthesisPort`], so the live-event 播报 path
//! and the AI voice-reply path share one implementation. Model details never
//! leak past this module: callers only see WAV bytes or `MediaIoError`.
//!
//! Graceful degradation rules:
//! * service disabled            -> `qwen3_tts_disabled` (no retry)
//! * connection refused / timeout -> `qwen3_tts_unreachable` (retryable)
//! * model still loading          -> `qwen3_tts_loading` (retryable)
//! * invalid request from client  -> `qwen3_tts_invalid_request` (no retry)
//! * 5xx from the service         -> `qwen3_tts_synthesis_failed` (retryable)

use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    application::media::{
        AudioOutputPort, AudioRequest, MediaIoError, SpeechEnginePort, SpeechRequest, SpeechSynthesisPort,
        SpeechVoice, SynthesisHealth, SynthesisRequest, SynthesisVoice,
    },
    domain::media::{validate_qwen3_tts_config, Qwen3TtsConfig},
    infrastructure::config_repository::ConfigRepository,
};

pub const QWEN3_TTS_CONFIG_KEY: &str = "media.qwen3_tts";
const CONFIG_SCOPE: &str = "global";
const MAX_PLAYBACK_WAIT: Duration = Duration::from_secs(120);
const PLAYBACK_POLL: Duration = Duration::from_millis(20);
/// Voice listing is a UI convenience; it must not inherit the synthesis timeout.
const VOICE_PROBE_TIMEOUT: Duration = Duration::from_millis(1_500);

fn lock_read<T>(lock: &RwLock<T>) -> std::sync::RwLockReadGuard<'_, T> {
    lock.read().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn lock_write<T>(lock: &RwLock<T>) -> std::sync::RwLockWriteGuard<'_, T> {
    lock.write().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Shared Qwen3-TTS service handle: configuration, persistence and transport.
pub struct Qwen3TtsService {
    config: RwLock<Qwen3TtsConfig>,
    http: reqwest::Client,
    pool: Option<SqlitePool>,
}

impl Qwen3TtsService {
    pub fn new(config: Qwen3TtsConfig, pool: Option<SqlitePool>) -> Arc<Self> {
        Arc::new(Self {
            config: RwLock::new(config),
            http: reqwest::Client::new(),
            pool,
        })
    }

    /// Loads the persisted record, falling back to defaults on any failure so a
    /// corrupted row can never block application start-up.
    pub async fn bootstrap(pool: Option<SqlitePool>) -> Arc<Self> {
        let mut config = Qwen3TtsConfig::default();
        if let Some(pool) = pool.as_ref() {
            match ConfigRepository::new(pool.clone())
                .get(QWEN3_TTS_CONFIG_KEY, CONFIG_SCOPE)
                .await
            {
                Ok(Some(stored)) => match serde_json::from_str::<Qwen3TtsConfig>(&stored.value) {
                    Ok(parsed) if validate_qwen3_tts_config(&parsed).is_ok() => config = parsed,
                    Ok(_) => log::warn!("qwen3 tts config is invalid; using defaults"),
                    Err(error) => log::warn!("qwen3 tts config is unreadable: {error}"),
                },
                Ok(None) => {}
                Err(error) => log::warn!("qwen3 tts config is degraded: {error}"),
            }
        }
        Self::new(config, pool)
    }

    pub fn config(&self) -> Qwen3TtsConfig {
        lock_read(&self.config).clone()
    }

    pub fn is_enabled(&self) -> bool {
        lock_read(&self.config).is_enabled
    }

    pub fn fallback_to_system(&self) -> bool {
        lock_read(&self.config).fallback_to_system
    }

    /// Replaces the in-memory configuration without persisting it.
    pub fn apply_config(&self, config: Qwen3TtsConfig) {
        *lock_write(&self.config) = config;
    }

    /// Validates, persists and applies a new configuration.
    pub async fn save_config(&self, config: Qwen3TtsConfig) -> Result<(), MediaIoError> {
        validate_qwen3_tts_config(&config)
            .map_err(|error| MediaIoError::coded("TTS_INVALID_CONFIG", error.to_string(), false))?;
        if let Some(pool) = self.pool.as_ref() {
            let value = serde_json::to_string(&config)
                .map_err(|_| MediaIoError::coded("TTS_INVALID_CONFIG", "config cannot be encoded", false))?;
            ConfigRepository::new(pool.clone())
                .set(QWEN3_TTS_CONFIG_KEY, &value, CONFIG_SCOPE)
                .await
                .map_err(|error| MediaIoError::operation(format!("qwen3 tts config save failed: {error}"), true))?;
        }
        self.apply_config(config);
        Ok(())
    }

    fn endpoint(&self, base: &str, suffix: &str) -> String {
        format!("{}{suffix}", base.trim().trim_end_matches('/'))
    }

    async fn send_with_timeout<T>(
        &self,
        timeout_ms: u64,
        future: impl std::future::Future<Output = Result<T, reqwest::Error>>,
    ) -> Result<T, MediaIoError> {
        match tokio::time::timeout(Duration::from_millis(timeout_ms), future).await {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) if error.is_timeout() || error.is_connect() => Err(MediaIoError::coded(
                "qwen3_tts_unreachable",
                format!("qwen3 tts service is unreachable: {error}"),
                true,
            )),
            Ok(Err(error)) => Err(MediaIoError::coded(
                "qwen3_tts_unreachable",
                format!("qwen3 tts request failed: {error}"),
                true,
            )),
            Err(_) => Err(MediaIoError::coded(
                "TTS_TIMEOUT",
                format!("qwen3 tts request exceeded {timeout_ms}ms"),
                true,
            )),
        }
    }

    fn map_status(&self, status: reqwest::StatusCode, body: &str) -> MediaIoError {
        let code = status.as_u16().to_string();
        if let Ok(payload) = serde_json::from_str::<ErrorPayload>(body) {
            if let Some(translated) = translated_code(&payload.error.code) {
                return MediaIoError::coded(
                    translated,
                    format!("qwen3 tts rejected the request ({code}): {}", payload.error.code),
                    !status.is_client_error() || status == reqwest::StatusCode::CONFLICT,
                );
            }
        }
        if status == reqwest::StatusCode::SERVICE_UNAVAILABLE {
            return MediaIoError::coded("TTS_MODEL_LOADING", "qwen3 tts model is still loading", true);
        }
        if status.is_client_error() {
            return MediaIoError::coded(
                "qwen3_tts_invalid_request",
                format!("qwen3 tts rejected the request ({code}): {body}"),
                false,
            );
        }
        MediaIoError::coded(
            "TTS_SYNTHESIS_FAILED_ENGINE",
            format!("qwen3 tts failed ({code}): {body}"),
            true,
        )
    }
}

/// One released Qwen3-TTS checkpoint as advertised by the service.
#[derive(Clone, Debug, PartialEq)]
pub struct Qwen3ModelEntry {
    pub model_id: String,
    pub display_name: String,
    pub task: String,
    pub is_default: bool,
    pub is_installed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Qwen3DownloadStatus {
    Idle,
    Downloading,
    Ready,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Qwen3DownloadState {
    pub model_id: Option<String>,
    pub status: Qwen3DownloadStatus,
    pub progress_percent: Option<u8>,
    pub bytes_downloaded: u64,
    pub bytes_total: Option<u64>,
    pub path: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Qwen3ModelCatalog {
    pub models: Vec<Qwen3ModelEntry>,
    pub download: Qwen3DownloadState,
}

#[derive(Debug, Deserialize)]
struct ModelCatalogPayload {
    #[serde(default)]
    models: Vec<ModelEntryPayload>,
    #[serde(default)]
    download: Option<DownloadPayload>,
}

#[derive(Debug, Deserialize)]
struct ModelEntryPayload {
    #[serde(default)]
    model_id: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    task: String,
    #[serde(default)]
    is_default: bool,
    #[serde(default)]
    is_installed: bool,
}

#[derive(Debug, Deserialize)]
struct DownloadPayload {
    #[serde(default)]
    model_id: Option<String>,
    #[serde(default)]
    status: String,
    #[serde(default)]
    progress_percent: Option<u8>,
    #[serde(default)]
    bytes_downloaded: u64,
    #[serde(default)]
    bytes_total: Option<u64>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Debug, Serialize)]
struct DownloadRequestPayload<'a> {
    model: Option<&'a str>,
}

/// Translates a stable service error code into the client-side vocabulary.
fn translated_code(server_code: &str) -> Option<&'static str> {
    match server_code {
        "tts_model_unknown" => Some("qwen3_tts_model_unknown"),
        "tts_download_busy" => Some("qwen3_tts_download_busy"),
        "TTS_DOWNLOAD_UNAVAILABLE_ENGINE" => Some("TTS_DOWNLOAD_UNAVAILABLE"),
        _ => None,
    }
}

#[derive(Debug, Deserialize)]
struct ErrorPayload {
    error: ErrorBody,
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    #[serde(default)]
    code: String,
}

#[derive(Debug, Deserialize)]
struct HealthPayload {
    #[serde(default)]
    ready: bool,
    #[serde(default)]
    status: String,
    #[serde(default)]
    engine: String,
    #[serde(default)]
    model: String,
    #[serde(default)]
    sample_rate: u32,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VoicesPayload {
    #[serde(default)]
    voices: Vec<VoicePayload>,
}

#[derive(Debug, Deserialize)]
struct VoicePayload {
    #[serde(default)]
    voice_id: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    language: String,
}

#[derive(Debug, Serialize)]
struct SpeechPayload<'a> {
    text: &'a str,
    voice: Option<&'a str>,
    language: &'a str,
    instruct: Option<&'a str>,
    response_format: &'a str,
}

impl From<ModelEntryPayload> for Qwen3ModelEntry {
    fn from(payload: ModelEntryPayload) -> Self {
        Self {
            display_name: if payload.display_name.trim().is_empty() {
                payload.model_id.clone()
            } else {
                payload.display_name
            },
            model_id: payload.model_id,
            task: payload.task,
            is_default: payload.is_default,
            is_installed: payload.is_installed,
        }
    }
}

impl From<DownloadPayload> for Qwen3DownloadState {
    fn from(payload: DownloadPayload) -> Self {
        Self {
            status: match payload.status.as_str() {
                "downloading" => Qwen3DownloadStatus::Downloading,
                "ready" => Qwen3DownloadStatus::Ready,
                "error" => Qwen3DownloadStatus::Error,
                _ => Qwen3DownloadStatus::Idle,
            },
            model_id: payload.model_id,
            progress_percent: payload.progress_percent.map(|value| value.min(100)),
            bytes_downloaded: payload.bytes_downloaded,
            bytes_total: payload.bytes_total,
            path: payload.path,
            error: payload.error,
        }
    }
}

const fn idle_download_state() -> Qwen3DownloadState {
    Qwen3DownloadState {
        model_id: None,
        status: Qwen3DownloadStatus::Idle,
        progress_percent: None,
        bytes_downloaded: 0,
        bytes_total: None,
        path: None,
        error: None,
    }
}

impl Qwen3TtsService {
    /// Lists the released checkpoints and the current download state.
    pub async fn model_catalog(&self) -> Result<Qwen3ModelCatalog, MediaIoError> {
        let config = self.config();
        let url = self.endpoint(&config.base_url, "/v1/models");
        let response = self
            .send_with_timeout(config.timeout_ms, self.http.get(url).send())
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(self.map_status(status, &body));
        }
        let payload: ModelCatalogPayload = self
            .send_with_timeout(config.timeout_ms, response.json())
            .await?;
        Ok(Qwen3ModelCatalog {
            models: payload.models.into_iter().map(Into::into).collect(),
            download: payload.download.map_or_else(idle_download_state, Into::into),
        })
    }

    /// Asks the service to fetch a checkpoint into its cache. `None` downloads
    /// the service default, which is the 1.7B CustomVoice model.
    pub async fn start_model_download(
        &self,
        model_id: Option<&str>,
    ) -> Result<Qwen3DownloadState, MediaIoError> {
        let config = self.config();
        let url = self.endpoint(&config.base_url, "/v1/models/download");
        let body = DownloadRequestPayload {
            model: model_id.filter(|value| !value.trim().is_empty()),
        };
        let response = self
            .send_with_timeout(
                // Starting a download is a local operation; the transfer itself
                // keeps running inside the service.
                config.timeout_ms,
                self.http.post(url).json(&body).send(),
            )
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(self.map_status(status, &body));
        }
        let payload: DownloadPayload = self
            .send_with_timeout(config.timeout_ms, response.json())
            .await?;
        Ok(payload.into())
    }
}

#[async_trait::async_trait]
impl SpeechSynthesisPort for Qwen3TtsService {
    async fn health(&self) -> Result<SynthesisHealth, MediaIoError> {
        let config = self.config();
        let url = self.endpoint(&config.base_url, "/health");
        let response = self
            .send_with_timeout(config.timeout_ms, self.http.get(url).send())
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(self.map_status(status, &body));
        }
        let payload: HealthPayload = self
            .send_with_timeout(config.timeout_ms, response.json())
            .await?;
        Ok(SynthesisHealth {
            ready: payload.ready,
            status: payload.status,
            engine: payload.engine,
            model: payload.model,
            sample_rate: payload.sample_rate,
            error: payload.error,
        })
    }

    async fn voices(&self) -> Result<Vec<SynthesisVoice>, MediaIoError> {
        let config = self.config();
        let url = self.endpoint(&config.base_url, "/v1/voices");
        let response = self
            .send_with_timeout(config.timeout_ms, self.http.get(url).send())
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(self.map_status(status, &body));
        }
        let payload: VoicesPayload = self
            .send_with_timeout(config.timeout_ms, response.json())
            .await?;
        Ok(payload
            .voices
            .into_iter()
            .filter(|voice| !voice.voice_id.trim().is_empty())
            .map(|voice| SynthesisVoice {
                display_name: if voice.display_name.trim().is_empty() {
                    voice.voice_id.clone()
                } else {
                    voice.display_name
                },
                voice_id: voice.voice_id,
                language: voice.language,
            })
            .collect())
    }

    async fn synthesize_wav(&self, request: SynthesisRequest) -> Result<Vec<u8>, MediaIoError> {
        if request.text.trim().is_empty() {
            return Err(MediaIoError::coded(
                "qwen3_tts_invalid_request",
                "synthesis text is required",
                false,
            ));
        }
        if !request.rate.is_finite() || request.rate <= 0.0 {
            return Err(MediaIoError::coded(
                "qwen3_tts_invalid_request",
                "synthesis rate is invalid",
                false,
            ));
        }
        let config = self.config();
        if !config.is_enabled {
            return Err(MediaIoError::coded(
                "qwen3_tts_disabled",
                "qwen3 tts is disabled",
                false,
            ));
        }
        let url = self.endpoint(&config.base_url, "/v1/audio/speech");
        let instruct = compose_instruct(&request.instruct, request.rate);
        let payload = SpeechPayload {
            text: request.text.trim(),
            voice: request.voice_id.as_deref(),
            language: &request.language,
            instruct: instruct.as_deref(),
            response_format: "wav",
        };
        let response = self
            .send_with_timeout(config.timeout_ms, self.http.post(url).json(&payload).send())
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(self.map_status(status, &body));
        }
        let bytes = self
            .send_with_timeout(config.timeout_ms + 10_000, response.bytes())
            .await?
            .to_vec();
        if !bytes.starts_with(b"RIFF") || bytes.len() <= 44 {
            return Err(MediaIoError::coded(
                "qwen3_tts_invalid_response",
                "qwen3 tts returned a payload that is not a RIFF/WAVE stream",
                false,
            ));
        }
        Ok(bytes)
    }
}

/// Folds a playback rate multiplier into a natural-language instruction, since
/// Qwen3-TTS controls speaking rate through instructions rather than resampling.
pub fn compose_instruct(base: &str, rate: f64) -> Option<String> {
    let base = base.trim();
    let pace = if rate >= 1.15 {
        Some("语速稍快")
    } else if rate <= 0.85 {
        Some("语速稍慢")
    } else {
        None
    };
    let mut parts: Vec<&str> = Vec::new();
    if !base.is_empty() {
        parts.push(base);
    }
    if let Some(pace) = pace {
        parts.push(pace);
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("，"))
    }
}

/// Deletes the temporary WAV file when the playback finishes or is cancelled.
struct TempWav {
    path: PathBuf,
}

impl TempWav {
    fn write(bytes: &[u8]) -> Result<Self, MediaIoError> {
        let path = std::env::temp_dir().join(format!("bili-tts-{}.wav", uuid::Uuid::new_v4()));
        std::fs::write(&path, bytes).map_err(|error| {
            MediaIoError::operation(format!("qwen3 tts temp file failed: {error}"), true)
        })?;
        Ok(Self { path })
    }

    fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for TempWav {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// `SpeechEnginePort` implementation used by the live-event 播报 queue.
pub struct Qwen3TtsSpeechEngine {
    service: Arc<Qwen3TtsService>,
    audio: Arc<dyn AudioOutputPort>,
    current: std::sync::Mutex<Option<Box<dyn crate::application::media::AudioPlayback>>>,
}

impl Qwen3TtsSpeechEngine {
    pub fn new(service: Arc<Qwen3TtsService>, audio: Arc<dyn AudioOutputPort>) -> Self {
        Self {
            service,
            audio,
            current: std::sync::Mutex::new(None),
        }
    }
}

#[async_trait::async_trait]
impl SpeechEnginePort for Qwen3TtsSpeechEngine {
    async fn voices(&self) -> Result<Vec<SpeechVoice>, MediaIoError> {
        if !self.service.is_enabled() {
            return Err(MediaIoError::coded(
                "qwen3_tts_disabled",
                "qwen3 tts is disabled",
                false,
            ));
        }
        // Voice enumeration feeds the settings page, so it must never stall the
        // UI for the full synthesis timeout when the service is unreachable.
        let voices = match tokio::time::timeout(VOICE_PROBE_TIMEOUT, self.service.voices()).await {
            Ok(result) => result?,
            Err(_) => {
                return Err(MediaIoError::coded(
                    "TTS_TIMEOUT",
                    "qwen3 tts voice catalogue timed out",
                    true,
                ))
            }
        };
        Ok(voices
            .into_iter()
            .map(|voice| SpeechVoice {
                id: voice.voice_id,
                display_name: voice.display_name,
            })
            .collect())
    }

    async fn speak(&self, request: SpeechRequest) -> Result<(), MediaIoError> {
        if request.text.trim().is_empty()
            || request.text.chars().any(char::is_control)
            || !request.volume.is_finite()
            || !(0.0..=1.0).contains(&request.volume)
        {
            return Err(MediaIoError::invalid("speech request is invalid"));
        }
        let config = self.service.config();
        if !config.is_enabled {
            return Err(MediaIoError::coded(
                "qwen3_tts_disabled",
                "qwen3 tts is disabled",
                false,
            ));
        }
        let bytes = self
            .service
            .synthesize_wav(SynthesisRequest {
                text: request.text,
                voice_id: Some(
                    request
                        .voice_id
                        .filter(|value| !value.trim().is_empty())
                        .unwrap_or_else(|| config.event_voice().to_owned()),
                ),
                language: config.language.clone(),
                instruct: config.instruct.clone(),
                rate: request.rate,
            })
            .await?;
        let wav = TempWav::write(&bytes)?;
        let playback = self
            .audio
            .play(AudioRequest {
                source: wav.path().to_path_buf(),
                volume: request.volume,
            })
            .await?;
        {
            let mut slot = self
                .current
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            *slot = Some(playback);
        }
        let started = std::time::Instant::now();
        loop {
            let finished = {
                let slot = self
                    .current
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                slot.as_ref().map_or(true, |playback| playback.is_finished())
            };
            if finished {
                break;
            }
            if started.elapsed() > MAX_PLAYBACK_WAIT {
                self.stop().await?;
                return Err(MediaIoError::operation("qwen3 tts playback timed out", true));
            }
            tokio::time::sleep(PLAYBACK_POLL).await;
        }
        self.current
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        drop(wav);
        Ok(())
    }

    async fn stop(&self) -> Result<(), MediaIoError> {
        let playback = self
            .current
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        if let Some(playback) = playback {
            playback.stop();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{compose_instruct, Qwen3TtsService, TempWav};
    use crate::{
        application::media::{SpeechSynthesisPort, SynthesisRequest},
        domain::media::Qwen3TtsConfig,
    };
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    fn wav_bytes() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&100u32.to_le_bytes());
        bytes.extend_from_slice(b"WAVEfmt ");
        bytes.extend_from_slice(&[0u8; 32]);
        bytes
    }

    /// Serves one canned HTTP response per connection so the adapter can be
    /// exercised without a real model service.
    async fn spawn_stub(status: &'static str, content_type: &'static str, body: Vec<u8>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move {
            loop {
                let Ok((mut socket, _)) = listener.accept().await else {
                    break;
                };
                let mut buffer = [0u8; 4096];
                let _ = socket.read(&mut buffer).await;
                let response = format!(
                    "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = socket.write_all(response.as_bytes()).await;
                let _ = socket.write_all(&body).await;
                let _ = socket.flush().await;
            }
        });
        format!("http://{address}")
    }

    fn enabled_service(base_url: String) -> Arc<Qwen3TtsService> {
        Qwen3TtsService::new(
            Qwen3TtsConfig {
                is_enabled: true,
                base_url,
                ..Qwen3TtsConfig::default()
            },
            None,
        )
    }

    #[test]
    fn composes_rate_into_a_natural_language_instruction() {
        assert_eq!(compose_instruct("", 1.0), None);
        assert_eq!(compose_instruct("   ", 1.0), None);
        assert_eq!(compose_instruct("轻声细语", 1.0).unwrap(), "轻声细语");
        assert_eq!(compose_instruct("轻声细语", 1.4).unwrap(), "轻声细语，语速稍快");
        assert_eq!(compose_instruct("", 0.6).unwrap(), "语速稍慢");
    }

    #[tokio::test]
    async fn parses_a_voice_catalogue() {
        let base = spawn_stub(
            "200 OK",
            "application/json",
            br#"{"voices":[{"voice_id":"Serena","display_name":"Serena","language":"Chinese"}],"languages":["Chinese"]}"#.to_vec(),
        )
        .await;
        let voices = enabled_service(base).voices().await.unwrap();
        assert_eq!(voices.len(), 1);
        assert_eq!(voices[0].voice_id, "Serena");
        assert_eq!(voices[0].display_name, "Serena");
    }

    #[tokio::test]
    async fn reports_health_from_the_service() {
        let base = spawn_stub(
            "200 OK",
            "application/json",
            br#"{"status":"ok","ready":true,"engine":"qwen3-tts","model":"Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice","sample_rate":24000,"error":null}"#.to_vec(),
        )
        .await;
        let health = enabled_service(base).health().await.unwrap();
        assert!(health.ready);
        assert_eq!(health.sample_rate, 24_000);
        assert_eq!(health.engine, "qwen3-tts");
    }

    #[tokio::test]
    async fn returns_rejected_when_the_service_is_disabled() {
        let service = Qwen3TtsService::new(Qwen3TtsConfig::default(), None);
        let error = service
            .synthesize_wav(SynthesisRequest {
                text: "hi".into(),
                voice_id: None,
                language: "Chinese".into(),
                instruct: String::new(),
                rate: 1.0,
            })
            .await
            .unwrap_err();
        assert_eq!(error.code, "qwen3_tts_disabled");
        assert!(!error.retryable);
    }

    #[tokio::test]
    async fn rejects_a_non_wav_payload() {
        let base = spawn_stub("200 OK", "audio/wav", b"not a wave".to_vec()).await;
        let error = enabled_service(base)
            .synthesize_wav(SynthesisRequest {
                text: "hi".into(),
                voice_id: None,
                language: "Chinese".into(),
                instruct: String::new(),
                rate: 1.0,
            })
            .await
            .unwrap_err();
        assert_eq!(error.code, "qwen3_tts_invalid_response");
        assert!(!error.retryable);
    }

    #[tokio::test]
    async fn returns_the_synthesised_wave() {
        let base = spawn_stub("200 OK", "audio/wav", wav_bytes()).await;
        let bytes = enabled_service(base)
            .synthesize_wav(SynthesisRequest {
                text: "用户A说：今天直播真不错".into(),
                voice_id: Some("Serena".into()),
                language: "Chinese".into(),
                instruct: "温暖".into(),
                rate: 1.0,
            })
            .await
            .unwrap();
        assert!(bytes.starts_with(b"RIFF"));
    }

    #[tokio::test]
    async fn maps_a_loading_service_to_a_retryable_error() {
        let base = spawn_stub("503 Service Unavailable", "application/json", b"{}".to_vec()).await;
        let error = enabled_service(base)
            .synthesize_wav(SynthesisRequest {
                text: "hi".into(),
                voice_id: None,
                language: "Chinese".into(),
                instruct: String::new(),
                rate: 1.0,
            })
            .await
            .unwrap_err();
        assert_eq!(error.code, "TTS_MODEL_LOADING");
        assert!(error.retryable);
    }

    #[tokio::test]
    async fn maps_a_bad_request_to_a_non_retryable_error() {
        let base = spawn_stub("400 Bad Request", "application/json", b"{}".to_vec()).await;
        let error = enabled_service(base)
            .synthesize_wav(SynthesisRequest {
                text: "hi".into(),
                voice_id: None,
                language: "Klingon".into(),
                instruct: String::new(),
                rate: 1.0,
            })
            .await
            .unwrap_err();
        assert_eq!(error.code, "qwen3_tts_invalid_request");
        assert!(!error.retryable);
    }

    #[test]
    fn temporary_wave_files_are_removed_on_drop() {
        let wav = TempWav::write(b"RIFF0000WAVE").unwrap();
        let path = wav.path().to_path_buf();
        assert!(path.exists());
        drop(wav);
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn parses_the_model_catalogue_and_download_state() {
        let base = spawn_stub(
            "200 OK",
            "application/json",
            br#"{"models":[{"model_id":"Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice","display_name":"Qwen3-TTS 1.7B CustomVoice","task":"custom_voice","is_default":true,"is_installed":false}],"download":{"model_id":null,"status":"idle","progress_percent":null,"bytes_downloaded":0,"bytes_total":null,"path":null,"error":null}}"#.to_vec(),
        )
        .await;
        let catalog = enabled_service(base).model_catalog().await.unwrap();
        assert_eq!(catalog.models.len(), 1);
        assert_eq!(catalog.models[0].model_id, "Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice");
        assert!(catalog.models[0].is_default);
        assert!(!catalog.models[0].is_installed);
        assert_eq!(catalog.download.status, super::Qwen3DownloadStatus::Idle);
    }

    #[tokio::test]
    async fn starting_a_download_returns_the_in_flight_state() {
        let base = spawn_stub(
            "202 Accepted",
            "application/json",
            br#"{"model_id":"Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice","status":"downloading","progress_percent":0,"bytes_downloaded":0,"bytes_total":4540000000,"path":null,"error":null}"#.to_vec(),
        )
        .await;
        let state = enabled_service(base).start_model_download(None).await.unwrap();
        assert_eq!(state.status, super::Qwen3DownloadStatus::Downloading);
        assert_eq!(state.model_id.as_deref(), Some("Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice"));
        assert_eq!(state.progress_percent, Some(0));
    }

    #[tokio::test]
    async fn maps_an_unknown_model_to_a_dedicated_code() {
        let base = spawn_stub(
            "400 Bad Request",
            "application/json",
            br#"{"error":{"code":"tts_model_unknown","message":"unsupported model","retryable":false}}"#.to_vec(),
        )
        .await;
        let error = enabled_service(base)
            .start_model_download(Some("Qwen/not-a-model"))
            .await
            .unwrap_err();
        assert_eq!(error.code, "qwen3_tts_model_unknown");
        assert!(!error.retryable);
    }

    #[tokio::test]
    async fn maps_a_busy_download_to_a_retryable_conflict() {
        let base = spawn_stub(
            "409 Conflict",
            "application/json",
            br#"{"error":{"code":"tts_download_busy","message":"busy","retryable":true}}"#.to_vec(),
        )
        .await;
        let error = enabled_service(base)
            .start_model_download(Some("Qwen/Qwen3-TTS-12Hz-0.6B-CustomVoice"))
            .await
            .unwrap_err();
        assert_eq!(error.code, "qwen3_tts_download_busy");
        assert!(error.retryable);
    }
}
