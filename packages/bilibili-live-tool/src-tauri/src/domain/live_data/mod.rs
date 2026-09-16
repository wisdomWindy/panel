mod analytics;
mod models;
mod pipeline;
mod safety;

pub use crate::domain::validation::{RoomId, UserUid};
pub use analytics::{
    aggregate_event, build_ranking_snapshot, build_stat_snapshot, record_filtered_event,
    AnalyticsState, DanmuRankingEntry, MoneyRankingEntry, RankingLimit, RankingSnapshot,
    StatMetrics,
};
pub use models::{
    normalize_identity, normalize_optional_reason, AudienceIdentity, AudienceIdentityInput,
    BlacklistEntry, CaptainAction, CaptainLevel, FanMedal, IdentityTag, LiveDataDomainError,
    LiveEvent, LiveEventBase,
};
pub use pipeline::{evaluate_live_event, PipelineOutcome};
pub use safety::{SensitiveMatchType, SensitiveMatcher, SensitiveRule, SensitiveWordCategory};

#[cfg(test)]
mod tests;
