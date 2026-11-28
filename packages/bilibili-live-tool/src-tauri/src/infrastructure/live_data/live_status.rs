//! One-shot probe of a room's broadcasting state.
//!
//! The open-live long connection only pushes *transitions* (`LIVE_START` /
//! `LIVE_END`), so attaching in the middle of a stream leaves the state unknown
//! until the next transition. This asks the same read-only public endpoint the
//! recording path already uses, once, right after connecting.

use std::time::Duration;

use serde::Deserialize;

use crate::application::live_data::LiveStatusProbe;

const ROOM_INFO_ENDPOINT: &str = "https://api.live.bilibili.com/room/v1/Room/get_info";
const PROBE_TIMEOUT: Duration = Duration::from_secs(5);

/// Wiring used by the application: one read-only request per connection.
pub struct BilibiliLiveStatusProbe;

#[async_trait::async_trait]
impl LiveStatusProbe for BilibiliLiveStatusProbe {
    async fn probe(&self, room_id: &str) -> Option<bool> {
        probe(room_id).await
    }
}

#[derive(Deserialize)]
struct Envelope {
    data: Option<RoomInfo>,
}

#[derive(Deserialize)]
struct RoomInfo {
    live_status: i64,
}

/// `Some(true)` broadcasts, `Some(false)` offline (including 轮播),
/// `None` when the probe could not find out.
pub async fn probe(room_id: &str) -> Option<bool> {
    let client = reqwest::Client::builder()
        .timeout(PROBE_TIMEOUT)
        .build()
        .ok()?;
    let response = client
        .get(ROOM_INFO_ENDPOINT)
        .query(&[("room_id", room_id)])
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
        )
        .header(reqwest::header::REFERER, "https://live.bilibili.com/")
        .send()
        .await
        .ok()?;
    if !response.status().is_success() {
        return None;
    }
    let body = response.text().await.ok()?;
    let envelope: Envelope = serde_json::from_str(&body).ok()?;
    envelope.data.map(|info| info.live_status == 1)
}

#[cfg(test)]
mod tests {
    use super::{probe, Envelope};

    #[test]
    fn reads_the_live_status_field() {
        let envelope: Envelope =
            serde_json::from_str(r#"{"code":0,"data":{"live_status":1}}"#).unwrap();
        assert_eq!(envelope.data.map(|info| info.live_status), Some(1));
    }

    #[test]
    fn a_missing_data_block_is_not_live() {
        let envelope: Envelope = serde_json::from_str(r#"{"code":0}"#).unwrap();
        assert_eq!(envelope.data.map(|info| info.live_status), None);
    }

    #[tokio::test]
    #[ignore = "hits the public room endpoint"]
    async fn probes_a_known_room() {
        assert!(probe("22877456").await.is_some());
    }
}
