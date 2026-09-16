use std::{collections::HashMap, io::SeekFrom, path::PathBuf, sync::RwLock};

use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::application::media::{
    ByteRange, MediaIoError, PreviewPort, PreviewRead, PreviewSession,
};

const MAX_RANGE_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Clone)]
struct PreviewEntry {
    file: PathBuf,
    owner: String,
    expires_at: i64,
    sequence: u64,
}

pub struct PreviewRegistry {
    capacity: usize,
    next_sequence: std::sync::atomic::AtomicU64,
    sessions: RwLock<HashMap<String, PreviewEntry>>,
}

impl PreviewRegistry {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            next_sequence: std::sync::atomic::AtomicU64::new(0),
            sessions: RwLock::new(HashMap::new()),
        }
    }

    fn get_entry(
        &self,
        token: &str,
        owner: &str,
        now_ms: i64,
    ) -> Result<PreviewEntry, MediaIoError> {
        let entry = self
            .sessions
            .read()
            .map_err(|_| MediaIoError::unavailable("preview registry is unavailable"))?
            .get(token)
            .cloned()
            .ok_or_else(|| MediaIoError::invalid("preview session is unknown"))?;
        if entry.owner != owner {
            return Err(MediaIoError::permission(
                "preview session owner does not match",
            ));
        }
        if now_ms > entry.expires_at {
            self.sessions
                .write()
                .map_err(|_| MediaIoError::unavailable("preview registry is unavailable"))?
                .remove(token);
            return Err(MediaIoError::invalid("preview session has expired"));
        }
        Ok(entry)
    }
}

#[async_trait::async_trait]
impl PreviewPort for PreviewRegistry {
    fn issue(
        &self,
        file: PathBuf,
        owner: &str,
        now_ms: i64,
        ttl_ms: i64,
    ) -> Result<PreviewSession, MediaIoError> {
        if self.capacity == 0 || owner.trim().is_empty() || ttl_ms <= 0 {
            return Err(MediaIoError::invalid("invalid preview session request"));
        }
        let metadata = file
            .metadata()
            .map_err(|_| MediaIoError::unavailable("preview file is unavailable"))?;
        if !metadata.is_file() || metadata.len() == 0 {
            return Err(MediaIoError::invalid(
                "preview source is not a non-empty file",
            ));
        }
        let file = file
            .canonicalize()
            .map_err(|_| MediaIoError::unavailable("preview file is unavailable"))?;
        let expires_at = now_ms
            .checked_add(ttl_ms)
            .ok_or_else(|| MediaIoError::invalid("preview expiry is out of range"))?;
        let sequence = self
            .next_sequence
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let token = uuid::Uuid::new_v4().to_string();
        let entry = PreviewEntry {
            file,
            owner: owner.to_owned(),
            expires_at,
            sequence,
        };
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| MediaIoError::unavailable("preview registry is unavailable"))?;
        if sessions.len() >= self.capacity {
            if let Some(oldest) = sessions
                .iter()
                .min_by_key(|(_, entry)| (entry.expires_at, entry.sequence))
                .map(|(token, _)| token.clone())
            {
                sessions.remove(&oldest);
            }
        }
        sessions.insert(token.clone(), entry);
        Ok(PreviewSession {
            url: format!("media-preview://localhost/{token}"),
            token,
            expires_at,
        })
    }

    async fn read(
        &self,
        token: &str,
        owner: &str,
        range: Option<ByteRange>,
        now_ms: i64,
    ) -> Result<PreviewRead, MediaIoError> {
        let entry = self.get_entry(token, owner, now_ms)?;
        let mut file = tokio::fs::File::open(entry.file)
            .await
            .map_err(|_| MediaIoError::unavailable("preview file is unavailable"))?;
        let total_size = file
            .metadata()
            .await
            .map_err(|_| MediaIoError::unavailable("preview file is unavailable"))?
            .len();
        let requested = range.unwrap_or(ByteRange {
            start: 0,
            end_inclusive: total_size.saturating_sub(1),
        });
        if requested.start >= total_size {
            return Err(MediaIoError::invalid("preview range is outside the file"));
        }
        let range = ByteRange {
            start: requested.start,
            end_inclusive: requested.end_inclusive.min(total_size.saturating_sub(1)),
        };
        let length = range
            .end_inclusive
            .checked_sub(range.start)
            .and_then(|length| length.checked_add(1))
            .filter(|length| *length <= MAX_RANGE_BYTES)
            .ok_or_else(|| MediaIoError::invalid("preview range is invalid"))?;
        let length = usize::try_from(length)
            .map_err(|_| MediaIoError::invalid("preview range is too large"))?;
        file.seek(SeekFrom::Start(range.start))
            .await
            .map_err(|_| MediaIoError::operation("preview seek failed", true))?;
        let mut bytes = vec![0; length];
        file.read_exact(&mut bytes)
            .await
            .map_err(|_| MediaIoError::operation("preview read failed", true))?;
        Ok(PreviewRead {
            bytes,
            total_size,
            range,
        })
    }

    fn revoke(&self, token: &str, owner: &str) -> Result<bool, MediaIoError> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|_| MediaIoError::unavailable("preview registry is unavailable"))?;
        if sessions
            .get(token)
            .is_some_and(|entry| entry.owner != owner)
        {
            return Err(MediaIoError::permission(
                "preview session owner does not match",
            ));
        }
        Ok(sessions.remove(token).is_some())
    }
}
