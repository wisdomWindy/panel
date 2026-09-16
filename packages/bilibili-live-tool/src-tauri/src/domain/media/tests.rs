use super::*;

fn gift(id: usize, uid: &str, gift_id: &str, value_minor: i64) -> GiftRecord {
    GiftRecord {
        event_id: format!("event-{id}"),
        room_id: "1".into(),
        user_uid: uid.into(),
        username: format!("user-{uid}"),
        gift_id: gift_id.into(),
        gift_name: "辣条".into(),
        quantity: 1,
        value_minor,
        occurred_at: id as i64 * 1_000,
    }
}

#[test]
fn validates_effect_and_gift_configuration_without_clamping() {
    let effect = CaptainEffectConfig {
        preset_id: "gold".into(),
        duration_seconds: 5,
        position: MediaPosition::TopRight,
        primary_color: "#FFD700".into(),
        secondary_color: "#FFA500".into(),
        first_sound: None,
        renewal_sound: None,
        combo_sound: None,
        volume: 0.7,
        combo_enabled: true,
        combo_window_seconds: 10,
    };
    assert_eq!(validate_captain_effect_config(&effect), Ok(()));
    assert!(validate_volume(f64::NAN).is_err());
    assert!(validate_volume(1.01).is_err());

    let mut invalid = effect;
    invalid.duration_seconds = 6;
    assert!(validate_captain_effect_config(&invalid).is_err());

    let notify = GiftNotifyConfig {
        is_enabled: true,
        high_value_threshold_minor: 10_000,
        methods: GiftNotifyMethods {
            system_notification: false,
            local_popup: false,
            sound: false,
        },
        combo_enabled: true,
        combo_window_seconds: 10,
        popup_position: MediaPosition::Center,
        popup_duration_seconds: 5,
    };
    assert!(validate_gift_notify_config(&notify).is_err());
}

#[test]
fn gift_stats_are_idempotent_exact_and_keep_latest_twenty() {
    let mut stats = GiftStats::default();
    for id in 1..=21 {
        stats = reduce_gift_stats(
            &stats,
            gift(id, if id % 2 == 0 { "2" } else { "1" }, "31036", 100),
        )
        .unwrap();
    }
    stats = reduce_gift_stats(&stats, gift(21, "1", "31036", 100)).unwrap();

    assert_eq!(stats.total_value_minor, 2_100);
    assert_eq!(stats.gift_count, 21);
    assert_eq!(stats.donor_uids.len(), 2);
    assert_eq!(stats.latest.len(), 20);
    assert_eq!(stats.latest.front().unwrap().event_id, "event-2");
    assert_eq!(stats.latest.back().unwrap().event_id, "event-21");
}

#[test]
fn validates_only_the_prd_template_placeholders_and_preserves_spaces() {
    assert_eq!(
        validate_tts_template(TtsTemplateKind::Danmu, " {用户}说：{内容} "),
        Ok(())
    );
    assert_eq!(
        validate_tts_template(TtsTemplateKind::Gift, "{用户}送出{礼物}×{数量}"),
        Ok(())
    );
    assert_eq!(
        validate_tts_template(TtsTemplateKind::Captain, "{用户}开通{等级}"),
        Ok(())
    );
    assert!(validate_tts_template(TtsTemplateKind::Danmu, "   ").is_err());
    assert!(validate_tts_template(TtsTemplateKind::Danmu, "{用户}\n{内容}").is_err());
    assert!(validate_tts_template(TtsTemplateKind::Gift, "{用户}{金额}").is_err());
    assert!(validate_tts_template(TtsTemplateKind::Captain, "{用户").is_err());
}

#[test]
fn tts_queue_is_priority_ordered_fifo_and_drops_oldest_low_priority() {
    let ordinary = TtsQueueItem {
        id: "o1".into(),
        text: "ordinary".into(),
        priority: TtsPriority::Ordinary,
        sequence: 1,
    };
    let admin = TtsQueueItem {
        id: "a1".into(),
        text: "admin".into(),
        priority: TtsPriority::Admin,
        sequence: 2,
    };
    let gift = TtsQueueItem {
        id: "g1".into(),
        text: "gift".into(),
        priority: TtsPriority::Gift,
        sequence: 3,
    };
    let captain = TtsQueueItem {
        id: "c1".into(),
        text: "captain".into(),
        priority: TtsPriority::Captain,
        sequence: 4,
    };

    let queue = enqueue_tts(&[ordinary, admin, gift], captain, 3);
    assert_eq!(
        queue
            .iter()
            .map(|item| item.id.as_str())
            .collect::<Vec<_>>(),
        ["c1", "g1", "a1"]
    );

    let next = TtsQueueItem {
        id: "c2".into(),
        text: "captain 2".into(),
        priority: TtsPriority::Captain,
        sequence: 5,
    };
    let queue = enqueue_tts(&queue, next, 4);
    assert_eq!(
        queue
            .iter()
            .map(|item| item.id.as_str())
            .collect::<Vec<_>>(),
        ["c1", "c2", "g1", "a1"]
    );
}

#[test]
fn gift_sound_queue_drops_the_oldest_low_priority_item() {
    let queue = [
        GiftSoundQueueItem {
            event_id: "low-old".into(),
            is_high_value: false,
            sequence: 1,
        },
        GiftSoundQueueItem {
            event_id: "high".into(),
            is_high_value: true,
            sequence: 2,
        },
        GiftSoundQueueItem {
            event_id: "low-new".into(),
            is_high_value: false,
            sequence: 3,
        },
    ];
    let next = GiftSoundQueueItem {
        event_id: "next".into(),
        is_high_value: true,
        sequence: 4,
    };
    let result = enqueue_gift_sound(&queue, next, 3);
    assert_eq!(
        result
            .iter()
            .map(|item| item.event_id.as_str())
            .collect::<Vec<_>>(),
        ["high", "low-new", "next"]
    );
}

#[test]
fn combo_requires_same_user_and_gift_inside_the_selected_window() {
    let first = gift(1, "1", "31036", 100);
    let mut second = gift(2, "1", "31036", 100);
    second.occurred_at = first.occurred_at + 9_999;
    assert!(should_merge_combo(&first, &second, 10));
    second.gift_id = "other".into();
    assert!(!should_merge_combo(&first, &second, 10));
    second.gift_id = first.gift_id.clone();
    second.occurred_at = first.occurred_at + 10_001;
    assert!(!should_merge_combo(&first, &second, 10));
}

#[test]
fn recording_transitions_and_disk_waterline_are_closed() {
    assert_eq!(
        next_recording_state(RecordingState::Idle, RecordingState::Starting),
        Ok(RecordingState::Starting)
    );
    assert_eq!(
        next_recording_state(RecordingState::Starting, RecordingState::Recording),
        Ok(RecordingState::Recording)
    );
    assert_eq!(
        next_recording_state(RecordingState::Recording, RecordingState::Stopping),
        Ok(RecordingState::Stopping)
    );
    assert_eq!(
        next_recording_state(RecordingState::Stopping, RecordingState::Finalizing),
        Ok(RecordingState::Finalizing)
    );
    assert_eq!(
        next_recording_state(RecordingState::Finalizing, RecordingState::Idle),
        Ok(RecordingState::Idle)
    );
    assert_eq!(
        next_recording_state(RecordingState::Idle, RecordingState::Recording),
        Err(MediaDomainError::InvalidTransition)
    );
    assert!(should_stop_for_disk(
        DEFAULT_RECORDING_WATERLINE_BYTES - 1,
        DEFAULT_RECORDING_WATERLINE_BYTES
    ));
    assert!(!should_stop_for_disk(
        DEFAULT_RECORDING_WATERLINE_BYTES,
        DEFAULT_RECORDING_WATERLINE_BYTES
    ));
}

#[test]
fn heat_highlight_and_filename_rules_preserve_prd_boundaries() {
    assert!(is_danmaku_surge(&[1_000, 5_000, 10_999, 11_000], 11_000, 4));
    assert!(!is_danmaku_surge(&[999, 1_000, 5_000], 11_000, 3));
    assert_eq!(
        highlight_window(20, Some(200)),
        HighlightWindow {
            start_seconds: 0,
            end_seconds: 50
        }
    );
    assert_eq!(
        highlight_window(190, Some(200)),
        HighlightWindow {
            start_seconds: 160,
            end_seconds: 200
        }
    );
    assert_eq!(
        validate_recording_filename_stem("  直播  精彩  ").unwrap(),
        "直播  精彩"
    );
    for invalid in [
        "   ",
        "bad/name",
        "bad\\name",
        "bad:name",
        "line\nbreak",
        "CON",
    ] {
        assert!(
            validate_recording_filename_stem(invalid).is_err(),
            "{invalid}"
        );
    }
}
