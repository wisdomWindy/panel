use std::collections::{HashMap, HashSet};

use super::{AudienceIdentity, LiveEvent, UserUid};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AnalyticsState {
    seen_event_ids: HashSet<String>,
    online_count: u64,
    danmu_count: u64,
    gift_value_minor: i64,
    captain_count: u64,
    filtered_count: u64,
    first_event_at: Option<i64>,
    danmu: HashMap<UserUid, RankingValue>,
    gift: HashMap<UserUid, RankingValue>,
    captain: HashMap<UserUid, RankingValue>,
    audiences: HashMap<UserUid, AudienceIdentity>,
}

pub fn record_filtered_event(mut state: AnalyticsState, event: &LiveEvent) -> AnalyticsState {
    if state.seen_event_ids.insert(event.base().event_id.clone()) {
        state.filtered_count += 1;
    }
    state
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RankingValue {
    username: String,
    value: i64,
}

impl AnalyticsState {
    pub fn with_online_count(mut self, online_count: u64) -> Self {
        self.online_count = online_count;
        self
    }

    pub fn audience_detail(&self, user_uid: &UserUid) -> Option<(AudienceIdentity, u64, i64)> {
        self.audiences.get(user_uid).cloned().map(|identity| {
            let danmu_count = self
                .danmu
                .get(user_uid)
                .map_or(0, |value| value.value as u64);
            let gift_value_minor = self.gift.get(user_uid).map_or(0, |value| value.value);
            (identity, danmu_count, gift_value_minor)
        })
    }
}

pub fn aggregate_event(mut state: AnalyticsState, event: &LiveEvent) -> AnalyticsState {
    if !state.seen_event_ids.insert(event.base().event_id.clone()) {
        return state;
    }
    state.first_event_at = Some(
        state
            .first_event_at
            .map_or(event.base().occurred_at, |value| {
                value.min(event.base().occurred_at)
            }),
    );
    let audience = &event.base().audience;
    state
        .audiences
        .insert(audience.user_uid.clone(), audience.clone());
    match event {
        LiveEvent::Danmu { .. } => {
            state.danmu_count += 1;
            add_ranking_value(&mut state.danmu, audience, 1);
        }
        LiveEvent::Gift { value_minor, .. } => {
            state.gift_value_minor += value_minor;
            add_ranking_value(&mut state.gift, audience, *value_minor);
        }
        LiveEvent::Captain { value_minor, .. } => {
            state.captain_count += 1;
            add_ranking_value(&mut state.captain, audience, *value_minor);
        }
        LiveEvent::Enter { .. } => {}
    }
    state
}

fn add_ranking_value(
    values: &mut HashMap<UserUid, RankingValue>,
    audience: &super::AudienceIdentity,
    amount: i64,
) {
    let entry = values
        .entry(audience.user_uid.clone())
        .or_insert_with(|| RankingValue {
            username: audience.username.clone(),
            value: 0,
        });
    entry.username.clone_from(&audience.username);
    entry.value += amount;
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatMetrics {
    pub online_count: u64,
    pub danmu_count: u64,
    pub danmu_rate: f64,
    pub gift_value_minor: i64,
    pub captain_count: u64,
    pub filtered_count: u64,
}

pub fn build_stat_snapshot(state: &AnalyticsState, generated_at: i64) -> StatMetrics {
    let elapsed_seconds = state
        .first_event_at
        .map(|started_at| ((generated_at - started_at).max(1_000) as f64) / 1_000.0)
        .unwrap_or(1.0);
    StatMetrics {
        online_count: state.online_count,
        danmu_count: state.danmu_count,
        danmu_rate: ((state.danmu_count as f64 / elapsed_seconds) * 10.0).round() / 10.0,
        gift_value_minor: state.gift_value_minor,
        captain_count: state.captain_count,
        filtered_count: state.filtered_count,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RankingLimit {
    Top10,
    Top20,
    Top50,
    Top100,
}

impl RankingLimit {
    const fn get(self) -> usize {
        match self {
            Self::Top10 => 10,
            Self::Top20 => 20,
            Self::Top50 => 50,
            Self::Top100 => 100,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MoneyRankingEntry {
    pub rank: u32,
    pub user_uid: UserUid,
    pub username: String,
    pub amount_minor: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DanmuRankingEntry {
    pub rank: u32,
    pub user_uid: UserUid,
    pub username: String,
    pub danmu_count: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RankingSnapshot {
    pub gift: Vec<MoneyRankingEntry>,
    pub danmu: Vec<DanmuRankingEntry>,
    pub captain: Vec<MoneyRankingEntry>,
}

pub fn build_ranking_snapshot(state: &AnalyticsState, limit: RankingLimit) -> RankingSnapshot {
    RankingSnapshot {
        gift: money_ranking(&state.gift, limit.get()),
        danmu: danmu_ranking(&state.danmu, limit.get()),
        captain: money_ranking(&state.captain, limit.get()),
    }
}

fn sorted_values(
    values: &HashMap<UserUid, RankingValue>,
    limit: usize,
) -> Vec<(&UserUid, &RankingValue)> {
    let mut rows: Vec<_> = values.iter().collect();
    rows.sort_by(|(left_uid, left), (right_uid, right)| {
        right
            .value
            .cmp(&left.value)
            .then_with(|| left_uid.as_str().cmp(right_uid.as_str()))
    });
    rows.truncate(limit);
    rows
}

fn money_ranking(values: &HashMap<UserUid, RankingValue>, limit: usize) -> Vec<MoneyRankingEntry> {
    sorted_values(values, limit)
        .into_iter()
        .enumerate()
        .map(|(index, (uid, value))| MoneyRankingEntry {
            rank: (index + 1) as u32,
            user_uid: uid.clone(),
            username: value.username.clone(),
            amount_minor: value.value,
        })
        .collect()
}

fn danmu_ranking(values: &HashMap<UserUid, RankingValue>, limit: usize) -> Vec<DanmuRankingEntry> {
    sorted_values(values, limit)
        .into_iter()
        .enumerate()
        .map(|(index, (uid, value))| DanmuRankingEntry {
            rank: (index + 1) as u32,
            user_uid: uid.clone(),
            username: value.username.clone(),
            danmu_count: value.value as u64,
        })
        .collect()
}
