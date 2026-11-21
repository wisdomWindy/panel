use std::collections::BTreeMap;

use serde::Serialize;

use crate::application::member::MemberServiceError;
use crate::application::{
    config::ConfigError,
    live_data::LiveRuntimeError,
    media::MediaIoError,
    ports::{PathPortError, StoragePortError},
    release_ports::ReleaseServiceError,
    shell::ShellError,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DesktopError {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    details: BTreeMap<String, String>,
    retryable: bool,
}

impl DesktopError {
    pub fn new(code: impl Into<String>, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: BTreeMap::new(),
            retryable,
        }
    }

    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let normalized_key = key.to_ascii_lowercase();
        if !["token", "secret", "password", "path"]
            .iter()
            .any(|sensitive| normalized_key.contains(sensitive))
        {
            self.details.insert(key, value.into());
        }
        self
    }
}

impl From<StoragePortError> for DesktopError {
    fn from(error: StoragePortError) -> Self {
        Self::new(error.code, "Storage is unavailable", error.retryable)
    }
}

impl From<ConfigError> for DesktopError {
    fn from(error: ConfigError) -> Self {
        match error {
            ConfigError::InvalidKey => {
                Self::new("invalid_config_key", "Configuration key is invalid", false)
            }
            ConfigError::InvalidScope => Self::new(
                "invalid_config_scope",
                "Configuration scope is invalid",
                false,
            ),
            ConfigError::Storage(error) => Self::from(error),
        }
    }
}

impl From<PathPortError> for DesktopError {
    fn from(_: PathPortError) -> Self {
        Self::new(
            "path_authorization_failed",
            "The selected path could not be authorized",
            false,
        )
    }
}

impl From<ShellError> for DesktopError {
    fn from(error: ShellError) -> Self {
        Self::new(
            error.code,
            "Application shell operation failed",
            error.retryable,
        )
    }
}

impl From<LiveRuntimeError> for DesktopError {
    fn from(error: LiveRuntimeError) -> Self {
        let message = match error.code {
            "invalid_room_id" => "房间号无效或未开播",
            "invalid_identity_code" => "主播身份码无效",
            "missing_open_live_credentials" => "内置开放平台凭证无效，请重新安装应用",
            "invalid_open_live_app_id" => "开放平台 appid 无效",
            "open_live_api_error" => "开放平台拒绝连接，请检查凭证、appid 和主播身份码",
            "open_live_auth_failed" => "开放平台长链鉴权失败",
            "open_live_network_error"
            | "open_live_http_error"
            | "open_live_websocket_unavailable"
            | "open_live_websocket_closed"
            | "open_live_auth_timeout" => "开放平台网络连接失败，请稍后重试",
            "invalid_open_live_response" | "invalid_open_live_packet" => "开放平台返回的数据无效",
            "external_service_unavailable" => "开放平台凭证失效，请重新配置",
            "reconnect_failed" => "重连失败，请手动重试",
            "no_export_data" => "暂无数据可导出",
            "path_authorization_failed" => "文件授权已失效，请重新选择",
            "file_permission_denied" => "没有文件写入权限，请更换位置",
            "disk_full" => "磁盘空间不足，请清理后重试",
            "invalid_import_file" => "导入文件格式无效",
            "invalid_sensitive_word" | "invalid_sensitive_pattern" => "敏感词内容无效",
            "sensitive_word_not_found" => "敏感词不存在或已删除",
            "invalid_user_uid" => "用户 UID 无效",
            "audience_not_found" | "room_context_mismatch" => "当前直播间未找到该用户",
            "room_connection_pending" => "直播间正在连接，请稍候",
            "storage_unavailable" => "本地数据暂不可用，请稍后重试",
            "historical_data_unavailable" => "历史数据暂不可用",
            _ => "操作失败，请稍后重试",
        };
        Self::new(error.code, message, error.retryable)
    }
}

impl From<MediaIoError> for DesktopError {
    fn from(error: MediaIoError) -> Self {
        let message = match error.code {
            "media_invalid" => "配置内容无效，请检查后重试",
            "media_permission_denied" => "文件授权已失效，请重新选择",
            "media_conflict" => "当前状态不允许此操作",
            "media_unavailable" => "所需的本地功能当前不可用",
            "media_operation_failed" => "本地媒体操作失败，请稍后重试",
            "qwen3_tts_disabled" => "Qwen3-TTS 语音服务尚未启用",
            "qwen3_tts_unreachable" => "无法连接 Qwen3-TTS 语音服务，请确认服务已启动",
            "qwen3_tts_timeout" => "Qwen3-TTS 语音服务响应超时，已自动回退",
            "qwen3_tts_loading" => "Qwen3-TTS 模型正在加载，请稍后重试",
            "qwen3_tts_invalid_request" => "Qwen3-TTS 拒绝了本次合成请求，请检查文本与音色",
            "qwen3_tts_invalid_response" => "Qwen3-TTS 返回的音频无效",
            "qwen3_tts_invalid_config" => "Qwen3-TTS 配置无效，请检查服务地址、超时与语言",
            "qwen3_tts_synthesis_failed" => "Qwen3-TTS 语音合成失败，请稍后重试",
            _ => "操作失败，请稍后重试",
        };
        Self::new(error.code, message, error.retryable)
    }
}

impl From<ReleaseServiceError> for DesktopError {
    fn from(error: ReleaseServiceError) -> Self {
        let message = match error.code {
            "update_provider_unavailable" => "更新服务暂不可用",
            "update_check_failed" => "无法检查更新，请检查网络后重试",
            "update_not_verified" => "没有可安装的已验证更新",
            "invalid_update_version" | "invalid_update_manifest" => "更新清单无效",
            _ => "发布服务操作失败",
        };
        Self::new(error.code, message, error.retryable)
    }
}

#[allow(clippy::items_after_test_module)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_errors_do_not_report_membership_failures() {
        let error = DesktopError::from(MemberServiceError::invalid("ai_model_not_configured"));
        assert_eq!(error.message, "请先配置并选择默认模型");
        let error = DesktopError::from(MemberServiceError::invalid("ai_unmapped_error"));
        assert!(!error.message.contains("会员"));
    }

    #[test]
    fn update_check_failures_are_retryable_and_actionable() {
        let serialized = serde_json::to_value(DesktopError::from(ReleaseServiceError {
            code: "update_check_failed",
            retryable: true,
        }))
        .unwrap();

        assert_eq!(serialized["message"], "无法检查更新，请检查网络后重试");
        assert_eq!(serialized["retryable"], true);
    }
}

impl From<MemberServiceError> for DesktopError {
    fn from(error: MemberServiceError) -> Self {
        let message = match error.code {
            "auth_uid_invalid" => "UID 无效或查询失败",
            "ai_input_required" => "请输入内容或选择上下文",
            "ai_generation_pending" => "正在生成，请稍候",
            "ai_model_not_configured" => "请先配置并选择默认模型",
            "ai_api_key_required" => "请填写模型 API Key",
            "ai_api_key_invalid" => "模型 API Key 无效",
            "ai_model_profile_not_found" => "模型档案不存在，请刷新后重试",
            "ai_audio_route_unconfirmed" => "请先确认音频路由",
            "ai_audio_device_invalid" | "ai_audio_device_missing" => {
                "音频输出设备不可用，请重新选择"
            }
            "ai_tts_voice_invalid" => "所选声色不可用，请重新选择",
            "member_storage_unavailable" => "本地数据读取或保存失败，请稍后重试",
            "member_data_invalid" => "本地配置数据无效，请检查配置",
            "moderation_risk_required" => "首次启用自动禁言前需确认风险",
            "membership_service_unavailable" => "会员服务暂不可用，请稍后重试",
            "auth_qr_unavailable" => "扫码登录暂不可用",
            "llm_unavailable" => "AI 服务暂不可用",
            "moderation_api_unavailable" => "禁言接口暂不可用",
            code if code.starts_with("ai_") || code.starts_with("llm_") => {
                "AI 操作失败，请检查配置后重试"
            }
            _ => "操作失败，请稍后重试",
        };
        Self::new(error.code, message, error.retryable)
    }
}
