mod audio;
mod ffmpeg;
mod files;
mod hotkey;
mod notification;
mod preview;
mod repository;
mod speech;
mod window;

pub use audio::{RodioAudioOutput, UnavailableAudioOutput};
pub use ffmpeg::{LocalFfmpegProcessPort, UnavailableRecorderProcessPort};

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
pub use repository::{MediaRepositoryError, SqliteMediaRepository};
pub use speech::{SystemSpeechEngine, UnavailableSpeechEngine};
pub use window::{TauriEffectWindowPort, UnavailableEffectWindowPort};

#[cfg(test)]
mod tests;
