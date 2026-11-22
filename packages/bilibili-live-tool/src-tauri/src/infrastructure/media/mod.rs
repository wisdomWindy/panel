mod audio;
mod ffmpeg;
mod ffmpeg_installer;
mod ffmpeg_locator;
mod files;
mod hotkey;
mod notification;
mod play_url;
mod preview;
mod qwen_tts;
mod repository;
mod speech;
mod window;

pub use audio::{RodioAudioOutput, UnavailableAudioOutput};
pub use ffmpeg::{LocalFfmpegProcessPort, UnavailableRecorderProcessPort};
pub use ffmpeg_installer::FfmpegInstaller;
pub use ffmpeg_locator::AppFfmpegLocator;
pub use play_url::{parse_play_url_response, quality_qn, validate_room_id, BilibiliPlayUrlResolver};

pub use crate::application::media::{
    GiftSoundBindingRecord, HighlightRecord, HighlightTriggerType, RecordingItemState,
    RecordingRecord,
};
pub use files::{
    compensated_delete, compensated_move, CompensatedDeleteError, CompensatedMoveError,
    GiftSoundDocument, GiftSoundFileStore, MediaFileError, PortableGiftSoundBinding,
};
pub use hotkey::{TauriHotkeyPort, UnavailableHotkeyPort};
pub use notification::{TauriNotificationPort, UnavailableNotificationPort};
pub use preview::PreviewRegistry;
pub use qwen_tts::{
    compose_instruct, Qwen3DownloadState, Qwen3DownloadStatus, Qwen3ModelCatalog, Qwen3ModelEntry,
    Qwen3TtsService, Qwen3TtsSpeechEngine, QWEN3_TTS_CONFIG_KEY,
};
pub use repository::{MediaRepositoryError, SqliteMediaRepository};
pub use speech::{SystemSpeechEngine, UnavailableSpeechEngine};
pub use window::{TauriEffectWindowPort, UnavailableEffectWindowPort};

#[cfg(test)]
mod tests;
