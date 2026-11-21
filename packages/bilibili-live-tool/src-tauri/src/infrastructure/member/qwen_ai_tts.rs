//! Qwen3-TTS based `AiSpeechOutput` for AI voice replies.
//!
//! The AI reply path must keep the existing audio-routing behaviour: operators
//! pick a SAPI output device and that exact device must reproduce the reply.
//! So instead of opening a second audio stack, the synthesised WAV is handed to
//! SAPI through `ISpeechVoice::SpeakStream`, on the same device-routed voice.
//!
//! Rollback is a configuration change, not a rebuild:
//! * `qwen3.isEnabled = false`           -> every reply uses the system voice
//! * selecting a non `qwen3:` voice id   -> that reply uses the system voice
//! * service failure + `fallbackToSystem` -> that reply uses the system voice

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use crate::{
    application::member::{
        AiOutputDeviceRecord, AiSpeechOutput, AiVoiceRecord, MemberServiceError,
    },
    application::media::{SpeechSynthesisPort, SynthesisRequest},
    infrastructure::media::Qwen3TtsService,
};

/// Voice ids from the Qwen3-TTS catalogue are surfaced with this prefix so a
/// single `voice_id` string unambiguously selects the synthesis backend.
pub const QWEN3_VOICE_PREFIX: &str = "qwen3:";

pub struct Qwen3TtsAiSpeechOutput {
    service: Arc<Qwen3TtsService>,
    fallback: Arc<dyn AiSpeechOutput>,
    cancel_epoch: Arc<AtomicU64>,
}

impl Qwen3TtsAiSpeechOutput {
    pub fn new(service: Arc<Qwen3TtsService>, fallback: Arc<dyn AiSpeechOutput>) -> Self {
        Self {
            service,
            fallback,
            cancel_epoch: Arc::new(AtomicU64::new(0)),
        }
    }

    fn map_error(error: crate::application::media::MediaIoError) -> MemberServiceError {
        match error.code {
            "qwen3_tts_disabled" => MemberServiceError::invalid("TTS_DISABLED"),
            "qwen3_tts_invalid_request" => MemberServiceError::invalid("TTS_REQUEST_INVALID"),
            _ if error.retryable => MemberServiceError::unavailable("TTS_UNAVAILABLE"),
            _ => MemberServiceError::invalid("TTS_SYNTHESIS_FAILED"),
        }
    }

    async fn synthesize_reply(
        &self,
        text: &str,
        speaker: Option<&str>,
    ) -> Result<Vec<u8>, MemberServiceError> {
        let config = self.service.config();
        // An explicitly selected `qwen3:<speaker>` wins over the configured default.
        let voice = speaker
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| config.reply_voice().to_owned());
        self.service
            .synthesize_wav(SynthesisRequest {
                text: text.to_owned(),
                voice_id: Some(voice),
                language: config.language.clone(),
                instruct: config.reply_instruct().to_owned(),
                rate: 1.0,
            })
            .await
            .map_err(Self::map_error)
    }
}

#[async_trait::async_trait]
impl AiSpeechOutput for Qwen3TtsAiSpeechOutput {
    async fn devices(&self) -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError> {
        // Routing is still SAPI's, so the device list is unchanged.
        self.fallback.devices().await
    }

    async fn voices(&self) -> Result<Vec<AiVoiceRecord>, MemberServiceError> {
        let mut voices: Vec<AiVoiceRecord> = Vec::new();
        if self.service.is_enabled() {
            if let Ok(catalogue) = self.service.voices().await {
                voices.extend(catalogue.into_iter().map(|voice| AiVoiceRecord {
                    voice_id: format!("{QWEN3_VOICE_PREFIX}{}", voice.voice_id),
                    display_name: format!("Qwen3-TTS · {}", voice.display_name),
                    is_available: true,
                    is_default: false,
                }));
            }
        }
        match self.fallback.voices().await {
            Ok(system) => voices.extend(system),
            Err(error) if voices.is_empty() => return Err(error),
            Err(_) => {}
        }
        Ok(voices)
    }

    async fn speak(
        &self,
        text: &str,
        device_id: &str,
        voice_id: &str,
        volume: f64,
    ) -> Result<(), MemberServiceError> {
        if text.trim().is_empty() {
            return Err(MemberServiceError::invalid("TTS_REQUEST_INVALID"));
        }
        let Some(speaker) = voice_id.strip_prefix(QWEN3_VOICE_PREFIX) else {
            return self.fallback.speak(text, device_id, voice_id, volume).await;
        };
        if !self.service.is_enabled() {
            return self.route_to_fallback(text, device_id, voice_id, volume, "TTS_DISABLED").await;
        }
        let bytes = match self.synthesize_reply(text, Some(speaker)).await {
            Ok(bytes) => bytes,
            Err(error) => {
                return self
                    .route_to_fallback(text, device_id, voice_id, volume, error.code)
                    .await
                    .map_err(|_| error)
            }
        };
        let epoch = self.cancel_epoch.load(Ordering::Acquire);
        match play_wav_on_device(&bytes, device_id, volume, &self.cancel_epoch, epoch) {
            Ok(()) => Ok(()),
            Err(error) => self
                .route_to_fallback(text, device_id, voice_id, volume, error.code)
                .await
                .map_err(|_| error),
        }
    }

    async fn stop(&self) -> Result<(), MemberServiceError> {
        self.cancel_epoch.fetch_add(1, Ordering::AcqRel);
        self.fallback.stop().await
    }
}

impl Qwen3TtsAiSpeechOutput {
    /// Uses the system voice for one reply when Qwen3-TTS cannot serve it.
    async fn route_to_fallback(
        &self,
        text: &str,
        device_id: &str,
        voice_id: &str,
        volume: f64,
        reason: &'static str,
    ) -> Result<(), MemberServiceError> {
        if !self.service.fallback_to_system() {
            return Err(MemberServiceError::unavailable("TTS_UNAVAILABLE"));
        }
        log::debug!("ai voice reply fell back to the system voice: {reason}");
        // The requested id is a Qwen3 speaker; ask the system engine for its own default.
        let fallback_voice = self
            .fallback
            .voices()
            .await
            .ok()
            .and_then(|voices| voices.into_iter().find(|voice| voice.is_default))
            .map(|voice| voice.voice_id)
            .unwrap_or_else(|| voice_id.to_owned());
        self.fallback
            .speak(text, device_id, &fallback_voice, volume)
            .await
    }
}

#[cfg(target_os = "windows")]
fn play_wav_on_device(
    bytes: &[u8],
    device_id: &str,
    volume: f64,
    cancel_epoch: &AtomicU64,
    epoch: u64,
) -> Result<(), MemberServiceError> {
    use std::io::Write;

    let path = std::env::temp_dir().join(format!("bili-ai-tts-{}.wav", uuid::Uuid::new_v4()));
    {
        let mut file =
            std::fs::File::create(&path).map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
        file.write_all(bytes)
            .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
    }
    let result = speak_stream_with_sapi(&path, device_id, volume, cancel_epoch, epoch);
    let _ = std::fs::remove_file(&path);
    result
}

#[cfg(target_os = "windows")]
fn speak_stream_with_sapi(
    path: &std::path::Path,
    device_id: &str,
    volume: f64,
    cancel_epoch: &AtomicU64,
    epoch: u64,
) -> Result<(), MemberServiceError> {
    use windows::{
        core::{BSTR, Interface},
        Win32::{
            Media::Speech::{
                ISpeechFileStream, ISpeechVoice, SpFileStream, SpVoice, SpeechStreamFileMode,
                SVSFlagsAsync, SVSFPurgeBeforeSpeak, SSFMOpenForRead,
            },
            System::Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
            },
        },
    };

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .ok()
            .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
        let result = (|| {
            let Some(file_name) = path.to_str() else {
                return Err(MemberServiceError::invalid("AUDIO_DEVICE_INVALID"));
            };
            let voice: ISpeechVoice = CoCreateInstance(&SpVoice, None, CLSCTX_ALL)
                .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
            let tokens = voice
                .GetAudioOutputs(&BSTR::new(), &BSTR::new())
                .map_err(|_| MemberServiceError::unavailable("AUDIO_DEVICE_UNAVAILABLE"))?;
            let count = tokens
                .Count()
                .map_err(|_| MemberServiceError::unavailable("AUDIO_DEVICE_UNAVAILABLE"))?;
            let mut selected = None;
            for index in 0..count {
                let token = tokens
                    .Item(index)
                    .map_err(|_| MemberServiceError::unavailable("AUDIO_DEVICE_UNAVAILABLE"))?;
                if token.Id().ok().is_some_and(|id| id == device_id) {
                    selected = Some(token);
                    break;
                }
            }
            let selected =
                selected.ok_or_else(|| MemberServiceError::invalid("AUDIO_DEVICE_INVALID"))?;
            voice
                .putref_AudioOutput(&selected)
                .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
            voice
                .SetVolume((volume * 100.0).round() as i32)
                .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
            let stream: ISpeechFileStream = CoCreateInstance(&SpFileStream, None, CLSCTX_ALL)
                .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
            stream
                .Open(
                    &BSTR::from(file_name),
                    SpeechStreamFileMode(SSFMOpenForRead.0),
                    false.into(),
                )
                .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
            let base = stream
                .cast::<windows::Win32::Media::Speech::ISpeechBaseStream>()
                .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
            voice
                .SpeakStream(&base, SVSFlagsAsync)
                .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?;
            loop {
                if cancel_epoch.load(Ordering::Acquire) != epoch {
                    let _ = voice.Speak(&BSTR::new(), SVSFPurgeBeforeSpeak);
                    break;
                }
                if voice
                    .WaitUntilDone(50)
                    .map_err(|_| MemberServiceError::unavailable("TTS_UNAVAILABLE"))?
                    .as_bool()
                {
                    break;
                }
            }
            let _ = stream.Close();
            Ok(())
        })();
        CoUninitialize();
        result
    }
}

#[cfg(not(target_os = "windows"))]
fn play_wav_on_device(
    _bytes: &[u8],
    _device_id: &str,
    _volume: f64,
    _cancel_epoch: &AtomicU64,
    _epoch: u64,
) -> Result<(), MemberServiceError> {
    Err(MemberServiceError::unavailable("TTS_UNAVAILABLE"))
}

#[cfg(test)]
mod tests {
    use super::{Qwen3TtsAiSpeechOutput, QWEN3_VOICE_PREFIX};
    use crate::{
        application::member::{AiOutputDeviceRecord, AiSpeechOutput, AiVoiceRecord, MemberServiceError},
        domain::media::Qwen3TtsConfig,
        infrastructure::media::Qwen3TtsService,
    };
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    struct RecordingSystemSpeech {
        spoken: Mutex<Vec<(String, String)>>,
    }

    #[async_trait::async_trait]
    impl AiSpeechOutput for RecordingSystemSpeech {
        async fn devices(&self) -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError> {
            Ok(vec![AiOutputDeviceRecord {
                device_id: "device-1".into(),
                display_name: "Speakers".into(),
                is_available: true,
                is_default: true,
            }])
        }

        async fn voices(&self) -> Result<Vec<AiVoiceRecord>, MemberServiceError> {
            Ok(vec![AiVoiceRecord {
                voice_id: "system-default".into(),
                display_name: "System".into(),
                is_available: true,
                is_default: true,
            }])
        }

        async fn speak(
            &self,
            text: &str,
            _device_id: &str,
            voice_id: &str,
            _volume: f64,
        ) -> Result<(), MemberServiceError> {
            self.spoken
                .lock()
                .unwrap()
                .push((text.to_owned(), voice_id.to_owned()));
            Ok(())
        }

        async fn stop(&self) -> Result<(), MemberServiceError> {
            Ok(())
        }
    }

    fn build(enabled: bool, fallback_to_system: bool) -> (Qwen3TtsAiSpeechOutput, Arc<RecordingSystemSpeech>) {
        let system = Arc::new(RecordingSystemSpeech::default());
        let service = Qwen3TtsService::new(
            Qwen3TtsConfig {
                is_enabled: enabled,
                base_url: "http://127.0.0.1:9".into(),
                timeout_ms: 500,
                fallback_to_system,
                ..Qwen3TtsConfig::default()
            },
            None,
        );
        (
            Qwen3TtsAiSpeechOutput::new(service, system.clone()),
            system,
        )
    }

    #[tokio::test]
    async fn a_system_voice_id_never_touches_the_qwen_service() {
        let (output, system) = build(true, true);
        output
            .speak("你好", "device-1", "system-default", 0.8)
            .await
            .unwrap();
        assert_eq!(system.spoken.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn an_unreachable_service_falls_back_to_the_system_voice() {
        let (output, system) = build(true, true);
        output
            .speak("你好", "device-1", &format!("{QWEN3_VOICE_PREFIX}Serena"), 0.8)
            .await
            .unwrap();
        let spoken = system.spoken.lock().unwrap();
        assert_eq!(spoken.len(), 1);
        assert_eq!(spoken[0].1, "system-default");
    }

    #[tokio::test]
    async fn a_disabled_service_falls_back_to_the_system_voice() {
        let (output, system) = build(false, true);
        output
            .speak("你好", "device-1", &format!("{QWEN3_VOICE_PREFIX}Serena"), 0.8)
            .await
            .unwrap();
        assert_eq!(system.spoken.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn a_failed_service_surfaces_the_error_when_fallback_is_off() {
        let (output, _system) = build(true, false);
        let error = output
            .speak("你好", "device-1", &format!("{QWEN3_VOICE_PREFIX}Serena"), 0.8)
            .await
            .unwrap_err();
        assert!(error.retryable);
    }

    #[tokio::test]
    async fn disabled_service_hides_qwen_voices_but_keeps_system_voices() {
        let (output, _system) = build(false, true);
        let voices = output.voices().await.unwrap();
        assert!(voices.iter().all(|voice| !voice.voice_id.starts_with(QWEN3_VOICE_PREFIX)));
        assert_eq!(voices.len(), 1);
    }

    /// Minimal HTTP stub that captures the raw request and returns a WAV payload.
    async fn spawn_capturing_stub(body: Vec<u8>) -> (String, Arc<Mutex<Vec<String>>>) {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let captured = Arc::new(Mutex::new(Vec::new()));
        let sink = captured.clone();
        tokio::spawn(async move {
            while let Ok((mut socket, _)) = listener.accept().await {
                let mut buffer = vec![0u8; 8192];
                let read = socket.read(&mut buffer).await.unwrap_or(0);
                sink.lock()
                    .unwrap()
                    .push(String::from_utf8_lossy(&buffer[..read]).to_string());
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: audio/wav\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = socket.write_all(response.as_bytes()).await;
                let _ = socket.write_all(&body).await;
                let _ = socket.flush().await;
            }
        });
        (format!("http://{address}"), captured)
    }

    fn wave_payload() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&100u32.to_le_bytes());
        bytes.extend_from_slice(b"WAVEfmt ");
        bytes.extend_from_slice(&[0u8; 32]);
        bytes
    }

    async fn capturing_output() -> (Qwen3TtsAiSpeechOutput, Arc<Mutex<Vec<String>>>) {
        let (base, captured) = spawn_capturing_stub(wave_payload()).await;
        let service = Qwen3TtsService::new(
            Qwen3TtsConfig {
                is_enabled: true,
                base_url: base,
                ..Qwen3TtsConfig::default()
            },
            None,
        );
        let output = Qwen3TtsAiSpeechOutput::new(service, Arc::new(RecordingSystemSpeech::default()));
        (output, captured)
    }

    #[tokio::test]
    async fn an_explicit_qwen3_speaker_overrides_the_configured_default() {
        let (output, captured) = capturing_output().await;
        output.synthesize_reply("你好", Some("Vivian")).await.unwrap();
        let request = captured.lock().unwrap().first().cloned().unwrap();
        assert!(request.contains("\"voice\":\"Vivian\""), "{request}");
    }

    #[tokio::test]
    async fn a_blank_speaker_falls_back_to_the_configured_reply_voice() {
        let (output, captured) = capturing_output().await;
        output.synthesize_reply("你好", None).await.unwrap();
        let request = captured.lock().unwrap().first().cloned().unwrap();
        assert!(request.contains("\"voice\":\"Serena\""), "{request}");
    }
}
