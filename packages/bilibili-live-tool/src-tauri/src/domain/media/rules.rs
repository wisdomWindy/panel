use super::{
    CaptainEffectConfig, GiftNotifyConfig, GiftRecord, GiftSoundPolicy, GiftSoundQueueItem,
    GiftStats, HighlightWindow, MediaDomainError, RecordingConfig, RecordingState, TtsConfig,
    TtsQueueItem, TtsTemplateKind, DANMAKU_HEAT_WINDOW_MS, HIGHLIGHT_AFTER_SECONDS,
    HIGHLIGHT_BEFORE_SECONDS, LATEST_GIFT_LIMIT,
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
