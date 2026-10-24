use super::{
    AiCapabilityState, Entitlement, EntitlementValidity, MemberModuleStatus, ModerationDefaults,
};

pub const QR_POLL_INTERVAL_SECONDS: u16 = 2;
pub const QR_EXPIRES_SECONDS: u16 = 180;
pub const QR_MAX_POLLS: u16 = 90;
pub const QR_MAX_CONSECUTIVE_FAILURES: u8 = 5;
pub const OFFLINE_VALIDITY_MILLIS: i64 = 7 * 24 * 60 * 60 * 1_000;
pub const AI_ASSEMBLY_TARGET_MILLIS: u16 = 500;
pub const DEFAULT_LLM_TIMEOUT_MILLIS: u16 = 5_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ModuleProgressError {
    #[error("module byte totals are invalid")]
    InvalidByteTotals,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum AiTextError {
    #[error("maximum AI output length must be positive")]
    InvalidMaximumLength,
    #[error("AI output contains a sensitive term")]
    SensitiveTerm,
}

pub fn evaluate_entitlement(
    entitlement: &Entitlement,
    now_millis: i64,
    is_online: bool,
) -> EntitlementValidity {
    if entitlement.issued_at <= 0
        || entitlement.last_verified_at < entitlement.issued_at
        || entitlement.expires_at <= entitlement.issued_at
        || now_millis < entitlement.issued_at
        || now_millis < entitlement.last_verified_at
    {
        return EntitlementValidity::InvalidClock;
    }

    if entitlement.status == super::EntitlementStatus::Expired
        || now_millis >= entitlement.expires_at
    {
        return EntitlementValidity::Expired;
    }

    if is_online {
        return EntitlementValidity::ActiveOnline;
    }

    if now_millis - entitlement.last_verified_at <= OFFLINE_VALIDITY_MILLIS {
        EntitlementValidity::ActiveOffline
    } else {
        EntitlementValidity::VerificationRequired
    }
}

pub fn derive_capability(
    validity: EntitlementValidity,
    module_status: MemberModuleStatus,
) -> AiCapabilityState {
    match validity {
        EntitlementValidity::Expired => AiCapabilityState::Expired,
        EntitlementValidity::ActiveOnline | EntitlementValidity::ActiveOffline => {
            if module_status == MemberModuleStatus::Installed {
                AiCapabilityState::Ready
            } else {
                AiCapabilityState::MissingModule
            }
        }
        EntitlementValidity::VerificationRequired | EntitlementValidity::InvalidClock => {
            AiCapabilityState::Unavailable
        }
    }
}

pub fn module_progress(
    downloaded_bytes: u64,
    total_bytes: Option<u64>,
) -> Result<u8, ModuleProgressError> {
    match total_bytes {
        None if downloaded_bytes == 0 => Ok(0),
        None => Err(ModuleProgressError::InvalidByteTotals),
        Some(0) if downloaded_bytes == 0 => Ok(0),
        Some(0) => Err(ModuleProgressError::InvalidByteTotals),
        Some(total) if downloaded_bytes <= total => {
            Ok(((u128::from(downloaded_bytes) * 100) / u128::from(total)) as u8)
        }
        Some(_) => Err(ModuleProgressError::InvalidByteTotals),
    }
}

pub fn filter_and_truncate(
    input: &str,
    sensitive_terms: &[String],
    maximum_chars: usize,
) -> Result<String, AiTextError> {
    if maximum_chars == 0 {
        return Err(AiTextError::InvalidMaximumLength);
    }

    if sensitive_terms
        .iter()
        .filter(|term| !term.is_empty())
        .any(|term| input.contains(term))
    {
        return Err(AiTextError::SensitiveTerm);
    }

    let flattened = input
        .lines()
        .map(strip_markdown_prefix)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let without_markdown = flattened.replace(['*', '`', '#', '_'], "");
    let normalized = without_markdown
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let conversational = strip_ai_preamble(&normalized);

    Ok(conversational.chars().take(maximum_chars).collect())
}

pub fn truncate_sentences(input: &str, maximum_sentences: usize) -> String {
    if maximum_sentences == 0 {
        return String::new();
    }
    let mut sentences = 0;
    let mut output = String::new();
    for character in input.chars() {
        output.push(character);
        if matches!(character, '。' | '！' | '？' | '!' | '?') {
            sentences += 1;
            if sentences >= maximum_sentences {
                break;
            }
        }
    }
    output.trim().to_owned()
}

fn strip_markdown_prefix(line: &str) -> &str {
    let trimmed = line.trim();
    let without_bullet = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
        .or_else(|| trimmed.strip_prefix("+ "))
        .unwrap_or(trimmed);
    let digit_count = without_bullet
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .count();
    let remainder = &without_bullet[digit_count..];
    if digit_count > 0 {
        remainder
            .strip_prefix(". ")
            .or_else(|| remainder.strip_prefix(") "))
            .or_else(|| remainder.strip_prefix("、"))
            .unwrap_or(without_bullet)
    } else {
        without_bullet.trim_start_matches('#').trim_start()
    }
}

fn strip_ai_preamble(input: &str) -> &str {
    const PREAMBLES: &[&str] = &[
        "当然可以，",
        "当然可以。",
        "好的，",
        "您好，",
        "作为 AI，",
        "作为AI，",
        "作为一个 AI，",
        "我是 AI，",
        "我是AI，",
    ];
    PREAMBLES
        .iter()
        .find_map(|preamble| input.strip_prefix(preamble))
        .unwrap_or(input)
        .trim_start()
}

pub const fn moderation_defaults() -> ModerationDefaults {
    ModerationDefaults {
        auto_mute_enabled: false,
        frequency_window_seconds: 10,
        frequency_limit: 5,
        duplicate_window_seconds: 60,
        duplicate_limit: 3,
        similarity_window_seconds: 60,
        similarity_limit: 5,
        similarity_threshold_percent: 80,
        mute_duration_seconds: 3_600,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::member::EntitlementStatus;

    fn entitlement(last_verified_at: i64, expires_at: i64) -> Entitlement {
        Entitlement {
            status: EntitlementStatus::Active,
            plan_id: "annual".to_owned(),
            expires_at,
            issued_at: 1_000,
            last_verified_at,
        }
    }

    #[test]
    fn entitlement_is_valid_offline_for_exactly_seven_days() {
        let value = entitlement(10_000, 10_000 + OFFLINE_VALIDITY_MILLIS + 1_000);

        assert_eq!(
            evaluate_entitlement(&value, 10_000 + OFFLINE_VALIDITY_MILLIS, false),
            EntitlementValidity::ActiveOffline
        );
        assert_eq!(
            evaluate_entitlement(&value, 10_001 + OFFLINE_VALIDITY_MILLIS, false),
            EntitlementValidity::VerificationRequired
        );
    }

    #[test]
    fn entitlement_rejects_expiry_and_impossible_clock_values() {
        let value = entitlement(2_000, 5_000);

        assert_eq!(
            evaluate_entitlement(&value, 5_000, true),
            EntitlementValidity::Expired
        );
        assert_eq!(
            evaluate_entitlement(&value, 999, true),
            EntitlementValidity::InvalidClock
        );
    }

    #[test]
    fn capability_requires_valid_entitlement_and_installed_module() {
        assert_eq!(
            derive_capability(
                EntitlementValidity::ActiveOffline,
                MemberModuleStatus::Installed
            ),
            AiCapabilityState::Ready
        );
        assert_eq!(
            derive_capability(
                EntitlementValidity::ActiveOnline,
                MemberModuleStatus::Missing
            ),
            AiCapabilityState::MissingModule
        );
        assert_eq!(
            derive_capability(EntitlementValidity::Expired, MemberModuleStatus::Installed),
            AiCapabilityState::Expired
        );
    }

    #[test]
    fn progress_uses_verified_byte_totals_and_rejects_impossible_values() {
        assert_eq!(module_progress(50, Some(200)), Ok(25));
        assert_eq!(module_progress(0, None), Ok(0));
        assert_eq!(
            module_progress(201, Some(200)),
            Err(ModuleProgressError::InvalidByteTotals)
        );
        assert_eq!(
            module_progress(1, Some(0)),
            Err(ModuleProgressError::InvalidByteTotals)
        );
    }

    #[test]
    fn ai_output_filters_before_unicode_safe_truncation() {
        let banned = vec!["forbidden".to_owned()];

        assert_eq!(
            filter_and_truncate("你好世界abc", &banned, 5),
            Ok("你好世界a".to_owned())
        );
        assert_eq!(
            filter_and_truncate("safe forbidden text", &banned, 100),
            Err(AiTextError::SensitiveTerm)
        );
        assert_eq!(
            filter_and_truncate("safe", &banned, 0),
            Err(AiTextError::InvalidMaximumLength)
        );
    }

    #[test]
    fn ai_output_is_flattened_and_strips_markdown_and_ai_preambles() {
        assert_eq!(
            filter_and_truncate("作为 AI，\n1. **今天状态很好**\n- 谢谢大家", &[], 50),
            Ok("今天状态很好 谢谢大家".to_owned())
        );
        assert_eq!(
            filter_and_truncate("当然可以，这就开始", &[], 50),
            Ok("这就开始".to_owned())
        );
    }

    #[test]
    fn ai_output_respects_the_sentence_limit() {
        assert_eq!(
            truncate_sentences("第一句。第二句！第三句？", 2),
            "第一句。第二句！"
        );
    }

    #[test]
    fn moderation_defaults_match_product_contract() {
        assert_eq!(
            moderation_defaults(),
            ModerationDefaults {
                auto_mute_enabled: false,
                frequency_window_seconds: 10,
                frequency_limit: 5,
                duplicate_window_seconds: 60,
                duplicate_limit: 3,
                similarity_window_seconds: 60,
                similarity_limit: 5,
                similarity_threshold_percent: 80,
                mute_duration_seconds: 3_600,
            }
        );
    }
}
