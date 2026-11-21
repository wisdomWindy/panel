use std::time::Duration;

use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use serde_json::Value;
use tokio::{net::TcpStream, sync::mpsc, time::Instant};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    application::live_data::{LiveRuntimeError, RoomConnection, RoomTransport},
    domain::live_data::{LiveEvent, RoomId},
};

use super::{
    api::{OpenLiveApi, StartSession},
    config::OpenLiveConfig,
    event_mapper, protocol,
};

const OP_HEARTBEAT: u32 = 2;
const OP_SEND_MESSAGE: u32 = 5;
const OP_AUTH: u32 = 7;
const OP_AUTH_REPLY: u32 = 8;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(20);
const AUTH_TIMEOUT: Duration = Duration::from_secs(10);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct BilibiliOpenLiveTransport {
    api: OpenLiveApi,
    heartbeat_interval: Duration,
}

impl BilibiliOpenLiveTransport {
    pub fn new(config: OpenLiveConfig) -> Self {
        Self::with_api(OpenLiveApi::new(config, "https://live-open.biliapi.com"))
    }

    pub(super) fn with_api(api: OpenLiveApi) -> Self {
        Self {
            api,
            heartbeat_interval: HEARTBEAT_INTERVAL,
        }
    }

    #[cfg(test)]
    pub(super) fn with_api_and_heartbeat(api: OpenLiveApi, heartbeat_interval: Duration) -> Self {
        Self {
            api,
            heartbeat_interval,
        }
    }
}

#[async_trait::async_trait]
impl RoomTransport for BilibiliOpenLiveTransport {
    async fn connect(&self, identity_code: &str) -> Result<RoomConnection, LiveRuntimeError> {
        let session = self.api.start(identity_code).await?;
        let room_id = RoomId::parse(&session.room_id).map_err(|_| {
            LiveRuntimeError::new(
                "invalid_open_live_response",
                "哔哩哔哩开放平台返回的房间号无效",
                false,
            )
        })?;
        let connection = connect_any(&session, CONNECT_TIMEOUT).await;
        let (socket, pending_packets) = match connection {
            Ok(value) => value,
            Err(error) => {
                let _ = self.api.end(&session.game_id).await;
                return Err(error);
            }
        };
        let (sender, receiver) = mpsc::channel(256);
        let api = self.api.clone();
        let heartbeat_interval = self.heartbeat_interval;
        let game_id = session.game_id;
        tokio::spawn(async move {
            run_session(
                api,
                game_id,
                socket,
                pending_packets,
                sender,
                heartbeat_interval,
            )
            .await;
        });
        Ok(RoomConnection {
            room_id,
            room_name: Some(session.room_name),
            online_count: None,
            events: receiver,
        })
    }
}

pub(super) async fn connect_any(
    session: &StartSession,
    connect_timeout: Duration,
) -> Result<(Socket, Vec<protocol::Packet>), LiveRuntimeError> {
    let mut last_error = None;
    for link in &session.wss_links {
        match tokio::time::timeout(connect_timeout, connect_async(link)).await {
            Ok(Ok((mut socket, _))) => match authenticate(&mut socket, &session.auth_body).await {
                Ok(packets) => return Ok((socket, packets)),
                Err(error) => last_error = Some(error),
            },
            Ok(Err(error)) => {
                last_error = Some(websocket_error(error));
            }
            Err(_) => last_error = Some(websocket_timeout()),
        }
    }
    Err(last_error.unwrap_or_else(|| {
        LiveRuntimeError::new(
            "SYS_OPEN_LIVE_WS_UNAVAILABLE",
            "哔哩哔哩开放平台没有可用的 WebSocket 地址",
            true,
        )
    }))
}

async fn authenticate(
    socket: &mut Socket,
    auth_body: &str,
) -> Result<Vec<protocol::Packet>, LiveRuntimeError> {
    socket
        .send(Message::Binary(protocol::encode_packet(
            0,
            OP_AUTH,
            1,
            auth_body.as_bytes(),
        )))
        .await
        .map_err(websocket_error)?;
    tokio::time::timeout(AUTH_TIMEOUT, async {
        loop {
            let message = socket
                .next()
                .await
                .ok_or_else(socket_closed)?
                .map_err(websocket_error)?;
            if let Message::Binary(bytes) = message {
                let packets = protocol::decode_packets(&bytes).map_err(protocol_error)?;
                if let Some(index) = packets
                    .iter()
                    .position(|packet| packet.operation == OP_AUTH_REPLY)
                {
                    validate_auth_reply(&packets[index].body)?;
                    return Ok(packets
                        .into_iter()
                        .enumerate()
                        .filter_map(|(packet_index, packet)| {
                            (packet_index != index).then_some(packet)
                        })
                        .collect());
                }
            }
        }
    })
    .await
    .map_err(|_| {
        LiveRuntimeError::new(
            "open_live_auth_timeout",
            "哔哩哔哩开放平台 WebSocket 鉴权超时",
            true,
        )
    })?
}

async fn run_session(
    api: OpenLiveApi,
    game_id: String,
    socket: Socket,
    pending_packets: Vec<protocol::Packet>,
    sender: mpsc::Sender<LiveEvent>,
    heartbeat_interval: Duration,
) {
    let (mut writer, mut reader) = socket.split();
    if !forward_packets(&pending_packets, &sender).await {
        cleanup(&api, &game_id, &mut writer).await;
        return;
    }
    let mut heartbeat =
        tokio::time::interval_at(Instant::now() + heartbeat_interval, heartbeat_interval);
    loop {
        tokio::select! {
            biased;
            _ = sender.closed() => break,
            _ = heartbeat.tick() => {
                if writer.send(Message::Binary(protocol::encode_packet(0, OP_HEARTBEAT, 1, &[]))).await.is_err()
                    || api.heartbeat(&game_id).await.is_err()
                {
                    break;
                }
            }
            message = reader.next() => {
                let Some(message) = message else { break };
                match message {
                    Ok(Message::Binary(bytes)) => {
                        let packets = match protocol::decode_packets(&bytes) {
                            Ok(packets) => packets,
                            Err(_) => break,
                        };
                        if contains_interaction_end(&packets) || !forward_packets(&packets, &sender).await {
                            break;
                        }
                    }
                    Ok(Message::Close(_)) | Err(_) => break,
                    Ok(_) => {}
                }
            }
        }
    }
    cleanup(&api, &game_id, &mut writer).await;
}

async fn forward_packets(packets: &[protocol::Packet], sender: &mpsc::Sender<LiveEvent>) -> bool {
    for packet in packets {
        if packet.operation != OP_SEND_MESSAGE {
            continue;
        }
        match event_mapper::map_command(&packet.body) {
            Ok(Some(event)) => {
                if sender.send(event).await.is_err() {
                    return false;
                }
            }
            Ok(None) => {}
            Err(error) => log::warn!("open live event ignored: code={}", error.code),
        }
    }
    true
}

fn contains_interaction_end(packets: &[protocol::Packet]) -> bool {
    packets.iter().any(|packet| {
        packet.operation == OP_SEND_MESSAGE
            && serde_json::from_slice::<Value>(&packet.body)
                .ok()
                .and_then(|value| value.get("cmd").and_then(Value::as_str).map(str::to_owned))
                .as_deref()
                == Some("LIVE_OPEN_PLATFORM_INTERACTION_END")
    })
}

fn validate_auth_reply(body: &[u8]) -> Result<(), LiveRuntimeError> {
    let value: Value = serde_json::from_slice(body).map_err(|_| auth_failed())?;
    let code = value
        .get("code")
        .and_then(Value::as_i64)
        .or_else(|| value.pointer("/data/code").and_then(Value::as_i64))
        .ok_or_else(auth_failed)?;
    if code == 0 {
        Ok(())
    } else {
        Err(auth_failed())
    }
}

async fn cleanup(api: &OpenLiveApi, game_id: &str, writer: &mut SplitSink<Socket, Message>) {
    let _ = writer.close().await;
    if let Err(error) = api.end(game_id).await {
        log::warn!("open live session cleanup failed: code={}", error.code);
    }
}

fn auth_failed() -> LiveRuntimeError {
    LiveRuntimeError::new(
        "SYS_OPEN_LIVE_AUTH_FAILED",
        "哔哩哔哩开放平台 WebSocket 鉴权失败",
        false,
    )
}

fn socket_closed() -> LiveRuntimeError {
    LiveRuntimeError::new(
        "open_live_websocket_closed",
        "哔哩哔哩开放平台 WebSocket 已关闭",
        true,
    )
}

fn websocket_error(error: tokio_tungstenite::tungstenite::Error) -> LiveRuntimeError {
    LiveRuntimeError::new(
        "SYS_OPEN_LIVE_WS_UNAVAILABLE",
        format!("哔哩哔哩开放平台 WebSocket 连接失败：{error}"),
        true,
    )
}

fn websocket_timeout() -> LiveRuntimeError {
    LiveRuntimeError::new(
        "SYS_OPEN_LIVE_WS_UNAVAILABLE",
        "哔哩哔哩开放平台 WebSocket 连接超时",
        true,
    )
}

fn protocol_error(_: protocol::ProtocolError) -> LiveRuntimeError {
    LiveRuntimeError::new(
        "invalid_open_live_packet",
        "哔哩哔哩开放平台 WebSocket 数据包无效",
        false,
    )
}
