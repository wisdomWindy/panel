use std::{
    collections::HashSet,
    mem,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tokio::sync::{watch, Mutex};

use crate::domain::live_data::{
    aggregate_event, build_ranking_snapshot, build_stat_snapshot, evaluate_live_event,
    record_filtered_event, AnalyticsState, BlacklistEntry, LiveEvent, PipelineOutcome,
    RankingLimit, RoomId, SensitiveMatchType, SensitiveMatcher, SensitiveRule,
    SensitiveWordCategory, UserUid,
};

use super::ports::NoopAcceptedLiveEventConsumer;
use super::ports::{
    ExportReceipt, ImportSummary, LiveRuntimeError, LiveStatusProbe, NoopLiveStatusProbe,
    RoomConnection, SensitiveFileFormat, SharedAcceptedLiveEventConsumer, SharedLiveEventSink,
    SharedLiveFilePort, SharedLiveRepository, SharedRoomTransport, UnavailableLiveFilePort,
};

/// Reconnect pacing while the room is offline: 5s, then 10s, then every 30s.
///
/// Monitoring has to outlive an offline room, so there is deliberately no
/// give-up count: the session keeps retrying until the operator disconnects.
const RECONNECT_BACKOFF: [Duration; 3] = [
    Duration::from_secs(5),
    Duration::from_secs(10),
    Duration::from_secs(30),
];

fn reconnect_delay(attempt: u32) -> Duration {
    let index = (attempt.saturating_sub(1) as usize).min(RECONNECT_BACKOFF.len() - 1);
    RECONNECT_BACKOFF[index]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoomSnapshot {
    pub status: ConnectionStatus,
    pub room_id: Option<RoomId>,
    pub room_name: Option<String>,
    pub online_count: Option<u64>,
    pub reconnect_attempt: u32,
    pub last_error_code: Option<String>,
    /// `None` until the platform says whether the room is broadcasting: the
    /// long connection carries transitions, and the post-connect probe fills
    /// in the state we attached in the middle of.
    pub is_live: Option<bool>,
}

impl Default for RoomSnapshot {
    fn default() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
            room_id: None,
            room_name: None,
            online_count: None,
            reconnect_attempt: 0,
            last_error_code: None,
            is_live: None,
        }
    }
}

#[derive(Default)]
struct LiveRuntimeState {
    room: RoomSnapshot,
    generation: u64,
    cancel: Option<watch::Sender<bool>>,
    matcher: SensitiveMatcher,
    sensitive_rules: Vec<SensitiveRule>,
    blacklist: HashSet<UserUid>,
    blacklist_entries: Vec<BlacklistEntry>,
    analytics: AnalyticsState,
}

pub struct LiveDataState {
    transport: SharedRoomTransport,
    repository: SharedLiveRepository,
    events: SharedLiveEventSink,
    files: SharedLiveFilePort,
    accepted_events: SharedAcceptedLiveEventConsumer,
    probe: Arc<dyn LiveStatusProbe>,
    runtime: Mutex<LiveRuntimeState>,
    effect_lock: Mutex<()>,
}

impl LiveDataState {
    pub fn new(
        transport: SharedRoomTransport,
        repository: SharedLiveRepository,
        events: SharedLiveEventSink,
    ) -> Self {
        Self {
            transport,
            repository,
            events,
            files: Arc::new(UnavailableLiveFilePort),
            accepted_events: Arc::new(NoopAcceptedLiveEventConsumer),
            probe: Arc::new(NoopLiveStatusProbe),
            runtime: Mutex::new(LiveRuntimeState::default()),
            effect_lock: Mutex::new(()),
        }
    }

    pub fn with_files(mut self, files: SharedLiveFilePort) -> Self {
        self.files = files;
        self
    }

    /// Production wires the real lookup; tests keep the no-op probe so they
    /// never touch the network.
    pub fn with_probe(mut self, probe: Arc<dyn LiveStatusProbe>) -> Self {
        self.probe = probe;
        self
    }

    pub fn with_accepted_consumer(mut self, consumer: SharedAcceptedLiveEventConsumer) -> Self {
        self.accepted_events = consumer;
        self
    }

    pub async fn load_safety(&self) -> Result<(), LiveRuntimeError> {
        let rules = self.repository.load_sensitive().await?;
        let matcher = SensitiveMatcher::compile(&rules)
            .map_err(|_| LiveRuntimeError::storage("sensitive matcher compilation failed"))?;
        let blacklist_entries = self.repository.list_blacklist_entries().await?;
        let blacklist = blacklist_entries
            .iter()
            .map(|entry| entry.uid.clone())
            .collect();
        let mut runtime = self.runtime.lock().await;
        runtime.matcher = matcher;
        runtime.sensitive_rules = rules;
        runtime.blacklist = blacklist;
        runtime.blacklist_entries = blacklist_entries;
        Ok(())
    }

    pub async fn sensitive_words(&self) -> Vec<SensitiveRule> {
        self.runtime.lock().await.sensitive_rules.clone()
    }

    pub async fn filtered_sensitive_words(
        &self,
        category: Option<SensitiveWordCategory>,
        search: Option<&str>,
    ) -> Vec<SensitiveRule> {
        let query = search.map(str::trim).filter(|value| !value.is_empty());
        self.runtime
            .lock()
            .await
            .sensitive_rules
            .iter()
            .filter(|rule| category.is_none_or(|category| rule.category == category))
            .filter(|rule| query.is_none_or(|query| rule.word.contains(query)))
            .cloned()
            .collect()
    }

    pub async fn add_sensitive_word(
        &self,
        word: &str,
        category: SensitiveWordCategory,
        match_type: SensitiveMatchType,
        is_enabled: bool,
    ) -> Result<SensitiveRule, LiveRuntimeError> {
        let _effect = self.effect_lock.lock().await;
        let draft =
            SensitiveRule::new(1, word, category, match_type, is_enabled).map_err(|_| {
                LiveRuntimeError::new("RULE_INVALID_WORD", "sensitive word is invalid", false)
            })?;
        let mut candidate = self.runtime.lock().await.sensitive_rules.clone();
        candidate.push(draft.clone());
        compile_sensitive(&candidate)?;
        let stored = self.repository.add_sensitive(&draft).await?;
        candidate.pop();
        candidate.push(stored.clone());
        self.swap_sensitive(candidate).await?;
        Ok(stored)
    }

    pub async fn update_sensitive_word(
        &self,
        id: i64,
        word: &str,
        category: SensitiveWordCategory,
        match_type: SensitiveMatchType,
        is_enabled: bool,
    ) -> Result<SensitiveRule, LiveRuntimeError> {
        let _effect = self.effect_lock.lock().await;
        let updated =
            SensitiveRule::new(id, word, category, match_type, is_enabled).map_err(|_| {
                LiveRuntimeError::new("RULE_INVALID_WORD", "sensitive word is invalid", false)
            })?;
        let mut candidate = self.runtime.lock().await.sensitive_rules.clone();
        let target = candidate
            .iter_mut()
            .find(|rule| rule.id == id)
            .ok_or_else(|| {
                LiveRuntimeError::new(
                    "sensitive_word_not_found",
                    "sensitive word was not found",
                    false,
                )
            })?;
        *target = updated.clone();
        compile_sensitive(&candidate)?;
        let stored = self.repository.update_sensitive(&updated).await?;
        self.swap_sensitive(candidate).await?;
        Ok(stored)
    }

    pub async fn toggle_sensitive_word(
        &self,
        id: i64,
        is_enabled: bool,
    ) -> Result<SensitiveRule, LiveRuntimeError> {
        let current = self
            .runtime
            .lock()
            .await
            .sensitive_rules
            .iter()
            .find(|rule| rule.id == id)
            .cloned()
            .ok_or_else(|| {
                LiveRuntimeError::new(
                    "sensitive_word_not_found",
                    "sensitive word was not found",
                    false,
                )
            })?;
        self.update_sensitive_word(
            id,
            &current.word,
            current.category,
            current.match_type,
            is_enabled,
        )
        .await
    }

    pub async fn delete_sensitive_word(&self, id: i64) -> Result<bool, LiveRuntimeError> {
        let _effect = self.effect_lock.lock().await;
        let candidate: Vec<_> = self
            .runtime
            .lock()
            .await
            .sensitive_rules
            .iter()
            .filter(|rule| rule.id != id)
            .cloned()
            .collect();
        let matcher = compile_sensitive(&candidate)?;
        let deleted = self.repository.delete_sensitive(id).await?;
        if deleted {
            let mut runtime = self.runtime.lock().await;
            runtime.sensitive_rules = candidate;
            runtime.matcher = matcher;
        }
        Ok(deleted)
    }

    async fn swap_sensitive(&self, rules: Vec<SensitiveRule>) -> Result<(), LiveRuntimeError> {
        let matcher = compile_sensitive(&rules)?;
        let mut runtime = self.runtime.lock().await;
        runtime.sensitive_rules = rules;
        runtime.matcher = matcher;
        Ok(())
    }

    pub async fn blacklist_entries(&self) -> Vec<BlacklistEntry> {
        self.runtime.lock().await.blacklist_entries.clone()
    }

    pub async fn today_sensitive_hits(&self) -> Result<u64, LiveRuntimeError> {
        self.repository.today_sensitive_hits().await
    }

    pub async fn today_blacklist_additions(&self) -> Result<u64, LiveRuntimeError> {
        self.repository.today_blacklist_additions().await
    }

    pub async fn add_blacklist_entry(
        &self,
        uid: &str,
        username: &str,
        reason: Option<&str>,
    ) -> Result<BlacklistEntry, LiveRuntimeError> {
        let _effect = self.effect_lock.lock().await;
        let entry = self
            .repository
            .upsert_blacklist(uid, username, reason)
            .await?;
        let mut runtime = self.runtime.lock().await;
        runtime.blacklist.insert(entry.uid.clone());
        if let Some(existing) = runtime
            .blacklist_entries
            .iter_mut()
            .find(|existing| existing.uid == entry.uid)
        {
            *existing = entry.clone();
        } else {
            runtime.blacklist_entries.push(entry.clone());
        }
        Ok(entry)
    }

    pub async fn remove_blacklist_entry(&self, uid: &str) -> Result<bool, LiveRuntimeError> {
        let _effect = self.effect_lock.lock().await;
        let deleted = self.repository.remove_blacklist(uid).await?;
        if deleted {
            let uid = UserUid::parse(uid).map_err(|_| {
                LiveRuntimeError::new("RULE_INVALID_USER", "user uid is invalid", false)
            })?;
            let mut runtime = self.runtime.lock().await;
            runtime.blacklist.remove(&uid);
            runtime.blacklist_entries.retain(|entry| entry.uid != uid);
        }
        Ok(deleted)
    }

    pub async fn ranking_snapshot(&self) -> crate::domain::live_data::RankingSnapshot {
        build_ranking_snapshot(&self.runtime.lock().await.analytics, RankingLimit::Top100)
    }

    pub async fn audience_detail(
        &self,
        room_id: &str,
        uid: &str,
    ) -> Result<(crate::domain::live_data::AudienceIdentity, u64, i64, bool), LiveRuntimeError>
    {
        let room_id = RoomId::parse(room_id)
            .map_err(|_| LiveRuntimeError::new("ROOM_INVALID_ID", "room id is invalid", false))?;
        let uid = UserUid::parse(uid)
            .map_err(|_| LiveRuntimeError::new("RULE_INVALID_USER", "user uid is invalid", false))?;
        let runtime = self.runtime.lock().await;
        if runtime.room.room_id.as_ref() != Some(&room_id) {
            return Err(LiveRuntimeError::new(
                "room_context_mismatch",
                "audience does not belong to the active room",
                false,
            ));
        }
        runtime
            .analytics
            .audience_detail(&uid)
            .map(|(identity, danmu_count, gift_value)| {
                (
                    identity,
                    danmu_count,
                    gift_value,
                    runtime.blacklist.contains(&uid),
                )
            })
            .ok_or_else(|| {
                LiveRuntimeError::new("ROOM_AUDIENCE_NOT_FOUND", "audience was not found", false)
            })
    }

    pub async fn import_sensitive_words(
        &self,
        grant_id: &str,
        format: SensitiveFileFormat,
    ) -> Result<ImportSummary, LiveRuntimeError> {
        let _effect = self.effect_lock.lock().await;
        self.events
            .publish_sensitive_compile_progress(0, "reading")?;
        let imported = self.files.read_sensitive(grant_id, format).await?;
        let imported_count = imported.len() as u32;
        let mut unique = std::collections::BTreeMap::new();
        for rule in imported {
            unique.insert(rule.word.clone(), rule);
        }
        let candidate: Vec<_> = unique.into_values().collect();
        self.events
            .publish_sensitive_compile_progress(50, "compiling")?;
        compile_sensitive(&candidate)?;
        let stored = self.repository.replace_sensitive(&candidate).await?;
        self.swap_sensitive(stored.clone()).await?;
        self.events
            .publish_sensitive_compile_progress(100, "complete")?;
        Ok(ImportSummary {
            imported_count: stored.len() as u32,
            duplicate_count: imported_count.saturating_sub(stored.len() as u32),
        })
    }

    pub async fn export_sensitive_words(
        &self,
        grant_id: &str,
        format: SensitiveFileFormat,
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        let rules = self.sensitive_words().await;
        if rules.is_empty() {
            return Err(no_export_data());
        }
        let receipt = self.files.write_sensitive(grant_id, format, &rules).await?;
        self.events
            .publish_export("sensitiveWords", &receipt.display_path)?;
        Ok(receipt)
    }

    pub async fn import_blacklist(
        &self,
        grant_id: &str,
    ) -> Result<ImportSummary, LiveRuntimeError> {
        let _effect = self.effect_lock.lock().await;
        let imported = self.files.read_blacklist(grant_id).await?;
        let imported_count = imported.len() as u32;
        let unique_count = imported
            .iter()
            .map(|entry| entry.uid.as_str())
            .collect::<HashSet<_>>()
            .len() as u32;
        let stored = self.repository.replace_blacklist(&imported).await?;
        let mut runtime = self.runtime.lock().await;
        runtime.blacklist = stored.iter().map(|entry| entry.uid.clone()).collect();
        runtime.blacklist_entries = stored;
        Ok(ImportSummary {
            imported_count: unique_count,
            duplicate_count: imported_count.saturating_sub(unique_count),
        })
    }

    pub async fn export_blacklist(
        &self,
        grant_id: &str,
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        let rows = self.blacklist_entries().await;
        if rows.is_empty() {
            return Err(no_export_data());
        }
        let receipt = self.files.write_blacklist(grant_id, &rows).await?;
        self.events
            .publish_export("blacklist", &receipt.display_path)?;
        Ok(receipt)
    }

    pub async fn export_statistics(
        &self,
        grant_id: &str,
    ) -> Result<ExportReceipt, LiveRuntimeError> {
        let metrics = self.stat_snapshot().await;
        if metrics.danmu_count == 0
            && metrics.gift_value_minor == 0
            && metrics.captain_count == 0
            && metrics.online_count == 0
        {
            return Err(no_export_data());
        }
        let receipt = self
            .files
            .write_statistics(grant_id, &metrics, now_millis())
            .await?;
        self.events
            .publish_export("statistics", &receipt.display_path)?;
        Ok(receipt)
    }

    pub async fn export_rankings(&self, grant_id: &str) -> Result<ExportReceipt, LiveRuntimeError> {
        let snapshot = self.ranking_snapshot().await;
        if snapshot.gift.is_empty() && snapshot.danmu.is_empty() && snapshot.captain.is_empty() {
            return Err(no_export_data());
        }
        let receipt = self
            .files
            .write_rankings(grant_id, &snapshot, now_millis())
            .await?;
        self.events
            .publish_export("rankings", &receipt.display_path)?;
        Ok(receipt)
    }

    pub async fn connect_room(
        self: &Arc<Self>,
        identity_code: &str,
    ) -> Result<RoomSnapshot, LiveRuntimeError> {
        let identity_code = identity_code.trim();
        if identity_code.is_empty()
            || identity_code.chars().count() > 128
            || identity_code.chars().any(char::is_control)
        {
            return Err(LiveRuntimeError::new(
                "invalid_identity_code",
                "identity code is invalid",
                false,
            ));
        }
        let (snapshot, generation, cancel) = {
            let mut runtime = self.runtime.lock().await;
            if matches!(
                runtime.room.status,
                ConnectionStatus::Connecting
                    | ConnectionStatus::Connected
                    | ConnectionStatus::Reconnecting
            ) {
                return Err(LiveRuntimeError::new(
                    "room_connection_pending",
                    "a room connection is already active",
                    false,
                ));
            }
            runtime.generation += 1;
            runtime.analytics = AnalyticsState::default();
            let (cancel, receiver) = watch::channel(false);
            runtime.cancel = Some(cancel);
            runtime.room = RoomSnapshot {
                status: ConnectionStatus::Connecting,
                room_id: None,
                room_name: None,
                online_count: None,
                reconnect_attempt: 0,
                last_error_code: None,
                is_live: None,
            };
            (runtime.room.clone(), runtime.generation, receiver)
        };
        self.events.publish_room_status(&snapshot)?;
        let state = Arc::clone(self);
        let identity_code = identity_code.to_owned();
        tokio::spawn(async move {
            state
                .run_room_session(identity_code, generation, cancel)
                .await
        });
        Ok(snapshot)
    }

    pub async fn disconnect_room(&self) -> Result<RoomSnapshot, LiveRuntimeError> {
        let snapshot = {
            let mut runtime = self.runtime.lock().await;
            runtime.generation += 1;
            if let Some(cancel) = runtime.cancel.take() {
                let _ = cancel.send(true);
            }
            runtime.room = RoomSnapshot::default();
            runtime.room.clone()
        };
        self.events.publish_room_status(&snapshot)?;
        Ok(snapshot)
    }

    pub async fn room_snapshot(&self) -> RoomSnapshot {
        self.runtime.lock().await.room.clone()
    }

    pub async fn stat_snapshot(&self) -> crate::domain::live_data::StatMetrics {
        build_stat_snapshot(&self.runtime.lock().await.analytics, now_millis())
    }

    async fn run_room_session(
        self: Arc<Self>,
        identity_code: String,
        generation: u64,
        mut cancel: watch::Receiver<bool>,
    ) {
        let mut attempt = 0_u32;
        loop {
            if *cancel.borrow() || !self.is_current(generation).await {
                return;
            }
            match self.transport.connect(&identity_code).await {
                Ok(connection) => {
                    attempt = 0;
                    if self.set_connected(generation, &connection).await.is_err() {
                        return;
                    }
                    // The platform only announces transitions, so a session
                    // joined mid-stream has no boundary yet: ask once.
                    let probe_room = connection.room_id.clone();
                    let probe_state = Arc::clone(&self);
                    let probe_cancel = cancel.clone();
                    tokio::spawn(async move {
                        let room_id = probe_room.as_str().to_owned();
                        if let Some(is_live) = probe_state.probe.probe(&room_id).await {
                            if !*probe_cancel.borrow() {
                                probe_state.set_initial_live(generation, is_live).await;
                            }
                        }
                    });
                    if self
                        .consume_connection(generation, connection, &mut cancel)
                        .await
                    {
                        return;
                    }
                }
                Err(error) if !error.retryable => {
                    let _ = self.set_error(generation, &error).await;
                    return;
                }
                Err(error) => {
                    // No give-up count: the room may stay offline for hours and
                    // monitoring must still be waiting when it goes live.
                    attempt = attempt.saturating_add(1);
                    if self
                        .set_reconnecting(generation, attempt, &error)
                        .await
                        .is_err()
                    {
                        return;
                    }
                }
            }
            tokio::select! {
                _ = tokio::time::sleep(reconnect_delay(attempt)) => {}
                changed = cancel.changed() => {
                    if changed.is_err() || *cancel.borrow() {
                        return;
                    }
                }
            }
        }
    }

    async fn consume_connection(
        &self,
        generation: u64,
        mut connection: RoomConnection,
        cancel: &mut watch::Receiver<bool>,
    ) -> bool {
        let mut stat_tick = tokio::time::interval(Duration::from_secs(5));
        let mut rank_tick = tokio::time::interval(Duration::from_secs(10));
        stat_tick.tick().await;
        rank_tick.tick().await;
        loop {
            tokio::select! {
                changed = cancel.changed() => {
                    if changed.is_err() || *cancel.borrow() {
                        return true;
                    }
                }
                event = connection.events.recv() => match event {
                    Some(event) => {
                        if self.consume_event(generation, event).await.is_err() {
                            return true;
                        }
                    }
                    None => return false,
                },
                _ = stat_tick.tick() => {
                    let stat = self.stat_snapshot().await;
                    if self.events.publish_stat(&stat).is_err() {
                        return true;
                    }
                }
                _ = rank_tick.tick() => {
                    let ranking = build_ranking_snapshot(
                        &self.runtime.lock().await.analytics,
                        RankingLimit::Top100,
                    );
                    if self.events.publish_ranking(&ranking).is_err() {
                        return true;
                    }
                }
            }
        }
    }

    async fn consume_event(
        &self,
        generation: u64,
        event: LiveEvent,
    ) -> Result<(), LiveRuntimeError> {
        let _effect = self.effect_lock.lock().await;
        // Session boundaries drive the 直播中 badge: they are room state, not
        // content, so they are applied before the event pipeline runs.
        match &event {
            LiveEvent::SessionStarted { .. } => self.set_live(generation, true).await?,
            LiveEvent::SessionEnded { .. } => self.set_live(generation, false).await?,
            _ => {}
        }
        let outcome = {
            let runtime = self.runtime.lock().await;
            if runtime.generation != generation {
                return Ok(());
            }
            evaluate_live_event(event, &runtime.blacklist, &runtime.matcher)
        };
        match outcome {
            PipelineOutcome::Accepted(event) => {
                self.repository.record_event(&event, None).await?;
                let mut runtime = self.runtime.lock().await;
                if runtime.generation != generation {
                    return Ok(());
                }
                runtime.analytics = aggregate_event(mem::take(&mut runtime.analytics), &event);
                drop(runtime);
                if let Err(error) = self.accepted_events.consume_accepted(&event).await {
                    log::warn!("accepted media consumer failed: {}", error.code);
                }
                self.events.publish_live_event(&event)
            }
            PipelineOutcome::SensitiveFiltered { event, .. } => {
                self.repository
                    .record_event(&event, Some("sensitive"))
                    .await?;
                let mut runtime = self.runtime.lock().await;
                runtime.analytics =
                    record_filtered_event(mem::take(&mut runtime.analytics), &event);
                Ok(())
            }
            PipelineOutcome::Blacklisted { event, .. } => {
                self.repository
                    .record_event(&event, Some("blacklist"))
                    .await?;
                let mut runtime = self.runtime.lock().await;
                runtime.analytics =
                    record_filtered_event(mem::take(&mut runtime.analytics), &event);
                Ok(())
            }
            PipelineOutcome::Invalid { .. } => Ok(()),
        }
    }

    async fn is_current(&self, generation: u64) -> bool {
        self.runtime.lock().await.generation == generation
    }

    async fn set_connected(
        &self,
        generation: u64,
        connection: &RoomConnection,
    ) -> Result<(), LiveRuntimeError> {
        let snapshot = {
            let mut runtime = self.runtime.lock().await;
            if runtime.generation != generation {
                return Err(LiveRuntimeError::new(
                    "stale_session",
                    "session is stale",
                    false,
                ));
            }
            runtime.room = RoomSnapshot {
                status: ConnectionStatus::Connected,
                room_id: Some(connection.room_id.clone()),
                room_name: connection.room_name.clone(),
                online_count: connection.online_count,
                reconnect_attempt: 0,
                last_error_code: None,
                is_live: None,
            };
            runtime.analytics = mem::take(&mut runtime.analytics)
                .with_online_count(connection.online_count.unwrap_or(0));
            runtime.room.clone()
        };
        self.events.publish_room_status(&snapshot)
    }

    async fn set_reconnecting(
        &self,
        generation: u64,
        attempt: u32,
        error: &LiveRuntimeError,
    ) -> Result<(), LiveRuntimeError> {
        self.set_failure_state(generation, ConnectionStatus::Reconnecting, attempt, error)
            .await
    }

    async fn set_error(
        &self,
        generation: u64,
        error: &LiveRuntimeError,
    ) -> Result<(), LiveRuntimeError> {
        self.set_failure_state(generation, ConnectionStatus::Error, 0, error)
            .await
    }

    /// Session boundaries decide whether the room is broadcasting; the page
    /// badge reads this, never the connection status, because the connection
    /// outlives 下播 on purpose.
    async fn set_live(&self, generation: u64, is_live: bool) -> Result<(), LiveRuntimeError> {
        let snapshot = {
            let mut runtime = self.runtime.lock().await;
            if runtime.generation != generation || runtime.room.is_live == Some(is_live) {
                return Ok(());
            }
            runtime.room.is_live = Some(is_live);
            runtime.room.clone()
        };
        self.events.publish_room_status(&snapshot)
    }

    /// Seeds the state right after connecting. A session boundary that already
    /// arrived is newer than the probe's request, so it wins.
    async fn set_initial_live(&self, generation: u64, is_live: bool) {
        let snapshot = {
            let mut runtime = self.runtime.lock().await;
            if runtime.generation != generation || runtime.room.is_live.is_some() {
                return;
            }
            runtime.room.is_live = Some(is_live);
            runtime.room.clone()
        };
        let _ = self.events.publish_room_status(&snapshot);
    }

    async fn set_failure_state(
        &self,
        generation: u64,
        status: ConnectionStatus,
        attempt: u32,
        error: &LiveRuntimeError,
    ) -> Result<(), LiveRuntimeError> {
        let snapshot = {
            let mut runtime = self.runtime.lock().await;
            if runtime.generation != generation {
                return Err(LiveRuntimeError::new(
                    "stale_session",
                    "session is stale",
                    false,
                ));
            }
            runtime.room.status = status;
            runtime.room.reconnect_attempt = attempt;
            runtime.room.last_error_code = Some(error.code.into());
            runtime.room.clone()
        };
        self.events.publish_room_status(&snapshot)
    }
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

fn compile_sensitive(rules: &[SensitiveRule]) -> Result<SensitiveMatcher, LiveRuntimeError> {
    SensitiveMatcher::compile(rules).map_err(|_| {
        LiveRuntimeError::new(
            "invalid_sensitive_pattern",
            "sensitive pattern is invalid",
            false,
        )
    })
}

fn no_export_data() -> LiveRuntimeError {
    LiveRuntimeError::new("STORAGE_NO_EXPORT_DATA", "there is no data to export", false)
}
