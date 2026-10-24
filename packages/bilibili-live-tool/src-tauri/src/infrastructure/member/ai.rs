use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use serde::Deserialize;

use crate::application::member::{
    AiModelProfileRecord, AiOutputDeviceRecord, AiSecretStore, AiSpeechOutput, AiVoiceRecord,
    LlmProvider, MemberRepository, MemberServiceError, DEFAULT_AI_VOICE_DISPLAY_NAME,
    DEFAULT_AI_VOICE_ID,
};

const KEYRING_SERVICE: &str = "bilibili-live-tool.ai";

pub struct SystemAiSecretStore;

#[async_trait::async_trait]
impl AiSecretStore for SystemAiSecretStore {
    async fn get(&self, profile_id: &str) -> Result<Option<String>, MemberServiceError> {
        #[cfg(not(target_os = "windows"))]
        {
            let _ = profile_id;
            return Err(secret_store_error());
        }
        #[cfg(target_os = "windows")]
        {
            let profile_id = profile_id.to_owned();
            tokio::task::spawn_blocking(move || windows_credential_get(&profile_id))
                .await
                .map_err(|_| secret_store_error())?
        }
    }

    async fn set(&self, profile_id: &str, secret: &str) -> Result<(), MemberServiceError> {
        #[cfg(not(target_os = "windows"))]
        {
            let _ = (profile_id, secret);
            return Err(secret_store_error());
        }
        #[cfg(target_os = "windows")]
        {
            let profile_id = profile_id.to_owned();
            let secret = secret.to_owned();
            tokio::task::spawn_blocking(move || windows_credential_set(&profile_id, &secret))
                .await
                .map_err(|_| secret_store_error())?
        }
    }

    async fn delete(&self, profile_id: &str) -> Result<(), MemberServiceError> {
        #[cfg(not(target_os = "windows"))]
        {
            let _ = profile_id;
            return Err(secret_store_error());
        }
        #[cfg(target_os = "windows")]
        {
            let profile_id = profile_id.to_owned();
            tokio::task::spawn_blocking(move || windows_credential_delete(&profile_id))
                .await
                .map_err(|_| secret_store_error())?
        }
    }
}

#[cfg(target_os = "windows")]
fn credential_target(profile_id: &str) -> Vec<u16> {
    format!("{KEYRING_SERVICE}/{profile_id}")
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}

#[cfg(target_os = "windows")]
fn windows_credential_get(profile_id: &str) -> Result<Option<String>, MemberServiceError> {
    use windows::{
        core::PCWSTR,
        Win32::{
            Foundation::ERROR_NOT_FOUND,
            Security::Credentials::{CredFree, CredReadW, CREDENTIALW, CRED_TYPE_GENERIC},
        },
    };

    let target = credential_target(profile_id);
    let mut credential: *mut CREDENTIALW = std::ptr::null_mut();
    let result = unsafe {
        CredReadW(
            PCWSTR(target.as_ptr()),
            CRED_TYPE_GENERIC,
            None,
            &mut credential,
        )
    };
    if let Err(error) = result {
        return if error.code() == ERROR_NOT_FOUND.to_hresult() {
            Ok(None)
        } else {
            Err(secret_store_error())
        };
    }
    let secret = unsafe {
        let value = &*credential;
        let bytes =
            std::slice::from_raw_parts(value.CredentialBlob, value.CredentialBlobSize as usize);
        let secret = String::from_utf8(bytes.to_vec()).map_err(|_| secret_store_error());
        CredFree(credential.cast());
        secret
    }?;
    Ok(Some(secret))
}

#[cfg(target_os = "windows")]
fn windows_credential_set(profile_id: &str, secret: &str) -> Result<(), MemberServiceError> {
    use windows::{
        core::PWSTR,
        Win32::Security::Credentials::{
            CredWriteW, CREDENTIALW, CRED_PERSIST_LOCAL_MACHINE, CRED_TYPE_GENERIC,
        },
    };

    let mut target = credential_target(profile_id);
    let mut username: Vec<u16> = "bilibili-live-tool"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let mut secret = secret.as_bytes().to_vec();
    let credential = CREDENTIALW {
        Type: CRED_TYPE_GENERIC,
        TargetName: PWSTR(target.as_mut_ptr()),
        CredentialBlobSize: secret.len() as u32,
        CredentialBlob: secret.as_mut_ptr(),
        Persist: CRED_PERSIST_LOCAL_MACHINE,
        UserName: PWSTR(username.as_mut_ptr()),
        ..Default::default()
    };
    let result = unsafe { CredWriteW(&credential, 0) }.map_err(|_| secret_store_error());
    secret.fill(0);
    result
}

#[cfg(target_os = "windows")]
fn windows_credential_delete(profile_id: &str) -> Result<(), MemberServiceError> {
    use windows::{
        core::PCWSTR,
        Win32::{
            Foundation::ERROR_NOT_FOUND,
            Security::Credentials::{CredDeleteW, CRED_TYPE_GENERIC},
        },
    };

    let target = credential_target(profile_id);
    match unsafe { CredDeleteW(PCWSTR(target.as_ptr()), CRED_TYPE_GENERIC, None) } {
        Ok(()) => Ok(()),
        Err(error) if error.code() == ERROR_NOT_FOUND.to_hresult() => Ok(()),
        Err(_) => Err(secret_store_error()),
    }
}

fn secret_store_error() -> MemberServiceError {
    MemberServiceError::unavailable("ai_secret_store_unavailable")
}

pub struct OpenAiCompatibleLlmProvider {
    repository: Arc<dyn MemberRepository>,
    secrets: Arc<dyn AiSecretStore>,
}

impl OpenAiCompatibleLlmProvider {
    pub fn new(repository: Arc<dyn MemberRepository>, secrets: Arc<dyn AiSecretStore>) -> Self {
        Self {
            repository,
            secrets,
        }
    }

    async fn request(
        &self,
        profile: &AiModelProfileRecord,
        api_key: Option<&str>,
        prompt: &str,
    ) -> Result<String, MemberServiceError> {
        let endpoint = chat_completions_url(&profile.base_url)?;
        let mut builder = reqwest::Client::builder().timeout(std::time::Duration::from_secs(
            u64::from(profile.timeout_seconds),
        ));
        if let Some(proxy_url) = profile.proxy_url.as_deref() {
            builder = builder.proxy(
                reqwest::Proxy::all(proxy_url)
                    .map_err(|_| MemberServiceError::invalid("ai_model_profile_invalid"))?,
            );
        }
        let client = builder
            .build()
            .map_err(|_| MemberServiceError::unavailable("ai_provider_unavailable"))?;
        let mut request = client.post(endpoint).json(&serde_json::json!({
            "model": profile.model,
            "messages": [
                {
                    "role": "system",
                    "content": "你必须像真人在直播间即时聊天一样回复：用口语短句，只回应一个核心点；不要分点、编号、标题、Markdown、客套开场或 AI 自指；输出适合直接朗读的纯文本。"
                },
                { "role": "user", "content": prompt }
            ],
            "reasoning_effort": profile.reasoning_effort.as_str(),
            "max_tokens": profile.max_tokens
        }));
        if let Some(api_key) = api_key.filter(|value| !value.is_empty()) {
            request = request.bearer_auth(api_key);
        }
        let response = request
            .send()
            .await
            .map_err(|_| MemberServiceError::unavailable("ai_provider_unavailable"))?;
        if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(MemberServiceError::unavailable("ai_rate_limited"));
        }
        if !response.status().is_success() {
            return Err(MemberServiceError::unavailable("ai_provider_unavailable"));
        }
        let payload: ChatCompletionResponse = response
            .json()
            .await
            .map_err(|_| MemberServiceError::invalid("ai_provider_invalid_response"))?;
        payload
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content.trim().to_owned())
            .filter(|content| !content.is_empty())
            .ok_or_else(|| MemberServiceError::invalid("ai_provider_invalid_response"))
    }
}

#[async_trait::async_trait]
impl LlmProvider for OpenAiCompatibleLlmProvider {
    async fn generate(&self, prompt: &str) -> Result<String, MemberServiceError> {
        let profile = self
            .repository
            .load_default_ai_model_profile()
            .await?
            .filter(|profile| profile.is_enabled)
            .ok_or_else(|| MemberServiceError::invalid("ai_model_not_configured"))?;
        let api_key = if profile.has_api_key {
            self.secrets.get(&profile.profile_id).await?
        } else {
            None
        };
        self.request(&profile, api_key.as_deref(), prompt).await
    }

    async fn test_profile(
        &self,
        profile: &AiModelProfileRecord,
        api_key: Option<&str>,
    ) -> Result<(), MemberServiceError> {
        self.request(profile, api_key, "只回复 OK")
            .await
            .map(|_| ())
    }
}

fn chat_completions_url(base_url: &str) -> Result<reqwest::Url, MemberServiceError> {
    let trimmed = base_url.trim().trim_end_matches('/');
    let endpoint = if trimmed.ends_with("/chat/completions") {
        trimmed.to_owned()
    } else if trimmed.ends_with("/v1") {
        format!("{trimmed}/chat/completions")
    } else {
        format!("{trimmed}/v1/chat/completions")
    };
    let url = reqwest::Url::parse(&endpoint)
        .map_err(|_| MemberServiceError::invalid("ai_model_profile_invalid"))?;
    if matches!(url.scheme(), "http" | "https") {
        Ok(url)
    } else {
        Err(MemberServiceError::invalid("ai_model_profile_invalid"))
    }
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatCompletionChoice>,
}

#[derive(Deserialize)]
struct ChatCompletionChoice {
    message: ChatCompletionMessage,
}

#[derive(Deserialize)]
struct ChatCompletionMessage {
    content: String,
}

pub struct SystemAiSpeechOutput {
    cancel_epoch: Arc<AtomicU64>,
}

impl Default for SystemAiSpeechOutput {
    fn default() -> Self {
        Self {
            cancel_epoch: Arc::new(AtomicU64::new(0)),
        }
    }
}

#[cfg(target_os = "windows")]
#[async_trait::async_trait]
impl AiSpeechOutput for SystemAiSpeechOutput {
    async fn devices(&self) -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError> {
        tokio::task::spawn_blocking(windows_output_devices)
            .await
            .map_err(|_| MemberServiceError::unavailable("ai_audio_device_unavailable"))?
    }

    async fn voices(&self) -> Result<Vec<AiVoiceRecord>, MemberServiceError> {
        tokio::task::spawn_blocking(windows_voices)
            .await
            .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?
    }

    async fn speak(
        &self,
        text: &str,
        device_id: &str,
        voice_id: &str,
        volume: f64,
    ) -> Result<(), MemberServiceError> {
        if text.trim().is_empty() || !volume.is_finite() || !(0.0..=1.0).contains(&volume) {
            return Err(MemberServiceError::invalid("ai_tts_request_invalid"));
        }
        let text = text.to_owned();
        let device_id = device_id.to_owned();
        let voice_id = voice_id.to_owned();
        let cancel_epoch = self.cancel_epoch.clone();
        let epoch = cancel_epoch.load(Ordering::Acquire);
        tokio::task::spawn_blocking(move || {
            windows_speak(&text, &device_id, &voice_id, volume, &cancel_epoch, epoch)
        })
        .await
        .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?
    }

    async fn stop(&self) -> Result<(), MemberServiceError> {
        self.cancel_epoch.fetch_add(1, Ordering::AcqRel);
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
#[async_trait::async_trait]
impl AiSpeechOutput for SystemAiSpeechOutput {
    async fn devices(&self) -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError> {
        Ok(vec![AiOutputDeviceRecord {
            device_id: "system-default".to_owned(),
            display_name: "System default output".to_owned(),
            is_available: true,
            is_default: true,
        }])
    }

    async fn voices(&self) -> Result<Vec<AiVoiceRecord>, MemberServiceError> {
        tokio::task::spawn_blocking(|| {
            let speech = tts::Tts::default()
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            let mut voices = vec![default_ai_voice()];
            voices.extend(
                speech
                    .voices()
                    .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?
                    .into_iter()
                    .map(|voice| AiVoiceRecord {
                        voice_id: voice.id(),
                        display_name: voice.name(),
                        is_available: true,
                        is_default: false,
                    }),
            );
            Ok(voices)
        })
        .await
        .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?
    }

    async fn speak(
        &self,
        text: &str,
        device_id: &str,
        voice_id: &str,
        volume: f64,
    ) -> Result<(), MemberServiceError> {
        if device_id != "system-default"
            || text.trim().is_empty()
            || !volume.is_finite()
            || !(0.0..=1.0).contains(&volume)
        {
            return Err(MemberServiceError::invalid("ai_audio_device_invalid"));
        }
        let text = text.to_owned();
        let voice_id = voice_id.to_owned();
        tokio::task::spawn_blocking(move || {
            let mut speech = tts::Tts::default()
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            if voice_id != DEFAULT_AI_VOICE_ID {
                let voice = speech
                    .voices()
                    .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?
                    .into_iter()
                    .find(|voice| voice.id() == voice_id)
                    .ok_or_else(|| MemberServiceError::invalid("ai_tts_voice_invalid"))?;
                speech
                    .set_voice(&voice)
                    .map_err(|_| MemberServiceError::invalid("ai_tts_voice_invalid"))?;
            }
            speech
                .set_volume(volume as f32)
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            speech
                .speak(text, false)
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))
        })
        .await
        .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?
    }

    async fn stop(&self) -> Result<(), MemberServiceError> {
        self.cancel_epoch.fetch_add(1, Ordering::AcqRel);
        Ok(())
    }
}

fn default_ai_voice() -> AiVoiceRecord {
    AiVoiceRecord {
        voice_id: DEFAULT_AI_VOICE_ID.to_owned(),
        display_name: DEFAULT_AI_VOICE_DISPLAY_NAME.to_owned(),
        is_available: true,
        is_default: true,
    }
}

#[cfg(target_os = "windows")]
fn windows_voices() -> Result<Vec<AiVoiceRecord>, MemberServiceError> {
    use windows::{
        core::BSTR,
        Win32::{
            Media::Speech::{ISpeechVoice, SpVoice},
            System::Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
            },
        },
    };

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .ok()
            .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
        let result = (|| {
            let speech: ISpeechVoice = CoCreateInstance(&SpVoice, None, CLSCTX_ALL)
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            let tokens = speech
                .GetVoices(&BSTR::new(), &BSTR::new())
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            let count = tokens
                .Count()
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            let mut voices = vec![default_ai_voice()];
            for index in 0..count {
                let token = tokens
                    .Item(index)
                    .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
                voices.push(AiVoiceRecord {
                    voice_id: token
                        .Id()
                        .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?
                        .to_string(),
                    display_name: token
                        .GetDescription(0)
                        .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?
                        .to_string(),
                    is_available: true,
                    is_default: false,
                });
            }
            Ok(voices)
        })();
        CoUninitialize();
        result
    }
}

#[cfg(target_os = "windows")]
fn windows_output_devices() -> Result<Vec<AiOutputDeviceRecord>, MemberServiceError> {
    use windows::{
        core::BSTR,
        Win32::{
            Media::Speech::{ISpeechVoice, SpVoice},
            System::Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
            },
        },
    };

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .ok()
            .map_err(|_| MemberServiceError::unavailable("ai_audio_device_unavailable"))?;
        let result = (|| {
            let voice: ISpeechVoice = CoCreateInstance(&SpVoice, None, CLSCTX_ALL)
                .map_err(|_| MemberServiceError::unavailable("ai_audio_device_unavailable"))?;
            let current_id = voice.AudioOutput().ok().and_then(|token| token.Id().ok());
            let tokens = voice
                .GetAudioOutputs(&BSTR::new(), &BSTR::new())
                .map_err(|_| MemberServiceError::unavailable("ai_audio_device_unavailable"))?;
            let count = tokens
                .Count()
                .map_err(|_| MemberServiceError::unavailable("ai_audio_device_unavailable"))?;
            (0..count)
                .map(|index| {
                    let token = tokens.Item(index).map_err(|_| {
                        MemberServiceError::unavailable("ai_audio_device_unavailable")
                    })?;
                    let id = token.Id().map_err(|_| {
                        MemberServiceError::unavailable("ai_audio_device_unavailable")
                    })?;
                    let description = token.GetDescription(0).map_err(|_| {
                        MemberServiceError::unavailable("ai_audio_device_unavailable")
                    })?;
                    Ok(AiOutputDeviceRecord {
                        device_id: id.to_string(),
                        display_name: description.to_string(),
                        is_available: true,
                        is_default: current_id.as_ref().is_some_and(|current| current == &id),
                    })
                })
                .collect()
        })();
        CoUninitialize();
        result
    }
}

#[cfg(target_os = "windows")]
fn windows_speak(
    text: &str,
    device_id: &str,
    voice_id: &str,
    volume: f64,
    cancel_epoch: &AtomicU64,
    epoch: u64,
) -> Result<(), MemberServiceError> {
    use windows::{
        core::BSTR,
        Win32::{
            Media::Speech::{
                ISpeechVoice, SVSFDefault, SVSFPurgeBeforeSpeak, SVSFlagsAsync, SpVoice,
            },
            System::Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
            },
        },
    };

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .ok()
            .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
        let result = (|| {
            let voice: ISpeechVoice = CoCreateInstance(&SpVoice, None, CLSCTX_ALL)
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            let tokens = voice
                .GetAudioOutputs(&BSTR::new(), &BSTR::new())
                .map_err(|_| MemberServiceError::unavailable("ai_audio_device_unavailable"))?;
            let count = tokens
                .Count()
                .map_err(|_| MemberServiceError::unavailable("ai_audio_device_unavailable"))?;
            let mut selected = None;
            for index in 0..count {
                let token = tokens
                    .Item(index)
                    .map_err(|_| MemberServiceError::unavailable("ai_audio_device_unavailable"))?;
                if token.Id().ok().is_some_and(|id| id == device_id) {
                    selected = Some(token);
                    break;
                }
            }
            let selected =
                selected.ok_or_else(|| MemberServiceError::invalid("ai_audio_device_invalid"))?;
            voice
                .putref_AudioOutput(&selected)
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            let voice_tokens = voice
                .GetVoices(&BSTR::new(), &BSTR::new())
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            let voice_count = voice_tokens
                .Count()
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            let mut selected_voice = if voice_id == DEFAULT_AI_VOICE_ID {
                voice.Voice().ok()
            } else {
                None
            };
            for index in 0..voice_count {
                let token = voice_tokens
                    .Item(index)
                    .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
                let id_matches = token.Id().ok().is_some_and(|id| id == voice_id);
                let is_preferred_default = voice_id == DEFAULT_AI_VOICE_ID
                    && token.GetDescription(0).ok().is_some_and(|name| {
                        name.to_string().to_ascii_lowercase().contains("huihui")
                    });
                if id_matches || is_preferred_default {
                    selected_voice = Some(token);
                    break;
                }
            }
            let selected_voice = selected_voice
                .ok_or_else(|| MemberServiceError::invalid("ai_tts_voice_invalid"))?;
            voice
                .putref_Voice(&selected_voice)
                .map_err(|_| MemberServiceError::invalid("ai_tts_voice_invalid"))?;
            voice
                .SetVolume((volume * 100.0).round() as i32)
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            voice
                .Speak(&BSTR::from(text), SVSFlagsAsync)
                .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?;
            loop {
                if cancel_epoch.load(Ordering::Acquire) != epoch {
                    let _ = voice.Speak(&BSTR::new(), SVSFPurgeBeforeSpeak);
                    break;
                }
                if voice
                    .WaitUntilDone(50)
                    .map_err(|_| MemberServiceError::unavailable("ai_tts_unavailable"))?
                    .as_bool()
                {
                    break;
                }
            }
            let _ = SVSFDefault;
            Ok(())
        })();
        CoUninitialize();
        result
    }
}
