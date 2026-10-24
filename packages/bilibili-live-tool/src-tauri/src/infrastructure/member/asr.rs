use std::path::{Path, PathBuf};

use futures_util::StreamExt;
use sha1::{Digest, Sha1};
use tokio::{fs, io::AsyncWriteExt};

use super::asr_runtime::AsrSession;
use crate::application::member::{
    AsrModelRecord, AsrProvider, AsrState, AsrStatusRecord, MemberEventSink, MemberServiceError,
    NoopMemberEventSink,
};
use std::sync::{Arc, Mutex};

struct CatalogEntry {
    id: &'static str,
    name: &'static str,
    file_name: &'static str,
    size_bytes: u64,
    sha1: &'static str,
}

const MODEL_SOURCE: &str = "whisper.cpp / Hugging Face";
const MODEL_LICENSE: &str = "MIT";
const MODEL_BASE_URLS: &[&str] = &[
    "https://huggingface.co/ggerganov/whisper.cpp/resolve/main",
    "https://hf-mirror.com/ggerganov/whisper.cpp/resolve/main",
];
const CATALOG: &[CatalogEntry] = &[
    CatalogEntry {
        id: "tiny",
        name: "Whisper Tiny（轻量）",
        file_name: "ggml-tiny.bin",
        size_bytes: 77_691_713,
        sha1: "bd577a113a864445d4c299885e0cb97d4ba92b5f",
    },
    CatalogEntry {
        id: "base",
        name: "Whisper Base（标准）",
        file_name: "ggml-base.bin",
        size_bytes: 147_951_465,
        sha1: "465707469ff3a37a2b9b8d8f89f2f99de7299dac",
    },
    CatalogEntry {
        id: "small",
        name: "Whisper Small（高精度）",
        file_name: "ggml-small.bin",
        size_bytes: 487_601_967,
        sha1: "55356645c2b361a969dfd0ef2c5a50d530afd8d5",
    },
];

pub struct WhisperAsrProvider {
    model_directory: PathBuf,
    client: reqwest::Client,
    model_sources: Vec<String>,
    session: tokio::sync::Mutex<Option<AsrSession>>,
    runtime_status: Arc<Mutex<Option<AsrStatusRecord>>>,
    events: Arc<dyn MemberEventSink>,
}

impl WhisperAsrProvider {
    pub fn new(model_directory: PathBuf) -> Self {
        Self {
            session: tokio::sync::Mutex::new(None),
            runtime_status: Arc::new(Mutex::new(None)),
            events: Arc::new(NoopMemberEventSink),
            model_directory,
            client: reqwest::Client::builder()
                .connect_timeout(std::time::Duration::from_secs(8))
                .timeout(std::time::Duration::from_secs(1800))
                .build()
                .expect("ASR HTTP client configuration is valid"),
            model_sources: MODEL_BASE_URLS
                .iter()
                .map(|source| (*source).to_owned())
                .collect(),
        }
    }

    fn catalog_entry(model_id: &str) -> Result<&'static CatalogEntry, MemberServiceError> {
        CATALOG
            .iter()
            .find(|entry| entry.id == model_id)
            .ok_or_else(|| MemberServiceError::invalid("asr_model_unknown"))
    }

    pub fn with_event_sink(mut self, events: Arc<dyn MemberEventSink>) -> Self {
        self.events = events;
        self
    }

    fn model_path(&self, entry: &CatalogEntry) -> PathBuf {
        self.model_directory.join(entry.file_name)
    }

    fn checksum_marker_path(&self, entry: &CatalogEntry) -> PathBuf {
        self.model_path(entry).with_extension("bin.sha1")
    }

    async fn is_installed(&self, entry: &CatalogEntry) -> bool {
        let has_expected_size = fs::metadata(self.model_path(entry))
            .await
            .is_ok_and(|metadata| metadata.is_file() && metadata.len() == entry.size_bytes);
        if !has_expected_size {
            return false;
        }
        fs::read_to_string(self.checksum_marker_path(entry))
            .await
            .is_ok_and(|checksum| checksum.trim() == entry.sha1)
    }

    async fn record(&self, entry: &CatalogEntry) -> AsrModelRecord {
        AsrModelRecord {
            model_id: entry.id.to_owned(),
            model_name: entry.name.to_owned(),
            size_bytes: entry.size_bytes,
            license: MODEL_LICENSE.to_owned(),
            source_label: MODEL_SOURCE.to_owned(),
            is_installed: self.is_installed(entry).await,
        }
    }

    async fn remove_partial(path: &Path) {
        if let Err(error) = fs::remove_file(path).await {
            if error.kind() != std::io::ErrorKind::NotFound {
                log::warn!("failed to remove partial ASR model: {error}");
            }
        }
    }
}

#[async_trait::async_trait]
impl AsrProvider for WhisperAsrProvider {
    fn runtime_status(&self) -> Option<AsrStatusRecord> {
        self.runtime_status
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .clone()
    }
    async fn list_models(&self) -> Result<Vec<AsrModelRecord>, MemberServiceError> {
        let mut models = Vec::with_capacity(CATALOG.len());
        for entry in CATALOG {
            models.push(self.record(entry).await);
        }
        Ok(models)
    }

    async fn download_model(&self, model_id: &str) -> Result<AsrModelRecord, MemberServiceError> {
        let entry = Self::catalog_entry(model_id)?;
        if self.is_installed(entry).await {
            return Ok(self.record(entry).await);
        }

        fs::create_dir_all(&self.model_directory)
            .await
            .map_err(|_| MemberServiceError::unavailable("asr_model_storage_failed"))?;
        let final_path = self.model_path(entry);
        let partial_path = final_path.with_extension("bin.part");
        let marker_path = self.checksum_marker_path(entry);
        let marker_partial_path = marker_path.with_extension("sha1.part");
        Self::remove_partial(&partial_path).await;
        Self::remove_partial(&marker_partial_path).await;

        let mut last_error = MemberServiceError::unavailable("asr_model_download_failed");
        for source in &self.model_sources {
            let result = async {
                let url = format!("{source}/{}", entry.file_name);
                let response = self
                    .client
                    .get(url)
                    .send()
                    .await
                    .and_then(reqwest::Response::error_for_status)
                    .map_err(|_| MemberServiceError::unavailable("asr_model_download_failed"))?;
                let mut file = fs::File::create(&partial_path)
                    .await
                    .map_err(|_| MemberServiceError::unavailable("asr_model_storage_failed"))?;
                let mut hasher = Sha1::new();
                let mut stream = response.bytes_stream();
                while let Some(chunk) =
                    tokio::time::timeout(std::time::Duration::from_secs(30), stream.next())
                        .await
                        .map_err(|_| MemberServiceError::unavailable("asr_model_download_failed"))?
                {
                    let chunk = chunk.map_err(|_| {
                        MemberServiceError::unavailable("asr_model_download_failed")
                    })?;
                    file.write_all(&chunk)
                        .await
                        .map_err(|_| MemberServiceError::unavailable("asr_model_storage_failed"))?;
                    hasher.update(&chunk);
                }
                file.flush()
                    .await
                    .map_err(|_| MemberServiceError::unavailable("asr_model_storage_failed"))?;
                drop(file);

                if fs::metadata(&partial_path)
                    .await
                    .map_err(|_| MemberServiceError::unavailable("asr_model_storage_failed"))?
                    .len()
                    != entry.size_bytes
                    || format!("{:x}", hasher.finalize()) != entry.sha1
                {
                    return Err(MemberServiceError::unavailable(
                        "asr_model_verification_failed",
                    ));
                }
                if fs::metadata(&final_path).await.is_ok() {
                    fs::remove_file(&final_path)
                        .await
                        .map_err(|_| MemberServiceError::unavailable("asr_model_storage_failed"))?;
                }
                fs::write(&marker_partial_path, entry.sha1)
                    .await
                    .map_err(|_| MemberServiceError::unavailable("asr_model_storage_failed"))?;
                fs::rename(&partial_path, &final_path)
                    .await
                    .map_err(|_| MemberServiceError::unavailable("asr_model_storage_failed"))?;
                if fs::metadata(&marker_path).await.is_ok() {
                    fs::remove_file(&marker_path)
                        .await
                        .map_err(|_| MemberServiceError::unavailable("asr_model_storage_failed"))?;
                }
                if fs::rename(&marker_partial_path, &marker_path)
                    .await
                    .is_err()
                {
                    Self::remove_partial(&final_path).await;
                    return Err(MemberServiceError::unavailable("asr_model_storage_failed"));
                }
                Ok(self.record(entry).await)
            }
            .await;

            match result {
                Ok(record) => return Ok(record),
                Err(error) => {
                    Self::remove_partial(&partial_path).await;
                    Self::remove_partial(&marker_partial_path).await;
                    if error.code == "asr_model_storage_failed" {
                        return Err(error);
                    }
                    log::warn!(
                        "ASR model {} download from {} failed: {}",
                        entry.id,
                        source,
                        error.code
                    );
                    last_error = error;
                }
            }
        }
        Err(last_error)
    }

    async fn start(&self) -> Result<(), MemberServiceError> {
        let mut session = self.session.lock().await;
        if session.is_some() {
            if self
                .runtime_status()
                .is_some_and(|status| status.state == AsrState::Listening)
            {
                return Ok(());
            }
            if let Some(previous) = session.take() {
                previous.stop().await;
            }
        }
        let mut selected = None;
        for entry in CATALOG {
            if self.is_installed(entry).await {
                selected = Some(entry);
                break;
            }
        }
        let entry = selected.ok_or_else(|| MemberServiceError::invalid("asr_model_missing"))?;
        *self
            .runtime_status
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = None;
        let next = AsrSession::start(
            self.model_path(entry),
            entry.name.to_owned(),
            self.runtime_status.clone(),
            self.events.clone(),
        )
        .await?;
        *session = Some(next);
        Ok(())
    }

    async fn stop(&self) -> Result<(), MemberServiceError> {
        let mut session = self.session.lock().await;
        if let Some(previous) = session.take() {
            previous.stop().await;
        }
        *self
            .runtime_status
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn failed_source_falls_back_and_corrupt_download_is_not_installed() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            for status in ["503 Service Unavailable", "200 OK"] {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = [0; 2048];
                let bytes_read = socket.read(&mut request).await.unwrap();
                assert!(bytes_read > 0);
                socket.write_all(format!("HTTP/1.1 {status}\r\nContent-Length: 3\r\nConnection: close\r\n\r\nbad").as_bytes()).await.unwrap();
            }
        });
        let directory = tempfile::tempdir().unwrap();
        let mut provider = WhisperAsrProvider::new(directory.path().to_owned());
        provider.model_sources = vec![
            format!("http://{address}/primary"),
            format!("http://{address}/fallback"),
        ];
        assert_eq!(
            provider.download_model("tiny").await.unwrap_err().code,
            "asr_model_verification_failed"
        );
        server.await.unwrap();
        assert!(!provider.list_models().await.unwrap()[0].is_installed);
        assert!(!directory.path().join("ggml-tiny.bin.part").exists());
        assert!(!directory.path().join("ggml-tiny.bin.sha1").exists());
    }

    #[tokio::test]
    #[ignore = "downloads the 78 MB Tiny model from the network"]
    async fn live_tiny_model_download_falls_back_and_installs() {
        let directory = tempfile::tempdir().unwrap();
        let provider = WhisperAsrProvider::new(directory.path().to_owned());
        assert!(provider.download_model("tiny").await.unwrap().is_installed);
        assert!(provider.list_models().await.unwrap()[0].is_installed);
        assert!(!directory.path().join("ggml-tiny.bin.part").exists());
    }
}
