use std::sync::{Arc, Mutex};

use crate::application::media::{MediaIoError, SpeechEnginePort, SpeechRequest, SpeechVoice};

#[derive(Clone)]
pub struct SystemSpeechEngine {
    engine: Arc<Mutex<tts::Tts>>,
}

impl SystemSpeechEngine {
    pub fn new() -> Result<Self, MediaIoError> {
        tts::Tts::default()
            .map(|engine| Self {
                engine: Arc::new(Mutex::new(engine)),
            })
            .map_err(|_| MediaIoError::unavailable("system speech is unavailable"))
    }
}

#[async_trait::async_trait]
impl SpeechEnginePort for SystemSpeechEngine {
    async fn voices(&self) -> Result<Vec<SpeechVoice>, MediaIoError> {
        let engine = self.engine.clone();
        tokio::task::spawn_blocking(move || {
            let engine = engine
                .lock()
                .map_err(|_| MediaIoError::unavailable("system speech is unavailable"))?;
            engine
                .voices()
                .map(|voices| {
                    voices
                        .into_iter()
                        .map(|voice| SpeechVoice {
                            id: voice.id(),
                            display_name: voice.name(),
                        })
                        .collect()
                })
                .map_err(|_| MediaIoError::unavailable("system voices are unavailable"))
        })
        .await
        .map_err(|_| MediaIoError::operation("speech worker failed", true))?
    }

    async fn speak(&self, request: SpeechRequest) -> Result<(), MediaIoError> {
        if request.text.trim().is_empty()
            || request.text.chars().any(char::is_control)
            || !request.rate.is_finite()
            || request.rate <= 0.0
            || !request.volume.is_finite()
            || !(0.0..=1.0).contains(&request.volume)
        {
            return Err(MediaIoError::invalid("speech request is invalid"));
        }
        let engine = self.engine.clone();
        tokio::task::spawn_blocking(move || {
            {
                let mut engine = engine
                    .lock()
                    .map_err(|_| MediaIoError::unavailable("system speech is unavailable"))?;
                if let Some(voice_id) = request.voice_id {
                    let voice = engine
                        .voices()
                        .map_err(|_| MediaIoError::unavailable("system voices are unavailable"))?
                        .into_iter()
                        .find(|voice| voice.id() == voice_id)
                        .ok_or_else(|| {
                            MediaIoError::invalid("selected speech voice is unavailable")
                        })?;
                    engine.set_voice(&voice).map_err(|_| {
                        MediaIoError::operation("speech voice selection failed", false)
                    })?;
                }
                engine
                    .set_rate(request.rate as f32)
                    .map_err(|_| MediaIoError::invalid("speech rate is unsupported"))?;
                engine
                    .set_volume(request.volume as f32)
                    .map_err(|_| MediaIoError::invalid("speech volume is unsupported"))?;
                engine
                    .speak(request.text, false)
                    .map_err(|_| MediaIoError::operation("speech playback failed", true))?;
            }
            loop {
                let speaking = engine
                    .lock()
                    .map_err(|_| MediaIoError::unavailable("system speech is unavailable"))?
                    .is_speaking()
                    .unwrap_or(false);
                if !speaking {
                    return Ok(());
                }
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        })
        .await
        .map_err(|_| MediaIoError::operation("speech worker failed", true))?
    }

    async fn stop(&self) -> Result<(), MediaIoError> {
        let engine = self.engine.clone();
        tokio::task::spawn_blocking(move || {
            engine
                .lock()
                .map_err(|_| MediaIoError::unavailable("system speech is unavailable"))?
                .stop()
                .map(|_| ())
                .map_err(|_| MediaIoError::operation("speech stop failed", true))
        })
        .await
        .map_err(|_| MediaIoError::operation("speech worker failed", true))?
    }
}

pub struct UnavailableSpeechEngine;

#[async_trait::async_trait]
impl SpeechEnginePort for UnavailableSpeechEngine {
    async fn voices(&self) -> Result<Vec<SpeechVoice>, MediaIoError> {
        Err(MediaIoError::unavailable("speech engine is unavailable"))
    }

    async fn speak(&self, _: SpeechRequest) -> Result<(), MediaIoError> {
        Err(MediaIoError::unavailable("speech engine is unavailable"))
    }

    async fn stop(&self) -> Result<(), MediaIoError> {
        Ok(())
    }
}
