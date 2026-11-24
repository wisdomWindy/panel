use super::{
    CaptainEffectConfig, GiftNotifyConfig, GiftRecord, GiftSoundPolicy, GiftSoundQueueItem,
    GiftStats, HighlightWindow, MediaDomainError, Qwen3TtsConfig, RecordingConfig, RecordingState,
    TtsConfig, TtsQueueItem, TtsTemplateKind, DANMAKU_HEAT_WINDOW_MS, HIGHLIGHT_AFTER_SECONDS,
    HIGHLIGHT_BEFORE_SECONDS, LATEST_GIFT_LIMIT, MAX_QWEN3_TTS_INSTRUCT_CHARS, QWEN3_TTS_LANGUAGES,
};

pub fn validate_volume(volume: f64) -> Result<(), MediaDomainError> {
    if volume.is_finite() && (0.0..=1.0).contains(&volume) {
        Ok(())
    } else {
        Err(MediaDomainError::InvalidField("volume"))
    }
}

pub fn validate_captain_effect_config(
    config: &CaptainEffectConfig,
) -> Result<(), MediaDomainError> {
    if config.preset_id.trim().is_empty() {
        return Err(MediaDomainError::InvalidField("presetId"));
    }
    if ![3, 5, 8, 10].contains(&config.duration_seconds) {
        return Err(MediaDomainError::InvalidField("durationSeconds"));
    }
    if !(320..=1920).contains(&config.window_width) {
        return Err(MediaDomainError::InvalidField("windowWidth"));
    }
    if !(180..=1080).contains(&config.window_height) {
        return Err(MediaDomainError::InvalidField("windowHeight"));
    }
    if !is_hex_color(&config.primary_color) || !is_hex_color(&config.secondary_color) {
        return Err(MediaDomainError::InvalidField("color"));
    }
    if ![5, 10, 30].contains(&config.combo_window_seconds) {
        return Err(MediaDomainError::InvalidField("comboWindowSeconds"));
    }
    for binding in [
        config.first_sound.as_ref(),
        config.renewal_sound.as_ref(),
        config.combo_sound.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        if binding.binding_id.trim().is_empty()
            || binding.file_name.trim().is_empty()
            || binding.source.as_os_str().is_empty()
        {
            return Err(MediaDomainError::InvalidField("soundBinding"));
        }
    }
    validate_volume(config.volume)
}

pub fn validate_gift_notify_config(config: &GiftNotifyConfig) -> Result<(), MediaDomainError> {
    if config.high_value_threshold_minor < 0 {
        return Err(MediaDomainError::InvalidField("highValueThresholdMinor"));
    }
    if ![5, 10, 30].contains(&config.combo_window_seconds) {
        return Err(MediaDomainError::InvalidField("comboWindowSeconds"));
    }
    if config.popup_duration_seconds == 0 {
        return Err(MediaDomainError::InvalidField("popupDurationSeconds"));
    }
    if config.is_enabled
        && !config.methods.system_notification
        && !config.methods.local_popup
        && !config.methods.sound
    {
        return Err(MediaDomainError::InvalidField("methods"));
    }
    Ok(())
}

pub fn validate_gift_sound_policy(config: &GiftSoundPolicy) -> Result<(), MediaDomainError> {
    validate_volume(config.global_volume)?;
    if config.queue_capacity_per_second == 0 {
        return Err(MediaDomainError::InvalidField("queueCapacityPerSecond"));
    }
    Ok(())
}

pub fn validate_qwen3_tts_config(config: &Qwen3TtsConfig) -> Result<(), MediaDomainError> {
    let url = config.base_url.trim();
    if url.is_empty() {
        return Err(MediaDomainError::InvalidField("qwen3Tts.baseUrl"));
    }
    let parsed = url
        .parse::<reqwest::Url>()
        .map_err(|_| MediaDomainError::InvalidField("qwen3Tts.baseUrl"))?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
        return Err(MediaDomainError::InvalidField("qwen3Tts.baseUrl"));
    }
    if !(500..=60_000).contains(&config.timeout_ms) {
        return Err(MediaDomainError::InvalidField("qwen3Tts.timeoutMs"));
    }
    if !QWEN3_TTS_LANGUAGES.contains(&config.language.as_str()) {
        return Err(MediaDomainError::InvalidField("qwen3Tts.language"));
    }
    for (field, value) in [
        ("qwen3Tts.instruct", config.instruct.as_str()),
        (
            "qwen3Tts.aiInstruct",
            config.ai_instruct.as_deref().unwrap_or_default(),
        ),
    ] {
        if value.chars().count() > MAX_QWEN3_TTS_INSTRUCT_CHARS {
            return Err(MediaDomainError::InvalidField(field));
        }
    }
    Ok(())
}

pub fn validate_tts_config(config: &TtsConfig) -> Result<(), MediaDomainError> {
    validate_volume(config.volume)?;
    if !config.rate.is_finite() || config.rate <= 0.0 {
        return Err(MediaDomainError::InvalidField("rate"));
    }
    if config.event_kinds.is_empty() || config.identity_kinds.is_empty() {
        return Err(MediaDomainError::InvalidField("filters"));
    }
    validate_tts_template(TtsTemplateKind::Danmu, &config.templates.danmu)?;
    validate_tts_template(TtsTemplateKind::Gift, &config.templates.gift)?;
    validate_tts_template(TtsTemplateKind::Captain, &config.templates.captain)
}

/// Live room ids are numeric; validating here keeps them out of file names and
/// URLs as anything other than digits.
pub fn validate_recording_room_id(room_id: &str) -> Result<&str, MediaDomainError> {
    let trimmed = room_id.trim();
    if trimmed.is_empty()
        || trimmed.len() > 20
        || !trimmed.chars().all(|character| character.is_ascii_digit())
    {
        return Err(MediaDomainError::InvalidField("recordingRoomId"));
    }
    Ok(trimmed)
}

pub fn validate_recording_config(config: &RecordingConfig) -> Result<(), MediaDomainError> {
    if config.segmentation_enabled && config.segment_duration_seconds == 0 {
        return Err(MediaDomainError::InvalidField("segmentDurationSeconds"));
    }
    if config.danmaku_threshold == 0
        || config.highlight_before_seconds != HIGHLIGHT_BEFORE_SECONDS as u8
        || config.highlight_after_seconds != HIGHLIGHT_AFTER_SECONDS as u8
        || config.quota_bytes == 0
        || config.waterline_bytes == 0
        || config.waterline_bytes >= config.quota_bytes
        || config.shortcut.trim().is_empty()
    {
        return Err(MediaDomainError::InvalidField("recordingConfig"));
    }
    if let Some(directory) = &config.directory {
        if directory.grant_id.trim().is_empty() || directory.display_path.trim().is_empty() {
            return Err(MediaDomainError::InvalidField("directory"));
        }
    }
    Ok(())
}

pub fn validate_tts_template(
    kind: TtsTemplateKind,
    template: &str,
) -> Result<(), MediaDomainError> {
    if template.trim().is_empty() || template.chars().any(char::is_control) {
        return Err(MediaDomainError::InvalidField("template"));
    }
    let allowed = match kind {
        TtsTemplateKind::Danmu => &["用户", "内容"][..],
        TtsTemplateKind::Gift => &["用户", "礼物", "数量"][..],
        TtsTemplateKind::Captain => &["用户", "等级"][..],
    };
    let mut token = None::<String>;
    for character in template.chars() {
        match character {
            '{' if token.is_none() => token = Some(String::new()),
            '{' => return Err(MediaDomainError::InvalidField("template")),
            '}' => {
                let value = token
                    .take()
                    .ok_or(MediaDomainError::InvalidField("template"))?;
                if !allowed.contains(&value.as_str()) {
                    return Err(MediaDomainError::InvalidField("template"));
                }
            }
            _ => {
                if let Some(value) = token.as_mut() {
                    value.push(character);
                }
            }
        }
    }
    if token.is_some() {
        return Err(MediaDomainError::InvalidField("template"));
    }
    Ok(())
}

pub fn reduce_gift_stats(
    current: &GiftStats,
    event: GiftRecord,
) -> Result<GiftStats, MediaDomainError> {
    if current.seen_event_ids.contains(&event.event_id) {
        return Ok(current.clone());
    }
    if event.event_id.trim().is_empty()
        || event.user_uid.trim().is_empty()
        || event.gift_id.trim().is_empty()
        || event.gift_name.trim().is_empty()
        || event.quantity == 0
        || event.value_minor < 0
        || event.occurred_at <= 0
    {
        return Err(MediaDomainError::InvalidField("gift"));
    }

    let mut next = current.clone();
    next.total_value_minor = next
        .total_value_minor
        .checked_add(event.value_minor)
        .ok_or(MediaDomainError::InvalidField("totalValueMinor"))?;
    next.gift_count = next
        .gift_count
        .checked_add(u64::from(event.quantity))
        .ok_or(MediaDomainError::InvalidField("giftCount"))?;
    next.donor_uids.insert(event.user_uid.clone());
    next.seen_event_ids.insert(event.event_id.clone());
    next.latest.push_back(event);
    while next.latest.len() > LATEST_GIFT_LIMIT {
        next.latest.pop_front();
    }
    Ok(next)
}

pub fn enqueue_tts(
    current: &[TtsQueueItem],
    item: TtsQueueItem,
    capacity: usize,
) -> Vec<TtsQueueItem> {
    if capacity == 0 || current.iter().any(|queued| queued.id == item.id) {
        return current.iter().take(capacity).cloned().collect();
    }
    let mut next = current.to_vec();
    next.push(item);
    while next.len() > capacity {
        let minimum_priority = next.iter().map(|queued| queued.priority).min().unwrap();
        let remove_index = next
            .iter()
            .enumerate()
            .filter(|(_, queued)| queued.priority == minimum_priority)
            .min_by_key(|(_, queued)| queued.sequence)
            .map(|(index, _)| index)
            .unwrap();
        next.remove(remove_index);
    }
    next.sort_by(|left, right| {
        right
            .priority
            .cmp(&left.priority)
            .then(left.sequence.cmp(&right.sequence))
    });
    next
}

pub fn enqueue_gift_sound(
    current: &[GiftSoundQueueItem],
    item: GiftSoundQueueItem,
    capacity: usize,
) -> Vec<GiftSoundQueueItem> {
    if capacity == 0
        || current
            .iter()
            .any(|queued| queued.event_id == item.event_id)
    {
        return current.iter().take(capacity).cloned().collect();
    }
    let mut next = current.to_vec();
    next.push(item);
    while next.len() > capacity {
        let remove_index = next
            .iter()
            .enumerate()
            .filter(|(_, queued)| !queued.is_high_value)
            .min_by_key(|(_, queued)| queued.sequence)
            .or_else(|| {
                next.iter()
                    .enumerate()
                    .min_by_key(|(_, queued)| queued.sequence)
            })
            .map(|(index, _)| index)
            .unwrap();
        next.remove(remove_index);
    }
    next
}

pub fn should_merge_combo(previous: &GiftRecord, current: &GiftRecord, window_seconds: u8) -> bool {
    previous.user_uid == current.user_uid
        && previous.gift_id == current.gift_id
        && current.occurred_at >= previous.occurred_at
        && current.occurred_at - previous.occurred_at <= i64::from(window_seconds) * 1_000
}

pub fn next_recording_state(
    current: RecordingState,
    requested: RecordingState,
) -> Result<RecordingState, MediaDomainError> {
    let is_allowed = current == requested
        || requested == RecordingState::Error
        || matches!(
            (current, requested),
            (RecordingState::Idle, RecordingState::Starting)
                | (RecordingState::Starting, RecordingState::Recording)
                | (RecordingState::Recording, RecordingState::Stopping)
                | (RecordingState::Stopping, RecordingState::Finalizing)
                | (RecordingState::Finalizing, RecordingState::Idle)
                | (RecordingState::Error, RecordingState::Idle)
        );
    if is_allowed {
        Ok(requested)
    } else {
        Err(MediaDomainError::InvalidTransition)
    }
}

pub const fn should_stop_for_disk(remaining_bytes: u64, waterline_bytes: u64) -> bool {
    remaining_bytes < waterline_bytes
}

pub fn is_danmaku_surge(timestamps: &[i64], now: i64, threshold: u32) -> bool {
    if threshold == 0 || now <= 0 {
        return false;
    }
    let cutoff = now.saturating_sub(DANMAKU_HEAT_WINDOW_MS);
    timestamps
        .iter()
        .filter(|timestamp| **timestamp >= cutoff && **timestamp <= now)
        .count()
        >= threshold as usize
}

pub fn highlight_window(position_seconds: u64, duration_seconds: Option<u64>) -> HighlightWindow {
    let start_seconds = position_seconds.saturating_sub(HIGHLIGHT_BEFORE_SECONDS);
    let requested_end = position_seconds.saturating_add(HIGHLIGHT_AFTER_SECONDS);
    HighlightWindow {
        start_seconds,
        end_seconds: duration_seconds.map_or(requested_end, |duration| requested_end.min(duration)),
    }
}

pub fn validate_recording_filename_stem(value: &str) -> Result<String, MediaDomainError> {
    let normalized = value.trim();
    if normalized.is_empty()
        || normalized.chars().any(char::is_control)
        || normalized
            .chars()
            .any(|character| "\\/:*?\"<>|".contains(character))
        || normalized.ends_with(['.', ' '])
        || is_windows_reserved_name(normalized)
    {
        return Err(MediaDomainError::InvalidField("fileNameStem"));
    }
    Ok(normalized.to_owned())
}

fn is_hex_color(value: &str) -> bool {
    value.len() == 7
        && value.starts_with('#')
        && value[1..].bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn is_windows_reserved_name(value: &str) -> bool {
    let stem = value
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || stem
            .strip_prefix("COM")
            .or_else(|| stem.strip_prefix("LPT"))
            .is_some_and(|suffix| {
                matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
            })
}

/// True when a file name looks like one the recorder wrote:
/// `<room>-<stamp>.<ext>` or a continuation segment `<room>-<stamp>-partN.<ext>`.
///
/// Cleanup relies on this to never touch a video the operator put in the folder.
pub fn is_app_recording_file(file_name: &str) -> bool {
    let Some((stem, extension)) = file_name.rsplit_once('.') else {
        return false;
    };
    if !matches!(
        extension.to_ascii_lowercase().as_str(),
        "mp4" | "flv" | "mkv" | "ts"
    ) {
        return false;
    }
    let mut parts = stem.split('-');
    let Some(room) = parts.next() else {
        return false;
    };
    if room.is_empty() || !room.chars().all(|character| character.is_ascii_digit()) {
        return false;
    }
    let Some(stamp) = parts.next() else {
        return false;
    };
    if stamp.is_empty() || !stamp.chars().all(|character| character.is_ascii_digit()) {
        return false;
    }
    match parts.next() {
        None => true,
        Some(segment) => segment
            .strip_prefix("part")
            .is_some_and(|index| !index.is_empty() && index.chars().all(|c| c.is_ascii_digit())),
    }
}

#[cfg(test)]
mod cleanup_file_tests {
    use super::is_app_recording_file;

    #[test]
    fn recognises_files_the_recorder_wrote() {
        for name in [
            "1869218529-1795428633962.mp4",
            "27183290-1795497559043.mp4",
            "6-1795497585364-part2.mp4",
            "6-1795497585364-part12.flv",
            "1-2.mkv",
            "1-2.ts",
        ] {
            assert!(is_app_recording_file(name), "{name} should be recognised");
        }
    }

    #[test]
    fn leaves_operator_files_alone() {
        for name in [
            "my-video.mp4",
            "会议录像.mp4",
            "room-abc.mp4",
            "1869218529.mp4",
            "1869218529-1795-part.mp4",
            "1869218529-1795.part2.mp4",
            "1869218529-1795.mp4.txt",
            "1869218529-1795.avi",
        ] {
            assert!(!is_app_recording_file(name), "{name} must be ignored");
        }
    }
}