use std::collections::HashSet;
use std::time::Instant;

use super::{
    aggregate_event, build_ranking_snapshot, build_stat_snapshot, evaluate_live_event,
    normalize_identity, AnalyticsState, AudienceIdentityInput, CaptainAction, CaptainLevel,
    IdentityTag, LiveEvent, LiveEventBase, PipelineOutcome, RankingLimit, SensitiveMatchType,
    SensitiveMatcher, SensitiveRule, SensitiveWordCategory, UserUid,
};

fn identity(uid: &str) -> super::AudienceIdentity {
    normalize_identity(AudienceIdentityInput {
        user_uid: uid.into(),
        username: format!("user-{uid}"),
        user_level: Some(12),
        medal_name: Some("medal".into()),
        medal_level: Some(-1),
        is_admin: false,
        captain_level: None,
        is_vip: false,
    })
    .unwrap()
}

fn danmu(event_id: &str, uid: &str, content: &str, occurred_at: i64) -> LiveEvent {
    LiveEvent::Danmu {
        base: LiveEventBase {
            event_id: event_id.into(),
            room_id: super::RoomId::parse("100").unwrap(),
            occurred_at,
            audience: identity(uid),
        },
        content: content.into(),
    }
}

#[test]
fn validates_ids_and_downgrades_invalid_optional_identity_fields() {
    assert_eq!(super::RoomId::parse(" 123 ").unwrap().as_str(), "123");
    assert!(super::RoomId::parse("１２３").is_err());
    assert!(UserUid::parse("abc").is_err());

    let audience = identity("9");
    assert_eq!(audience.user_level, Some(12));
    assert_eq!(audience.medal.unwrap().level, None);
    assert!(audience.tags.is_empty());

    let captain = normalize_identity(AudienceIdentityInput {
        user_uid: "8".into(),
        username: "captain".into(),
        user_level: None,
        medal_name: None,
        medal_level: None,
        is_admin: true,
        captain_level: Some(CaptainLevel::Governor),
        is_vip: true,
    })
    .unwrap();
    assert_eq!(
        captain.tags,
        [IdentityTag::Admin, IdentityTag::Governor, IdentityTag::Vip]
    );
}

#[test]
fn blacklist_wins_before_sensitive_matching_and_both_block_downstream() {
    let matcher = SensitiveMatcher::compile(&[SensitiveRule::new(
        1,
        "forbidden",
        SensitiveWordCategory::Illegal,
        SensitiveMatchType::Exact,
        true,
    )
    .unwrap()])
    .unwrap();
    let event = danmu("event-1", "9", "forbidden text", 1_000);
    let blacklist = HashSet::from([UserUid::parse("9").unwrap()]);

    assert!(matches!(
        evaluate_live_event(event.clone(), &blacklist, &matcher),
        PipelineOutcome::Blacklisted { .. }
    ));
    assert!(matches!(
        evaluate_live_event(event, &HashSet::new(), &matcher),
        PipelineOutcome::SensitiveFiltered { rule_id: 1, .. }
    ));
}

#[test]
fn compiles_exact_and_fuzzy_rules_and_rejects_invalid_patterns() {
    let matcher = SensitiveMatcher::compile(&[
        SensitiveRule::new(
            1,
            "exact phrase",
            SensitiveWordCategory::Custom,
            SensitiveMatchType::Exact,
            true,
        )
        .unwrap(),
        SensitiveRule::new(
            2,
            "sp.m",
            SensitiveWordCategory::Advertising,
            SensitiveMatchType::Fuzzy,
            true,
        )
        .unwrap(),
    ])
    .unwrap();

    assert_eq!(matcher.match_rule("has exact phrase"), Some(1));
    assert_eq!(matcher.match_rule("spam"), Some(2));
    assert!(SensitiveMatcher::compile(&[SensitiveRule::new(
        3,
        "(",
        SensitiveWordCategory::Custom,
        SensitiveMatchType::Fuzzy,
        true,
    )
    .unwrap(),])
    .is_err());
}

#[test]
fn aggregates_each_event_once_and_builds_the_five_metrics() {
    let first = danmu("event-1", "9", "hello", 1_000);
    let gift = LiveEvent::Gift {
        base: LiveEventBase {
            event_id: "event-2".into(),
            room_id: super::RoomId::parse("100").unwrap(),
            occurred_at: 2_000,
            audience: identity("9"),
        },
        gift_id: "gift-31036".into(),
        gift_name: "gift".into(),
        quantity: 2,
        value_minor: 300,
    };
    let captain = LiveEvent::Captain {
        base: LiveEventBase {
            event_id: "event-3".into(),
            room_id: super::RoomId::parse("100").unwrap(),
            occurred_at: 3_000,
            audience: identity("8"),
        },
        action: CaptainAction::Renewal,
        level: CaptainLevel::Captain,
        value_minor: 1_000,
    };

    let state = aggregate_event(AnalyticsState::default(), &first);
    let state = aggregate_event(state, &first);
    let state = aggregate_event(state, &gift);
    let state = aggregate_event(state, &captain).with_online_count(88);
    let snapshot = build_stat_snapshot(&state, 3_000);

    assert_eq!(snapshot.online_count, 88);
    assert_eq!(snapshot.danmu_count, 1);
    assert_eq!(snapshot.gift_value_minor, 300);
    assert_eq!(snapshot.captain_count, 1);
    assert_eq!(snapshot.danmu_rate, 0.5);
}

#[test]
fn ranking_is_stable_and_captain_value_accumulates_purchase_and_renewal() {
    let mut state = AnalyticsState::default();
    state = aggregate_event(state, &danmu("d1", "9", "one", 1_000));
    state = aggregate_event(state, &danmu("d2", "8", "two", 2_000));
    for (id, uid, action, value) in [
        ("c1", "9", CaptainAction::Purchase, 1_000),
        ("c2", "9", CaptainAction::Renewal, 2_000),
        ("c3", "8", CaptainAction::Purchase, 3_000),
    ] {
        state = aggregate_event(
            state,
            &LiveEvent::Captain {
                base: LiveEventBase {
                    event_id: id.into(),
                    room_id: super::RoomId::parse("100").unwrap(),
                    occurred_at: 3_000,
                    audience: identity(uid),
                },
                action,
                level: CaptainLevel::Captain,
                value_minor: value,
            },
        );
    }

    let ranking = build_ranking_snapshot(&state, RankingLimit::Top100);
    assert_eq!(ranking.danmu[0].user_uid.as_str(), "8");
    assert_eq!(ranking.captain[0].amount_minor, 3_000);
    assert_eq!(ranking.captain[0].user_uid.as_str(), "8");
    assert_eq!(ranking.captain[1].amount_minor, 3_000);
}

#[test]
fn ten_thousand_rule_and_blacklist_lookups_stay_within_the_acceptance_fixture() {
    let rules: Vec<_> = (1..=10_000)
        .map(|id| {
            SensitiveRule::new(
                id,
                &format!("blocked-{id}"),
                SensitiveWordCategory::Custom,
                SensitiveMatchType::Exact,
                true,
            )
            .unwrap()
        })
        .collect();
    let matcher = SensitiveMatcher::compile(&rules).unwrap();
    let started = Instant::now();
    assert_eq!(
        matcher.match_rule("prefix blocked-9999 suffix"),
        Some(9_999)
    );
    assert!(started.elapsed().as_millis() < 10);

    let blacklist: HashSet<_> = (1..=10_000)
        .map(|uid| UserUid::parse(&uid.to_string()).unwrap())
        .collect();
    let started = Instant::now();
    assert!(blacklist.contains(&UserUid::parse("9999").unwrap()));
    assert!(started.elapsed().as_millis() < 5);
}
