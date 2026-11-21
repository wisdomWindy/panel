//! Bilibili live play-URL resolution.
//!
//! The recorder needs one thing from the platform: a URL FFmpeg can pull. The
//! legacy `room/v1/Room/playUrl` endpoint answers anonymously, so recording
//! works without the operator logging in.

use serde::Deserialize;

use crate::{
    application::media::{MediaIoError, PlayUrlResolver, ResolvedStream},
    domain::media::{
        RecordingQuality, RECORDING_REFERER, RECORDING_USER_AGENT,
    },
};

const ENDPOINT: &str = "https://api.live.bilibili.com/room/v1/Room/playUrl";
const ROOM_INFO_ENDPOINT: &str = "https://api.live.bilibili.com/room/v1/Room/get_info";

#[derive(Default)]
pub struct BilibiliPlayUrlResolver {
    client: reqwest::Client,
}

impl BilibiliPlayUrlResolver {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
        }
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// The play-URL endpoint happily returns a stale address for an ended
    /// stream, which the CDN then rejects with 403. Checking the room state
    /// first turns that into an actionable "未开播" instead of a recorder crash.
    async fn ensure_live(&self, room_id: &str) -> Result<(), MediaIoError> {
        let response = self
            .client
            .get(ROOM_INFO_ENDPOINT)
            .query(&[("room_id", room_id)])
            .header(reqwest::header::USER_AGENT, RECORDING_USER_AGENT)
            .header(reqwest::header::REFERER, RECORDING_REFERER)
            .send()
            .await
            .map_err(|error| {
                MediaIoError::coded(
                    "RECORD_STREAM_UNAVAILABLE",
                    format!("直播间状态查询失败：{error}"),
                    true,
                )
            })?;
        let body = response.text().await.map_err(|error| {
            MediaIoError::coded(
                "RECORD_STREAM_UNAVAILABLE",
                format!("直播间状态读取失败：{error}"),
                true,
            )
        })?;
        parse_live_status(&body)
    }
}

#[async_trait::async_trait]
impl PlayUrlResolver for BilibiliPlayUrlResolver {
    async fn resolve(
        &self,
        room_id: &str,
        quality: RecordingQuality,
    ) -> Result<ResolvedStream, MediaIoError> {
        let room_id = validate_room_id(room_id)?;
        self.ensure_live(room_id).await?;
        let request = self
            .client
            .get(ENDPOINT)
            .query(&[
                ("cid", room_id),
                ("qn", &quality_qn(quality).to_string()),
                ("platform", "web"),
            ])
            .header(reqwest::header::USER_AGENT, RECORDING_USER_AGENT)
            .header(reqwest::header::REFERER, RECORDING_REFERER);
        let response = request
            .send()
            .await
            .map_err(|error| {
                MediaIoError::coded(
                    "RECORD_STREAM_UNAVAILABLE",
                    format!("直播流地址获取失败：{error}"),
                    true,
                )
            })?;
        if !response.status().is_success() {
            return Err(MediaIoError::coded(
                "RECORD_STREAM_UNAVAILABLE",
                format!("直播流地址获取失败：HTTP {}", response.status().as_u16()),
                true,
            ));
        }
        let body = response.text().await.map_err(|error| {
            MediaIoError::coded(
                "RECORD_STREAM_UNAVAILABLE",
                format!("直播流地址读取失败：{error}"),
                true,
            )
        })?;
        parse_play_url_response(&body)
    }
}

/// `qn` values accepted by the play-URL endpoint (best available first).
pub const fn quality_qn(quality: RecordingQuality) -> u32 {
    match quality {
        RecordingQuality::Original => 10_000,
        RecordingQuality::High => 400,
        RecordingQuality::Standard => 250,
    }
}

fn quality_from_qn(qn: u32) -> Option<RecordingQuality> {
    match qn {
        10_000 => Some(RecordingQuality::Original),
        400 => Some(RecordingQuality::High),
        250 => Some(RecordingQuality::Standard),
        _ => None,
    }
}

pub fn validate_room_id(room_id: &str) -> Result<&str, MediaIoError> {
    crate::domain::media::validate_recording_room_id(room_id).map_err(|_| {
        MediaIoError::coded("ROOM_INVALID_ROOM", "请输入正确的直播间号", false)
    })
}

#[derive(Debug, Deserialize)]
struct RoomInfoEnvelope {
    code: i64,
    #[serde(default)]
    message: String,
    #[serde(default)]
    data: Option<RoomInfoData>,
}

#[derive(Debug, Deserialize)]
struct RoomInfoData {
    #[serde(default)]
    live_status: i64,
}

/// `live_status == 1` means the room is broadcasting right now.
pub fn parse_live_status(body: &str) -> Result<(), MediaIoError> {
    let envelope: RoomInfoEnvelope = serde_json::from_str(body).map_err(|_| {
        MediaIoError::coded(
            "RECORD_STREAM_UNAVAILABLE",
            "直播间状态响应格式无法识别",
            true,
        )
    })?;
    if envelope.code != 0 {
        let message = envelope.message.trim();
        let message = if message.is_empty() {
            format!("错误码 {}", envelope.code)
        } else {
            message.to_owned()
        };
        if is_offline_message(&message) {
            return Err(MediaIoError::coded("ROOM_OFFLINE", message, false));
        }
        return Err(MediaIoError::coded(
            "RECORD_STREAM_UNAVAILABLE",
            format!("直播间状态查询失败：{message}"),
            true,
        ));
    }
    match envelope.data {
        Some(data) if data.live_status == 1 => Ok(()),
        _ => Err(MediaIoError::coded(
            "ROOM_OFFLINE",
            "直播间当前未开播，开始直播后再试",
            false,
        )),
    }
}

#[derive(Debug, Deserialize)]
struct Envelope {
    code: i64,
    #[serde(default)]
    message: String,
    #[serde(default)]
    data: Option<Data>,
}

#[derive(Debug, Default, Deserialize)]
struct Data {
    #[serde(default)]
    durl: Vec<Durl>,
    #[serde(default)]
    quality_description: Vec<QualityDescription>,
}

#[derive(Debug, Deserialize)]
struct Durl {
    #[serde(default)]
    url: String,
}

#[derive(Debug, Deserialize)]
struct QualityDescription {
    #[serde(default)]
    qn: u32,
}

/// Parses a play-URL response body into a stream location.
///
/// Split out from the HTTP call so every failure mode below is covered by a
/// unit test instead of a live request.
pub fn parse_play_url_response(body: &str) -> Result<ResolvedStream, MediaIoError> {
    let envelope: Envelope = serde_json::from_str(body).map_err(|_| {
        MediaIoError::coded(
            "RECORD_STREAM_UNAVAILABLE",
            "直播流地址响应格式无法识别",
            true,
        )
    })?;
    if envelope.code != 0 {
        let message = envelope.message.trim();
        let message = if message.is_empty() {
            format!("错误码 {}", envelope.code)
        } else {
            message.to_owned()
        };
        if is_offline_message(&message) {
            return Err(MediaIoError::coded("ROOM_OFFLINE", message, false));
        }
        return Err(MediaIoError::coded(
            "RECORD_STREAM_UNAVAILABLE",
            format!("直播流地址获取失败：{message}"),
            true,
        ));
    }
    let data = envelope.data.unwrap_or_default();
    let url = data
        .durl
        .into_iter()
        .map(|entry| entry.url)
        .find(|url| !url.trim().is_empty())
        .ok_or_else(|| {
            MediaIoError::coded(
                "RECORD_STREAM_UNAVAILABLE",
                "当前直播没有可用的拉流地址",
                true,
            )
        })?;
    let available_qualities = data
        .quality_description
        .into_iter()
        .filter_map(|entry| quality_from_qn(entry.qn))
        .collect();
    Ok(ResolvedStream {
        url,
        available_qualities,
    })
}

fn is_offline_message(message: &str) -> bool {
    ["未开播", "不存在", "已关闭", "下播", "没有开播", "not live", "offline"]
        .iter()
        .any(|needle| message.to_lowercase().contains(&needle.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::{parse_live_status, parse_play_url_response, quality_qn, validate_room_id};
    use crate::domain::media::RecordingQuality;

    #[test]
    fn accepts_a_room_that_is_broadcasting() {
        assert!(parse_live_status(r#"{"code":0,"message":"OK","data":{"live_status":1}}"#).is_ok());
    }

    #[test]
    fn rejects_a_room_that_is_not_broadcasting() {
        let error =
            parse_live_status(r#"{"code":0,"message":"OK","data":{"live_status":0}}"#).unwrap_err();
        assert_eq!(error.code, "ROOM_OFFLINE");
        assert!(!error.retryable);
        assert!(error.message.contains("未开播"));
    }

    #[test]
    fn rejects_a_room_that_does_not_exist() {
        let error = parse_live_status(
            r#"{"code":19002003,"message":"房间信息不存在","data":null}"#,
        )
        .unwrap_err();
        assert_eq!(error.code, "ROOM_OFFLINE");
    }

    #[test]
    fn parses_a_live_room_into_a_stream_url() {
        let body = r#"{"code":0,"message":"OK","data":{"current_qn":10000,"durl":[{"url":"https://cdn.example/live.flv?expires=1","length":0,"order":1,"stream_type":0}],"quality_description":[{"qn":10000,"desc":"原画"},{"qn":400,"desc":"蓝光"}]}}"#;
        let stream = parse_play_url_response(body).unwrap();
        assert_eq!(stream.url, "https://cdn.example/live.flv?expires=1");
        assert_eq!(
            stream.available_qualities,
            vec![RecordingQuality::Original, RecordingQuality::High]
        );
    }

    #[test]
    fn treats_an_offline_room_as_a_non_retryable_error() {
        let body = r#"{"code":19002003,"message":"房间信息不存在","data":null}"#;
        let error = parse_play_url_response(body).unwrap_err();
        assert_eq!(error.code, "ROOM_OFFLINE");
        assert!(!error.retryable);
        assert_eq!(error.message, "房间信息不存在");
    }

    #[test]
    fn treats_an_unknown_platform_error_as_retryable() {
        let body = r#"{"code":-400,"message":"请求错误","data":null}"#;
        let error = parse_play_url_response(body).unwrap_err();
        assert_eq!(error.code, "RECORD_STREAM_UNAVAILABLE");
        assert!(error.retryable);
    }

    #[test]
    fn reports_a_room_without_any_stream_address() {
        let body = r#"{"code":0,"message":"OK","data":{"durl":[]}}"#;
        let error = parse_play_url_response(body).unwrap_err();
        assert_eq!(error.code, "RECORD_STREAM_UNAVAILABLE");
    }

    #[test]
    fn ignores_empty_addresses_in_favour_of_the_next_one() {
        let body = r#"{"code":0,"message":"OK","data":{"durl":[{"url":"   "},{"url":"https://cdn.example/second.flv"}]}}"#;
        assert_eq!(
            parse_play_url_response(body).unwrap().url,
            "https://cdn.example/second.flv"
        );
    }

    #[test]
    fn rejects_a_response_that_is_not_json() {
        let error = parse_play_url_response("<html>blocked</html>").unwrap_err();
        assert_eq!(error.code, "RECORD_STREAM_UNAVAILABLE");
    }

    #[test]
    fn validates_numeric_room_ids() {
        assert_eq!(validate_room_id(" 21452505 ").unwrap(), "21452505");
        assert_eq!(validate_room_id("").unwrap_err().code, "ROOM_INVALID_ROOM");
        assert_eq!(validate_room_id("abc").unwrap_err().code, "ROOM_INVALID_ROOM");
        assert_eq!(
            validate_room_id("1; rm -rf /").unwrap_err().code,
            "ROOM_INVALID_ROOM"
        );
    }

    #[test]
    fn maps_quality_to_the_platform_qn_values() {
        assert_eq!(quality_qn(RecordingQuality::Original), 10_000);
        assert_eq!(quality_qn(RecordingQuality::High), 400);
        assert_eq!(quality_qn(RecordingQuality::Standard), 250);
    }
}
