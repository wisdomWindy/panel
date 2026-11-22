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
                Self::new("CONFIG_INVALID_KEY", "Configuration key is invalid", false)
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
            "STORAGE_PATH_DENIED",
            "The selected path could not be authorized",
            false,
        )
    }
}

impl From<crate::application::data_maintenance::DataMaintenanceError> for DesktopError {
    fn from(error: crate::application::data_maintenance::DataMaintenanceError) -> Self {
        use crate::application::data_maintenance::DataMaintenanceError;
        match error {
            DataMaintenanceError::Database(_) => {
                Self::new("STORAGE_MAINTENANCE_FAILED", "Data maintenance failed", true)
            }
        }
    }
}

impl From<ShellError> for DesktopError {    fn from(error: ShellError) -> Self {
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
            "ROOM_INVALID_ID" => "房间号无效或未开播",
            "invalid_identity_code" => "主播身份码无效",
            "SYS_OPEN_LIVE_CREDENTIALS_MISSING" => "内置开放平台凭证无效，请重新安装应用",
            "invalid_open_live_app_id" => "开放平台 appid 无效",
            "SYS_OPEN_LIVE_API_ERROR" => "开放平台拒绝连接，请检查凭证、appid 和主播身份码",
            "SYS_OPEN_LIVE_AUTH_FAILED" => "开放平台长链鉴权失败",
            "SYS_OPEN_LIVE_NETWORK_ERROR"
            | "SYS_OPEN_LIVE_HTTP_ERROR"
            | "SYS_OPEN_LIVE_WS_UNAVAILABLE"
            | "open_live_websocket_closed"
            | "open_live_auth_timeout" => "开放平台网络连接失败，请稍后重试",
            "invalid_open_live_response" | "invalid_open_live_packet" => "开放平台返回的数据无效",
            "NET_SERVICE_UNAVAILABLE" => "开放平台凭证失效，请重新配置",
            "SYS_RECONNECT_FAILED" => "重连失败，请手动重试",
            "STORAGE_NO_EXPORT_DATA" => "暂无数据可导出",
            "STORAGE_PATH_DENIED" => "文件授权已失效，请重新选择",
            "STORAGE_PERMISSION_DENIED" => "没有文件写入权限，请更换位置",
            "disk_full" => "磁盘空间不足，请清理后重试",
            "RULE_INVALID_IMPORT" => "导入文件格式无效",
            "RULE_INVALID_WORD" | "invalid_sensitive_pattern" => "敏感词内容无效",
            "sensitive_word_not_found" => "敏感词不存在或已删除",
            "RULE_INVALID_USER" => "用户 UID 无效",
            "ROOM_AUDIENCE_NOT_FOUND" | "room_context_mismatch" => "当前直播间未找到该用户",
            "room_connection_pending" => "直播间正在连接，请稍候",
            "STORAGE_UNAVAILABLE" => "本地数据暂不可用，请稍后重试",
            "STORAGE_HISTORICAL_UNAVAILABLE" => "历史数据暂不可用",
            _ => "操作失败，请稍后重试",
        };
        Self::new(error.code, message, error.retryable)
    }
}

impl From<MediaIoError> for DesktopError {
    fn from(error: MediaIoError) -> Self {
        let message = match error.code {
            "RECORD_INVALID_PARAM" => "配置内容无效，请检查后重试",
            "SYS_PERMISSION_DENIED" => "文件授权已失效，请重新选择",
            "RECORD_CONFLICT" => "当前状态不允许此操作",
            "RECORD_UNAVAILABLE" => "所需的本地功能当前不可用",
            "RECORD_OPERATION_FAILED" => "本地媒体操作失败，请稍后重试",
            "qwen3_tts_disabled" => "Qwen3-TTS 语音服务尚未启用",
            "qwen3_tts_unreachable" => "无法连接 Qwen3-TTS 语音服务，请确认服务已启动",
            "TTS_TIMEOUT" => "Qwen3-TTS 语音服务响应超时，已自动回退",
            "TTS_MODEL_LOADING" => "Qwen3-TTS 模型正在加载，请稍后重试",
            "qwen3_tts_invalid_request" => "Qwen3-TTS 拒绝了本次合成请求，请检查文本与音色",
            "qwen3_tts_invalid_response" => "Qwen3-TTS 返回的音频无效",
            "TTS_INVALID_CONFIG" => "Qwen3-TTS 配置无效，请检查服务地址、超时与语言",
            "TTS_SYNTHESIS_FAILED_ENGINE" => "Qwen3-TTS 语音合成失败，请稍后重试",
            "qwen3_tts_model_unknown" => "不支持该语音模型",
            "qwen3_tts_download_busy" => "已有模型正在下载，请等待完成",
            "TTS_DOWNLOAD_UNAVAILABLE" => "模型下载不可用，请安装 huggingface_hub 或 modelscope",
            "ROOM_INVALID_ROOM" => "请输入正确的直播间号",
            "ROOM_OFFLINE" => "直播间当前未开播，开始直播后再试",
            "RECORD_STREAM_UNAVAILABLE" => "无法获取直播流地址，请稍后重试",
            "RECORD_DIRECTORY_REQUIRED" => "请先在录制设置中授权保存目录",
            "RECORD_DIRECTORY_UNAVAILABLE" => "录制目录不可写，请更换保存位置",
            "RECORD_FFMPEG_DOWNLOAD_FAILED" => "FFmpeg 安装包下载失败，请检查网络后重试",
            "RECORD_FFMPEG_INSTALL_FAILED" => "FFmpeg 安装失败，请检查磁盘空间与权限后重试",
            "RECORD_FFMPEG_UNSUPPORTED" => "当前平台请通过系统包管理器安装 FFmpeg",
            "RECORD_FFMPEG_MISSING" => "未找到可用的 FFmpeg，请在录制设置中选择",
            "RECORD_PIPELINE_INVALID" => "录制参数无效，请检查录制设置",
            "RECORD_START_FAILED" => "录制启动失败，请检查 FFmpeg 与网络后重试",
            "recording_disk_waterline" => "磁盘可用空间低于水位线，已停止录制",
            _ => "操作失败，请稍后重试",
        };
        Self::new(error.code, message, error.retryable)
    }
}

impl From<ReleaseServiceError> for DesktopError {
    fn from(error: ReleaseServiceError) -> Self {
        let message = match error.code {
            "UPDATE_PROVIDER_UNAVAILABLE" => "更新服务暂不可用",
            "UPDATE_CHECK_FAILED" => "无法检查更新，请检查网络后重试",
            "UPDATE_NOT_VERIFIED" => "没有可安装的已验证更新",
            "UPDATE_INVALID_VERSION" | "UPDATE_INVALID_MANIFEST" => "更新清单无效",
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
        let error = DesktopError::from(MemberServiceError::invalid("AI_MODEL_NOT_CONFIGURED"));
        assert_eq!(error.message, "请先配置并选择默认模型");
        let error = DesktopError::from(MemberServiceError::invalid("AI_GEN_UNMAPPED_ERROR"));
        assert!(!error.message.contains("会员"));
    }

    #[test]
    fn update_check_failures_are_retryable_and_actionable() {
        let serialized = serde_json::to_value(DesktopError::from(ReleaseServiceError {
            code: "UPDATE_CHECK_FAILED",
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
            "MEMBER_AUTH_UID_INVALID" => "UID 无效或查询失败",
            "AI_GEN_INPUT_REQUIRED" => "请输入内容或选择上下文",
            "AI_GEN_PENDING" => "正在生成，请稍候",
            "AI_MODEL_NOT_CONFIGURED" => "请先配置并选择默认模型",
            "AI_MODEL_API_KEY_REQUIRED" => "请填写模型 API Key",
            "AI_MODEL_API_KEY_INVALID" => "模型 API Key 无效",
            "AI_MODEL_PROFILE_NOT_FOUND" => "模型档案不存在，请刷新后重试",
            "AI_MODEL_LIST_UNAVAILABLE" => "无法获取该服务商的模型列表，请检查地址与 API Key",
            "AI_MODEL_LIST_INVALID" => "服务商返回的模型列表无法解析",
            "AUDIO_ROUTE_UNCONFIRMED" => "请先确认音频路由",
            "AUDIO_DEVICE_INVALID" | "AUDIO_DEVICE_MISSING" => {
                "音频输出设备不可用，请重新选择"
            }
            "TTS_VOICE_INVALID" => "所选声色不可用，请重新选择",
            "STORAGE_MEMBER_UNAVAILABLE" => "本地数据读取或保存失败，请稍后重试",
            "MEMBER_DATA_INVALID" => "本地配置数据无效，请检查配置",
            "MEMBER_MODERATION_RISK_REQUIRED" => "首次启用自动禁言前需确认风险",
            "MEMBER_SERVICE_UNAVAILABLE" => "会员服务暂不可用，请稍后重试",
            "MEMBER_AUTH_QR_UNAVAILABLE" => "扫码登录暂不可用",
            "NET_LLM_UNAVAILABLE" => "AI 服务暂不可用",
            "MEMBER_MODERATION_API_UNAVAILABLE" => "禁言接口暂不可用",
            code if code.starts_with("ai_") || code.starts_with("llm_") => {
                "AI 操作失败，请检查配置后重试"
            }
            _ => "操作失败，请稍后重试",
        };
        Self::new(error.code, message, error.retryable)
    }
}
