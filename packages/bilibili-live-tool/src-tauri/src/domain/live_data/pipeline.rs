use std::collections::HashSet;

use super::{LiveEvent, SensitiveMatcher, UserUid};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PipelineOutcome {
    Accepted(LiveEvent),
    SensitiveFiltered { event: LiveEvent, rule_id: i64 },
    Blacklisted { event: LiveEvent, user_uid: UserUid },
    Invalid { event: LiveEvent },
}

pub fn evaluate_live_event(
    event: LiveEvent,
    blacklist: &HashSet<UserUid>,
    matcher: &SensitiveMatcher,
) -> PipelineOutcome {
    if event.validate().is_err() {
        return PipelineOutcome::Invalid { event };
    }
    let user_uid = event.base().audience.user_uid.clone();
    if blacklist.contains(&user_uid) {
        return PipelineOutcome::Blacklisted { event, user_uid };
    }
    if let LiveEvent::Danmu { content, .. } = &event {
        if let Some(rule_id) = matcher.match_rule(content) {
            return PipelineOutcome::SensitiveFiltered { event, rule_id };
        }
    }
    PipelineOutcome::Accepted(event)
}
