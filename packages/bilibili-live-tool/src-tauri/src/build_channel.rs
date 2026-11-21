#[cfg(not(any(feature = "free", feature = "member")))]
compile_error!("either feature `free` or `member` must be enabled");

#[cfg(all(feature = "free", feature = "member"))]
compile_error!("features `free` and `member` are mutually exclusive");

use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildChannel {
    Free,
    Member,
}

impl BuildChannel {
    pub const fn current() -> Self {
        #[cfg(feature = "member")]
        {
            Self::Member
        }

        #[cfg(all(feature = "free", not(feature = "member")))]
        {
            Self::Free
        }
    }
}

pub const fn foundation_command_names() -> &'static [&'static str] {
    &[
        "app_bootstrap",
        "app_status",
        "config_get",
        "config_set",
        "path_select",
        "path_revoke",
        "room_connect",
        "room_disconnect",
        "audience_detail_query",
        "sensitive_word_list",
        "sensitive_word_add",
        "sensitive_word_update",
        "sensitive_word_delete",
        "sensitive_word_toggle",
        "sensitive_word_import",
        "sensitive_word_export",
        "blacklist_list",
        "blacklist_add",
        "blacklist_remove",
        "blacklist_export",
        "blacklist_import",
        "stat_query",
        "stat_refresh",
        "stat_export",
        "rank_query",
        "rank_export",
    ]
}

pub const fn release_command_names() -> &'static [&'static str] {
    &[
        "update_status",
        "update_check",
        "update_download",
        "update_install",
        "update_skip",
        "about_info",
        "resilience_status",
    ]
}

pub const fn ai_command_names() -> &'static [&'static str] {
    &[
        "ai_config_get",
        "ai_config_save",
        "ai_model_profile_list",
        "ai_model_profile_save",
        "ai_model_profile_delete",
        "ai_model_set_default",
        "ai_model_test",
        "ai_voice_reply_get",
        "ai_voice_reply_set_enabled",
        "ai_voice_reply_update_rules",
        "ai_tts_list_output_devices",
        "ai_tts_set_output_device",
        "ai_tts_test",
        "ai_tts_pause",
        "ai_tts_skip",
        "ai_tts_clear_queue",
        "ai_generate",
        "ai_regenerate",
        "ai_log_query",
    ]
}

#[cfg(all(feature = "free", not(feature = "member")))]
pub const fn member_command_names() -> &'static [&'static str] {
    &[]
}

#[cfg(feature = "member")]
pub const fn member_command_names() -> &'static [&'static str] {
    &[
        "member_status",
        "auth_qrcode_generate",
        "auth_qrcode_cancel",
        "auth_query_uid",
        "auth_bind_uid",
        "auth_logout",
        "membership_purchase",
        "membership_cancel_order",
        "membership_orders",
        "module_list",
        "module_update",
        "module_cleanup",
        "moderate_status",
        "moderate_set_auto",
        "moderate_update_rules",
        "moderate_mute",
        "moderate_ignore",
        "moderate_log_query",
        "moderate_mark_false_positive",
    ]
}
