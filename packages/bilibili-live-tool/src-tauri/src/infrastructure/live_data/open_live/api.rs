use std::time::{Duration, SystemTime, UNIX_EPOCH};

use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::application::live_data::LiveRuntimeError;

use super::{config::OpenLiveConfig, signing};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartSession {
    pub game_id: String,
    pub auth_body: String,
    pub wss_links: Vec<String>,
    pub room_id: String,
    pub room_name: String,
}

#[derive(Clone)]
pub struct OpenLiveApi {
    config: OpenLiveConfig,
    client: reqwest::Client,
    base_url: String,
}

#[derive(Deserialize)]
struct ApiEnvelope<T> {
    code: i64,
    message: String,
    #[serde(default)]
    request_id: String,
    data: Option<T>,
}

#[derive(Deserialize)]
struct StartData {
    game_info: GameInfo,
    websocket_info: WebSocketInfo,
    anchor_info: AnchorInfo,
}

#[derive(Deserialize)]
struct GameInfo {
    game_id: String,
}

#[derive(Deserialize)]
struct WebSocketInfo {
    auth_body: String,
    wss_link: Vec<String>,
}

#[derive(Deserialize)]
struct AnchorInfo {
    room_id: u64,
    uname: String,
}

#[derive(Deserialize)]
struct EmptyData {}

#[derive(Serialize)]
struct StartRequest<'a> {
    code: &'a str,
    app_id: u64,
}

#[derive(Serialize)]
struct HeartbeatRequest<'a> {
    game_id: &'a str,
}

#[derive(Serialize)]
struct EndRequest<'a> {
    app_id: u64,
    game_id: &'a str,
}

impl OpenLiveApi {
    pub fn new(config: OpenLiveConfig, base_url: impl Into<String>) -> Self {
        Self {
            config,
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("reqwest client configuration is valid"),
            base_url: base_url.into().trim_end_matches('/').to_owned(),
        }
    }

    pub async fn start(&self, identity_code: &str) -> Result<StartSession, LiveRuntimeError> {
        let identity_code = validate_identity_code(identity_code)?;
        let data: StartData = self
            .post(
                "/v2/app/start",
                &StartRequest {
                    code: identity_code,
                    app_id: self.config.app_id,
                },
            )
            .await?;
        if data.game_info.game_id.trim().is_empty()
            || data.websocket_info.auth_body.trim().is_empty()
            || data.websocket_info.wss_link.is_empty()
            || data.anchor_info.room_id == 0
        {
            return Err(LiveRuntimeError::new(
                "invalid_open_live_response",
                "哔哩哔哩开放平台返回的连接信息不完整",
                false,
            ));
        }
        Ok(StartSession {
            game_id: data.game_info.game_id,
            auth_body: data.websocket_info.auth_body,
            wss_links: data.websocket_info.wss_link,
            room_id: data.anchor_info.room_id.to_string(),
            room_name: data.anchor_info.uname,
        })
    }

    pub async fn heartbeat(&self, game_id: &str) -> Result<(), LiveRuntimeError> {
        self.post::<_, EmptyData>("/v2/app/heartbeat", &HeartbeatRequest { game_id })
            .await
            .map(|_| ())
    }

    pub async fn end(&self, game_id: &str) -> Result<(), LiveRuntimeError> {
        self.post::<_, EmptyData>(
            "/v2/app/end",
            &EndRequest {
                app_id: self.config.app_id,
                game_id,
            },
        )
        .await
        .map(|_| ())
    }

    async fn post<T, R>(&self, path: &str, request: &T) -> Result<R, LiveRuntimeError>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let body = serde_json::to_vec(request).map_err(|_| {
            LiveRuntimeError::new(
                "SYS_OPEN_LIVE_ENCODING_FAILED",
                "哔哩哔哩开放平台请求编码失败",
                false,
            )
        })?;
        let nonce = Uuid::new_v4().to_string();
        let signed = signing::sign_request(
            &body,
            &self.config.access_key_id,
            self.config.access_key_secret(),
            &nonce,
            now_seconds(),
        );
        let response = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header("x-bili-content-md5", signed.content_md5)
            .header("x-bili-timestamp", signed.timestamp)
            .header("x-bili-signature-method", "HMAC-SHA256")
            .header("x-bili-signature-nonce", signed.nonce)
            .header("x-bili-accesskeyid", signed.access_key_id)
            .header("x-bili-signature-version", "1.0")
            .header(AUTHORIZATION, signed.authorization)
            .body(body)
            .send()
            .await
            .map_err(network_error)?;
        let status = response.status();
        let response_body = response.bytes().await.map_err(network_error)?;
        if !status.is_success() {
            return Err(LiveRuntimeError::new(
                "SYS_OPEN_LIVE_HTTP_ERROR",
                format!("哔哩哔哩开放平台 HTTP 错误：{status}"),
                status.is_server_error(),
            ));
        }
        let envelope: ApiEnvelope<R> = serde_json::from_slice(&response_body).map_err(|_| {
            LiveRuntimeError::new(
                "invalid_open_live_response",
                "哔哩哔哩开放平台响应格式无效",
                false,
            )
        })?;
        if envelope.code != 0 {
            log::warn!(
                "open live API rejected request: path={path}, code={}, request_id={}",
                envelope.code,
                envelope.request_id
            );
            return Err(LiveRuntimeError::new(
                "SYS_OPEN_LIVE_API_ERROR",
                format!(
                    "哔哩哔哩开放平台错误 {}：{}",
                    envelope.code, envelope.message
                ),
                envelope.code == 7001 || (5000..6000).contains(&envelope.code),
            ));
        }
        envelope.data.ok_or_else(|| {
            LiveRuntimeError::new(
                "invalid_open_live_response",
                "哔哩哔哩开放平台响应缺少 data",
                false,
            )
        })
    }
}

fn validate_identity_code(value: &str) -> Result<&str, LiveRuntimeError> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > 128 || value.chars().any(char::is_control) {
        return Err(LiveRuntimeError::new(
            "invalid_identity_code",
            "主播身份码无效",
            false,
        ));
    }
    Ok(value)
}

fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}

fn network_error(error: reqwest::Error) -> LiveRuntimeError {
    LiveRuntimeError::new(
        "SYS_OPEN_LIVE_NETWORK_ERROR",
        format!("哔哩哔哩开放平台网络请求失败：{error}"),
        true,
    )
}
