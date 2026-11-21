use std::{
    collections::HashMap,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use tauri::{AppHandle, Emitter, State};

use crate::{
    application::live_data::{
        ConnectionStatus, ExportReceipt, ImportSummary, LiveEventSink, LiveRuntimeError,
        RoomSnapshot, SensitiveFileFormat,
    },
    domain::live_data::{
        AudienceIdentity, BlacklistEntry, CaptainAction, CaptainLevel, DanmuRankingEntry,
        IdentityTag, LiveEvent, MoneyRankingEntry, RankingSnapshot, SensitiveMatchType,
        SensitiveRule, SensitiveWordCategory, StatMetrics,
    },
    AppState,
};

use super::{dto::*, error::DesktopError};

#[derive(Default)]
pub struct RankingPresenter {
    previous: Mutex<PreviousRankings>,
}

#[derive(Default)]
struct PreviousRankings {
    gift: HashMap<String, u32>,
    danmu: HashMap<String, u32>,
    captain: HashMap<String, u32>,
}

impl RankingPresenter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn present(
        &self,
        snapshot: &RankingSnapshot,
        range: DataRangeDto,
    ) -> Result<RankingSnapshotDto, LiveRuntimeError> {
        validate_range(&range)?;
        let mut previous = self.previous.lock().map_err(|_| {
            LiveRuntimeError::new(
                "ranking_state_unavailable",
                "ranking state is unavailable",
                true,
            )
        })?;
        let gift = snapshot
            .gift
            .iter()
            .map(|row| money_ranking(row, &previous.gift))
            .collect();
        let danmu = snapshot
            .danmu
            .iter()
            .map(|row| danmu_ranking(row, &previous.danmu))
            .collect();
        let captain = snapshot
            .captain
            .iter()
            .map(|row| money_ranking(row, &previous.captain))
            .collect();
        previous.gift = rank_index(&snapshot.gift);
        previous.danmu = danmu_rank_index(&snapshot.danmu);
        previous.captain = rank_index(&snapshot.captain);
        Ok(RankingSnapshotDto {
            range: range.clone(),
            gift,
            danmu,
            captain,
            gift_total_users: snapshot.gift.len() as u32,
            danmu_total_users: snapshot.danmu.len() as u32,
            captain_total_users: snapshot.captain.len() as u32,
            generated_at: now_millis(),
            is_historical_data_available: matches!(range, DataRangeDto::LiveSession),
            is_stale: false,
        })
    }
}

pub struct TauriLiveEventSink {
    app: AppHandle,
    rankings: std::sync::Arc<RankingPresenter>,
}

impl TauriLiveEventSink {
    pub fn new(app: AppHandle, rankings: std::sync::Arc<RankingPresenter>) -> Self {
        Self { app, rankings }
    }

    fn emit<T: serde::Serialize + Clone>(
        &self,
        event: &str,
        payload: T,
    ) -> Result<(), LiveRuntimeError> {
        self.app
            .emit(event, payload)
            .map_err(|error| LiveRuntimeError::new("event_publish_failed", error.to_string(), true))
    }
}

impl LiveEventSink for TauriLiveEventSink {
    fn publish_room_status(&self, snapshot: &RoomSnapshot) -> Result<(), LiveRuntimeError> {
        self.emit(
            "room_status_changed",
            RoomStatusChangedDto {
                status: connection_status(snapshot.status),
                room_id: snapshot.room_id.as_ref().map(|id| id.as_str().to_owned()),
                attempt: snapshot.reconnect_attempt,
                error_code: snapshot.last_error_code.clone(),
            },
        )
    }

    fn publish_live_event(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError> {
        match event {
            LiveEvent::Danmu { base, content } => self.emit(
                "danmu_received",
                DanmuEventDto {
                    kind: "danmu".into(),
                    event_id: base.event_id.clone(),
                    room_id: base.room_id.as_str().to_owned(),
                    occurred_at: base.occurred_at,
                    audience: audience(&base.audience),
                    content: content.clone(),
                },
            ),
            LiveEvent::Gift {
                base,
                gift_id,
                gift_name,
                gift_icon_url,
                quantity,
                value_minor,
            } => self.emit(
                "gift_received",
                GiftEventDto {
                    kind: "gift".into(),
                    event_id: base.event_id.clone(),
                    room_id: base.room_id.as_str().to_owned(),
                    occurred_at: base.occurred_at,
                    audience: audience(&base.audience),
                    gift_id: gift_id.clone(),
                    gift_name: gift_name.clone(),
                    gift_icon_url: gift_icon_url.clone(),
                    quantity: *quantity,
                    value_minor: *value_minor,
                },
            ),
            LiveEvent::Captain {
                base,
                action,
                level,
                value_minor,
            } => self.emit(
                "captain_received",
                CaptainEventDto {
                    kind: "captain".into(),
                    event_id: base.event_id.clone(),
                    room_id: base.room_id.as_str().to_owned(),
                    occurred_at: base.occurred_at,
                    audience: audience(&base.audience),
                    action: captain_action(*action),
                    level: captain_level(*level),
                    value_minor: *value_minor,
                },
            ),
            LiveEvent::Enter { base } => self.emit(
                "enter_received",
                EnterEventDto {
                    kind: "enter".into(),
                    event_id: base.event_id.clone(),
                    room_id: base.room_id.as_str().to_owned(),
                    occurred_at: base.occurred_at,
                    audience: audience(&base.audience),
                },
            ),
            // Session boundaries reuse the enter payload shape: the renderer
            // only needs kind + room + time to reset or seal its view.
            LiveEvent::SessionStarted { base } => self.emit(
                "session_started",
                EnterEventDto {
                    kind: "sessionStarted".into(),
                    event_id: base.event_id.clone(),
                    room_id: base.room_id.as_str().to_owned(),
                    occurred_at: base.occurred_at,
                    audience: audience(&base.audience),
                },
            ),
            LiveEvent::SessionEnded { base } => self.emit(
                "session_ended",
                EnterEventDto {
                    kind: "sessionEnded".into(),
                    event_id: base.event_id.clone(),
                    room_id: base.room_id.as_str().to_owned(),
                    occurred_at: base.occurred_at,
                    audience: audience(&base.audience),
                },
            ),
        }
    }

    fn publish_stat(&self, metrics: &StatMetrics) -> Result<(), LiveRuntimeError> {
        self.emit(
            "stat_updated",
            stat_snapshot(metrics, DataRangeDto::LiveSession)?,
        )
    }

    fn publish_ranking(&self, snapshot: &RankingSnapshot) -> Result<(), LiveRuntimeError> {
        self.emit(
            "rank_updated",
            self.rankings.present(snapshot, DataRangeDto::LiveSession)?,
        )
    }

    fn publish_sensitive_compile_progress(
        &self,
        percentage: u8,
        stage: &str,
    ) -> Result<(), LiveRuntimeError> {
        self.emit(
            "sensitive_word_compile_progress",
            SensitiveWordCompileProgressDto {
                percentage,
                stage: stage.to_owned(),
            },
        )
    }

    fn publish_export(
        &self,
        export_type: &str,
        display_path: &str,
    ) -> Result<(), LiveRuntimeError> {
        self.emit(
            "export_done",
            ExportDoneDto {
                export_type: export_type_from_name(export_type)?,
                display_path: display_path.to_owned(),
            },
        )
    }
}

#[tauri::command]
pub async fn room_connect(
    state: State<'_, AppState>,
    request: RoomConnectRequestDto,
) -> Result<RoomSnapshotDto, DesktopError> {
    state
        .live_data()
        .connect_room(&request.identity_code)
        .await
        .map(room_snapshot)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn room_disconnect(state: State<'_, AppState>) -> Result<RoomSnapshotDto, DesktopError> {
    state
        .live_data()
        .disconnect_room()
        .await
        .map(room_snapshot)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn audience_detail_query(
    state: State<'_, AppState>,
    request: AudienceDetailQueryRequestDto,
) -> Result<AudienceDetailDto, DesktopError> {
    state
        .live_data()
        .audience_detail(&request.room_id, &request.user_uid)
        .await
        .map(
            |(identity, session_danmu_count, gift_value_minor, is_blacklisted)| AudienceDetailDto {
                identity: audience(&identity),
                session_danmu_count,
                gift_value_minor,
                is_blacklisted,
            },
        )
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sensitive_word_list(
    state: State<'_, AppState>,
    request: SensitiveWordListRequestDto,
) -> Result<SensitiveWordSnapshotDto, DesktopError> {
    let category = request.category.map(sensitive_category_from_dto);
    let today_hit_count = state.live_data().today_sensitive_hits().await?;
    Ok(SensitiveWordSnapshotDto {
        items: state
            .live_data()
            .filtered_sensitive_words(category, request.search.as_deref())
            .await
            .iter()
            .map(sensitive_word)
            .collect(),
        today_hit_count,
    })
}

#[tauri::command]
pub async fn sensitive_word_add(
    state: State<'_, AppState>,
    request: SensitiveWordFormDto,
) -> Result<SensitiveWordDto, DesktopError> {
    state
        .live_data()
        .add_sensitive_word(
            &request.word,
            sensitive_category_from_dto(request.category),
            sensitive_match_from_dto(request.match_type),
            request.is_enabled,
        )
        .await
        .map(|rule| sensitive_word(&rule))
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sensitive_word_update(
    state: State<'_, AppState>,
    request: SensitiveWordUpdateRequestDto,
) -> Result<SensitiveWordDto, DesktopError> {
    state
        .live_data()
        .update_sensitive_word(
            request.id,
            &request.form.word,
            sensitive_category_from_dto(request.form.category),
            sensitive_match_from_dto(request.form.match_type),
            request.form.is_enabled,
        )
        .await
        .map(|rule| sensitive_word(&rule))
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sensitive_word_delete(
    state: State<'_, AppState>,
    request: SensitiveWordIdRequestDto,
) -> Result<DeleteResultDto, DesktopError> {
    state
        .live_data()
        .delete_sensitive_word(request.id)
        .await
        .map(|deleted| DeleteResultDto { deleted })
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sensitive_word_toggle(
    state: State<'_, AppState>,
    request: SensitiveWordToggleRequestDto,
) -> Result<SensitiveWordDto, DesktopError> {
    state
        .live_data()
        .toggle_sensitive_word(request.id, request.is_enabled)
        .await
        .map(|rule| sensitive_word(&rule))
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sensitive_word_import(
    state: State<'_, AppState>,
    request: SensitiveWordFileRequestDto,
) -> Result<ImportResultDto, DesktopError> {
    state
        .live_data()
        .import_sensitive_words(&request.grant_id, sensitive_format(request.format))
        .await
        .map(import_result)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn sensitive_word_export(
    state: State<'_, AppState>,
    request: SensitiveWordFileRequestDto,
) -> Result<ExportResultDto, DesktopError> {
    state
        .live_data()
        .export_sensitive_words(&request.grant_id, sensitive_format(request.format))
        .await
        .map(|receipt| export_result(ExportTypeDto::SensitiveWords, receipt))
        .map_err(Into::into)
}

#[tauri::command]
pub async fn blacklist_list(
    state: State<'_, AppState>,
) -> Result<BlacklistSnapshotDto, DesktopError> {
    let today_added_count = state.live_data().today_blacklist_additions().await?;
    Ok(BlacklistSnapshotDto {
        items: state
            .live_data()
            .blacklist_entries()
            .await
            .iter()
            .map(blacklist_entry)
            .collect(),
        today_added_count,
    })
}

#[tauri::command]
pub async fn blacklist_add(
    state: State<'_, AppState>,
    request: BlacklistAddRequestDto,
) -> Result<BlacklistEntryDto, DesktopError> {
    state
        .live_data()
        .add_blacklist_entry(&request.uid, &request.username, request.reason.as_deref())
        .await
        .map(|entry| blacklist_entry(&entry))
        .map_err(Into::into)
}

#[tauri::command]
pub async fn blacklist_remove(
    state: State<'_, AppState>,
    request: BlacklistUidRequestDto,
) -> Result<DeleteResultDto, DesktopError> {
    state
        .live_data()
        .remove_blacklist_entry(&request.uid)
        .await
        .map(|deleted| DeleteResultDto { deleted })
        .map_err(Into::into)
}

#[tauri::command]
pub async fn blacklist_export(
    state: State<'_, AppState>,
    request: GrantRequestDto,
) -> Result<ExportResultDto, DesktopError> {
    state
        .live_data()
        .export_blacklist(&request.grant_id)
        .await
        .map(|receipt| export_result(ExportTypeDto::Blacklist, receipt))
        .map_err(Into::into)
}

#[tauri::command]
pub async fn blacklist_import(
    state: State<'_, AppState>,
    request: GrantRequestDto,
) -> Result<ImportResultDto, DesktopError> {
    state
        .live_data()
        .import_blacklist(&request.grant_id)
        .await
        .map(import_result)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn stat_query(
    state: State<'_, AppState>,
    request: RangeRequestDto,
) -> Result<StatSnapshotDto, DesktopError> {
    stat_snapshot(&state.live_data().stat_snapshot().await, request.range).map_err(Into::into)
}

#[tauri::command]
pub async fn stat_refresh(
    state: State<'_, AppState>,
    request: RangeRequestDto,
) -> Result<StatSnapshotDto, DesktopError> {
    stat_query(state, request).await
}

#[tauri::command]
pub async fn stat_export(
    state: State<'_, AppState>,
    request: RangeExportRequestDto,
) -> Result<ExportResultDto, DesktopError> {
    validate_range(&request.range).map_err(DesktopError::from)?;
    validate_export_range(&request.range).map_err(DesktopError::from)?;
    state
        .live_data()
        .export_statistics(&request.grant_id)
        .await
        .map(|receipt| export_result(ExportTypeDto::Statistics, receipt))
        .map_err(Into::into)
}

#[tauri::command]
pub async fn rank_query(
    state: State<'_, AppState>,
    request: RangeRequestDto,
) -> Result<RankingSnapshotDto, DesktopError> {
    let snapshot = state.live_data().ranking_snapshot().await;
    state
        .ranking_presenter()
        .present(&snapshot, request.range)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn rank_export(
    state: State<'_, AppState>,
    request: RangeExportRequestDto,
) -> Result<ExportResultDto, DesktopError> {
    validate_range(&request.range).map_err(DesktopError::from)?;
    validate_export_range(&request.range).map_err(DesktopError::from)?;
    state
        .live_data()
        .export_rankings(&request.grant_id)
        .await
        .map(|receipt| export_result(ExportTypeDto::Rankings, receipt))
        .map_err(Into::into)
}

fn validate_range(range: &DataRangeDto) -> Result<(), LiveRuntimeError> {
    if let DataRangeDto::Custom { start_at, end_at } = range {
        if *start_at <= 0 || *end_at <= *start_at {
            return Err(LiveRuntimeError::new(
                "invalid_data_range",
                "data range is invalid",
                false,
            ));
        }
    }
    Ok(())
}

fn validate_export_range(range: &DataRangeDto) -> Result<(), LiveRuntimeError> {
    if !matches!(range, DataRangeDto::LiveSession) {
        return Err(LiveRuntimeError::new(
            "historical_data_unavailable",
            "historical data is unavailable",
            false,
        ));
    }
    Ok(())
}

fn stat_snapshot(
    metrics: &StatMetrics,
    range: DataRangeDto,
) -> Result<StatSnapshotDto, LiveRuntimeError> {
    validate_range(&range)?;
    let generated_at = now_millis();
    let metrics_dto = stat_metrics(metrics);
    Ok(StatSnapshotDto {
        range: range.clone(),
        metrics: metrics_dto.clone(),
        trend_points: vec![StatTrendPointDto {
            occurred_at: generated_at,
            metrics: metrics_dto,
        }],
        generated_at,
        is_historical_data_available: matches!(range, DataRangeDto::LiveSession),
        is_stale: false,
    })
}

fn room_snapshot(snapshot: RoomSnapshot) -> RoomSnapshotDto {
    RoomSnapshotDto {
        status: connection_status(snapshot.status),
        room_id: snapshot.room_id.map(|id| id.as_str().to_owned()),
        room_name: snapshot.room_name,
        online_count: snapshot.online_count,
        reconnect_attempt: snapshot.reconnect_attempt,
        last_error_code: snapshot.last_error_code,
    }
}

fn connection_status(status: ConnectionStatus) -> ConnectionStatusDto {
    match status {
        ConnectionStatus::Disconnected => ConnectionStatusDto::Disconnected,
        ConnectionStatus::Connecting => ConnectionStatusDto::Connecting,
        ConnectionStatus::Connected => ConnectionStatusDto::Connected,
        ConnectionStatus::Reconnecting => ConnectionStatusDto::Reconnecting,
        ConnectionStatus::Error => ConnectionStatusDto::Error,
    }
}

fn audience(value: &AudienceIdentity) -> AudienceIdentityDto {
    AudienceIdentityDto {
        user_uid: value.user_uid.as_str().to_owned(),
        username: value.username.clone(),
        user_level: value.user_level,
        medal: value.medal.as_ref().map(|medal| FanMedalDto {
            name: medal.name.clone(),
            level: medal.level,
        }),
        tags: value
            .tags
            .iter()
            .map(|tag| match tag {
                IdentityTag::Admin => IdentityTagDto::Admin,
                IdentityTag::Governor => IdentityTagDto::Governor,
                IdentityTag::Admiral => IdentityTagDto::Admiral,
                IdentityTag::Captain => IdentityTagDto::Captain,
                IdentityTag::Vip => IdentityTagDto::Vip,
            })
            .collect(),
    }
}

fn captain_action(value: CaptainAction) -> CaptainActionDto {
    match value {
        CaptainAction::Purchase => CaptainActionDto::Purchase,
        CaptainAction::Renewal => CaptainActionDto::Renewal,
    }
}
fn captain_level(value: CaptainLevel) -> CaptainLevelDto {
    match value {
        CaptainLevel::Governor => CaptainLevelDto::Governor,
        CaptainLevel::Admiral => CaptainLevelDto::Admiral,
        CaptainLevel::Captain => CaptainLevelDto::Captain,
    }
}
fn sensitive_category_from_dto(value: SensitiveWordCategoryDto) -> SensitiveWordCategory {
    match value {
        SensitiveWordCategoryDto::Illegal => SensitiveWordCategory::Illegal,
        SensitiveWordCategoryDto::Advertising => SensitiveWordCategory::Advertising,
        SensitiveWordCategoryDto::PersonalAttack => SensitiveWordCategory::PersonalAttack,
        SensitiveWordCategoryDto::Custom => SensitiveWordCategory::Custom,
    }
}
fn sensitive_category(value: SensitiveWordCategory) -> SensitiveWordCategoryDto {
    match value {
        SensitiveWordCategory::Illegal => SensitiveWordCategoryDto::Illegal,
        SensitiveWordCategory::Advertising => SensitiveWordCategoryDto::Advertising,
        SensitiveWordCategory::PersonalAttack => SensitiveWordCategoryDto::PersonalAttack,
        SensitiveWordCategory::Custom => SensitiveWordCategoryDto::Custom,
    }
}
fn sensitive_match_from_dto(value: SensitiveMatchTypeDto) -> SensitiveMatchType {
    match value {
        SensitiveMatchTypeDto::Exact => SensitiveMatchType::Exact,
        SensitiveMatchTypeDto::Fuzzy => SensitiveMatchType::Fuzzy,
    }
}
fn sensitive_match(value: SensitiveMatchType) -> SensitiveMatchTypeDto {
    match value {
        SensitiveMatchType::Exact => SensitiveMatchTypeDto::Exact,
        SensitiveMatchType::Fuzzy => SensitiveMatchTypeDto::Fuzzy,
    }
}
fn sensitive_format(value: SensitiveFileFormatDto) -> SensitiveFileFormat {
    match value {
        SensitiveFileFormatDto::Csv => SensitiveFileFormat::Csv,
        SensitiveFileFormatDto::Json => SensitiveFileFormat::Json,
        SensitiveFileFormatDto::Txt => SensitiveFileFormat::Txt,
    }
}

fn sensitive_word(rule: &SensitiveRule) -> SensitiveWordDto {
    SensitiveWordDto {
        id: rule.id,
        word: rule.word.clone(),
        category: sensitive_category(rule.category),
        match_type: sensitive_match(rule.match_type),
        is_enabled: rule.is_enabled,
    }
}

fn blacklist_entry(entry: &BlacklistEntry) -> BlacklistEntryDto {
    BlacklistEntryDto {
        uid: entry.uid.as_str().to_owned(),
        username: entry.username.clone(),
        reason: entry.reason.clone(),
        created_at: entry.created_at,
    }
}

fn stat_metrics(metrics: &StatMetrics) -> StatMetricsDto {
    StatMetricsDto {
        online_count: metrics.online_count,
        danmu_count: metrics.danmu_count,
        danmu_rate: metrics.danmu_rate,
        gift_value_minor: metrics.gift_value_minor,
        captain_count: metrics.captain_count,
        filtered_count: metrics.filtered_count,
    }
}

fn import_result(summary: ImportSummary) -> ImportResultDto {
    ImportResultDto {
        imported_count: summary.imported_count,
        duplicate_count: summary.duplicate_count,
        error_count: 0,
        errors: Vec::new(),
        is_rolled_back: false,
    }
}

fn export_result(export_type: ExportTypeDto, receipt: ExportReceipt) -> ExportResultDto {
    ExportResultDto {
        export_type,
        display_path: receipt.display_path,
        exported_count: receipt.exported_count,
    }
}

fn export_type_from_name(value: &str) -> Result<ExportTypeDto, LiveRuntimeError> {
    match value {
        "sensitiveWords" => Ok(ExportTypeDto::SensitiveWords),
        "blacklist" => Ok(ExportTypeDto::Blacklist),
        "statistics" => Ok(ExportTypeDto::Statistics),
        "rankings" => Ok(ExportTypeDto::Rankings),
        _ => Err(LiveRuntimeError::new(
            "invalid_export_type",
            "export type is invalid",
            false,
        )),
    }
}

fn rank_change(current: u32, previous: Option<u32>) -> RankChangeDto {
    match previous {
        None => RankChangeDto::New,
        Some(value) if current < value => RankChangeDto::Up,
        Some(value) if current > value => RankChangeDto::Down,
        Some(_) => RankChangeDto::Same,
    }
}

fn money_ranking(row: &MoneyRankingEntry, previous: &HashMap<String, u32>) -> MoneyRankingEntryDto {
    let previous_rank = previous.get(row.user_uid.as_str()).copied();
    MoneyRankingEntryDto {
        rank: row.rank,
        user_uid: row.user_uid.as_str().to_owned(),
        username: row.username.clone(),
        amount_minor: row.amount_minor,
        previous_rank,
        change: rank_change(row.rank, previous_rank),
    }
}

fn danmu_ranking(row: &DanmuRankingEntry, previous: &HashMap<String, u32>) -> DanmuRankingEntryDto {
    let previous_rank = previous.get(row.user_uid.as_str()).copied();
    DanmuRankingEntryDto {
        rank: row.rank,
        user_uid: row.user_uid.as_str().to_owned(),
        username: row.username.clone(),
        danmu_count: row.danmu_count,
        previous_rank,
        change: rank_change(row.rank, previous_rank),
    }
}

fn rank_index(rows: &[MoneyRankingEntry]) -> HashMap<String, u32> {
    rows.iter()
        .map(|row| (row.user_uid.as_str().to_owned(), row.rank))
        .collect()
}
fn danmu_rank_index(rows: &[DanmuRankingEntry]) -> HashMap<String, u32> {
    rows.iter()
        .map(|row| (row.user_uid.as_str().to_owned(), row.rank))
        .collect()
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::live_data::UserUid;

    fn money(rank: u32, uid: &str) -> MoneyRankingEntry {
        MoneyRankingEntry {
            rank,
            user_uid: UserUid::parse(uid).unwrap(),
            username: format!("user-{uid}"),
            amount_minor: 100,
        }
    }

    #[test]
    fn presents_rank_changes_against_the_previous_rust_snapshot() {
        let presenter = RankingPresenter::new();
        let first = RankingSnapshot {
            gift: vec![money(1, "1"), money(2, "2")],
            danmu: Vec::new(),
            captain: Vec::new(),
        };
        let initial = presenter
            .present(&first, DataRangeDto::LiveSession)
            .unwrap();
        assert_eq!(initial.gift[0].change, RankChangeDto::New);

        let second = RankingSnapshot {
            gift: vec![money(1, "2"), money(2, "1")],
            danmu: Vec::new(),
            captain: Vec::new(),
        };
        let changed = presenter
            .present(&second, DataRangeDto::LiveSession)
            .unwrap();
        assert_eq!(changed.gift[0].previous_rank, Some(2));
        assert_eq!(changed.gift[0].change, RankChangeDto::Up);
        assert_eq!(changed.gift[1].change, RankChangeDto::Down);
    }

    #[test]
    fn rejects_inverted_custom_ranges() {
        let error = validate_range(&DataRangeDto::Custom {
            start_at: 20,
            end_at: 10,
        })
        .unwrap_err();
        assert_eq!(error.code, "invalid_data_range");
    }

    #[test]
    fn rejects_export_ranges_without_persisted_history() {
        validate_export_range(&DataRangeDto::LiveSession).unwrap();
        let error = validate_export_range(&DataRangeDto::Today).unwrap_err();
        assert_eq!(error.code, "historical_data_unavailable");
    }
}
