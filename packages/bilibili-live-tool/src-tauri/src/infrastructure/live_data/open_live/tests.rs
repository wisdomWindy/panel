use std::{collections::HashMap, io::Write};

use flate2::{write::ZlibEncoder, Compression};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::{mpsc, oneshot},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::{
    application::live_data::RoomTransport,
    domain::live_data::{CaptainAction, CaptainLevel, IdentityTag, LiveEvent},
};

use super::{
    api::OpenLiveApi, config::OpenLiveConfig, event_mapper, protocol, signing,
    transport::BilibiliOpenLiveTransport,
};

#[derive(Debug)]
struct RecordedHttpRequest {
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

async fn spawn_http_server(
    responses: Vec<String>,
) -> (String, mpsc::Receiver<RecordedHttpRequest>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (sender, receiver) = mpsc::channel(responses.len());
    tokio::spawn(async move {
        for response_body in responses {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let header_end = loop {
                let mut chunk = [0_u8; 1024];
                let read = stream.read(&mut chunk).await.unwrap();
                assert!(read > 0);
                request.extend_from_slice(&chunk[..read]);
                if let Some(index) = request.windows(4).position(|window| window == b"\r\n\r\n") {
                    break index + 4;
                }
            };
            let head = String::from_utf8(request[..header_end].to_vec()).unwrap();
            let mut lines = head.split("\r\n");
            let path = lines
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .to_owned();
            let headers: HashMap<_, _> = lines
                .filter_map(|line| line.split_once(':'))
                .map(|(name, value)| (name.to_ascii_lowercase(), value.trim().to_owned()))
                .collect();
            let content_length = headers
                .get("content-length")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            while request.len() - header_end < content_length {
                let mut chunk = [0_u8; 1024];
                let read = stream.read(&mut chunk).await.unwrap();
                request.extend_from_slice(&chunk[..read]);
            }
            sender
                .send(RecordedHttpRequest {
                    path,
                    headers,
                    body: String::from_utf8(
                        request[header_end..header_end + content_length].to_vec(),
                    )
                    .unwrap(),
                })
                .await
                .unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(), response_body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        }
    });
    (format!("http://{address}"), receiver)
}

#[test]
fn validates_configuration_without_exposing_the_secret() {
    let config = OpenLiveConfig::from_values("1716751199246", "test-access-key", "test-secret")
        .expect("valid configuration");

    assert_eq!(config.app_id, 1_716_751_199_246);
    assert_eq!(config.access_key_id, "test-access-key");
    assert_eq!(config.access_key_secret(), "test-secret");
    assert!(!format!("{config:?}").contains("test-access-key"));
    assert!(!format!("{config:?}").contains("test-secret"));

    let error = OpenLiveConfig::from_values("0", "", "").unwrap_err();
    assert_eq!(error.code, "missing_open_live_credentials");
    assert!(!error.message.contains("test-secret"));
}

#[test]
fn reports_missing_credentials_when_the_build_did_not_inject_them() {
    // Credentials come from `OPEN_LIVE_*` at build time so they never live in
    // the repository; without them the app must fail closed with a code the UI
    // can explain instead of shipping a hardcoded secret.
    assert_eq!(
        OpenLiveConfig::embedded().unwrap_err().code,
        "missing_open_live_credentials"
    );
}

#[test]
fn signs_the_official_document_header_vector() {
    let headers = signing::SignedHeaders {
        content_md5: "fa6837e35b2f591865b288dfd859ce9d".into(),
        timestamp: "1624594467".into(),
        nonce: "ad184c09-095f-91c3-0849-230dd3744045".into(),
        access_key_id: "xxxx".into(),
        authorization: String::new(),
    };
    let canonical = signing::canonical_signature(&headers);
    assert_eq!(
        canonical,
        "x-bili-accesskeyid:xxxx\n\
x-bili-content-md5:fa6837e35b2f591865b288dfd859ce9d\n\
x-bili-signature-method:HMAC-SHA256\n\
x-bili-signature-nonce:ad184c09-095f-91c3-0849-230dd3744045\n\
x-bili-signature-version:1.0\n\
x-bili-timestamp:1624594467"
    );

    let signed = signing::sign_request(
        b"ignored for this fixed vector",
        "xxxx",
        "JzOzZfSHeYYnAMZ",
        "ad184c09-095f-91c3-0849-230dd3744045",
        1_624_594_467,
    );
    let fixed = signing::SignedHeaders {
        content_md5: headers.content_md5,
        ..signed
    };
    assert_eq!(
        signing::sign_request_canonical(&fixed, "JzOzZfSHeYYnAMZ"),
        "a81c50234b6bbf15bc56e387ee4f19c6f871af2f70b837dc56db16517d4a341f"
    );
}

#[test]
fn hashes_the_exact_request_body_bytes() {
    let signed = signing::sign_request(b"{}", "key", "secret", "nonce", 123);
    assert_eq!(signed.content_md5, "99914b932bd37a50b983c5e7c90ae93b");
}

#[test]
fn encodes_and_decodes_big_endian_proto_packets() {
    let first = protocol::encode_packet(0, 7, 1, br#"{"roomid":1}"#);
    assert_eq!(&first[0..4], &(28_u32.to_be_bytes()));
    assert_eq!(&first[4..6], &(16_u16.to_be_bytes()));
    assert_eq!(&first[6..8], &(0_u16.to_be_bytes()));
    assert_eq!(&first[8..12], &(7_u32.to_be_bytes()));
    assert_eq!(&first[12..16], &(1_u32.to_be_bytes()));

    let second = protocol::encode_packet(0, 2, 2, &[]);
    let packets = protocol::decode_packets(&[first, second].concat()).unwrap();
    assert_eq!(packets.len(), 2);
    assert_eq!(packets[0].operation, 7);
    assert_eq!(packets[0].body, br#"{"roomid":1}"#);
    assert_eq!(packets[1].operation, 2);
    assert!(packets[1].body.is_empty());
}

#[test]
fn recursively_decodes_multiple_zlib_compressed_packets() {
    let dm = protocol::encode_packet(0, 5, 1, br#"{"cmd":"one"}"#);
    let gift = protocol::encode_packet(0, 5, 2, br#"{"cmd":"two"}"#);
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&[dm, gift].concat()).unwrap();
    let compressed = encoder.finish().unwrap();
    let outer = protocol::encode_packet(2, 5, 3, &compressed);

    let packets = protocol::decode_packets(&outer).unwrap();
    assert_eq!(packets.len(), 2);
    assert_eq!(packets[0].body, br#"{"cmd":"one"}"#);
    assert_eq!(packets[1].body, br#"{"cmd":"two"}"#);
}

#[test]
fn rejects_truncated_or_invalid_proto_lengths() {
    assert!(protocol::decode_packets(&[0; 15]).is_err());
    let mut invalid = protocol::encode_packet(0, 5, 1, b"{}");
    invalid[4..6].copy_from_slice(&15_u16.to_be_bytes());
    assert!(protocol::decode_packets(&invalid).is_err());
    let mut extended_header = protocol::encode_packet(0, 5, 1, b"{}");
    extended_header[4..6].copy_from_slice(&17_u16.to_be_bytes());
    assert!(protocol::decode_packets(&extended_header).is_err());
}

#[tokio::test]
async fn falls_back_after_a_websocket_handshake_timeout() {
    let stalled_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let stalled_address = stalled_listener.local_addr().unwrap();
    let stalled_task = tokio::spawn(async move {
        let _connection = stalled_listener.accept().await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    });

    let healthy_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let healthy_address = healthy_listener.local_addr().unwrap();
    let healthy_task = tokio::spawn(async move {
        let (stream, _) = healthy_listener.accept().await.unwrap();
        let mut socket = accept_async(stream).await.unwrap();
        let auth = socket.next().await.unwrap().unwrap().into_data();
        assert_eq!(protocol::decode_packets(&auth).unwrap()[0].operation, 7);
        socket
            .send(Message::Binary(protocol::encode_packet(
                0,
                8,
                1,
                br#"{"code":0}"#,
            )))
            .await
            .unwrap();
        while socket.next().await.is_some() {}
    });
    let session = super::api::StartSession {
        game_id: "game-timeout".into(),
        auth_body: r#"{"roomid":100}"#.into(),
        wss_links: vec![
            format!("ws://{stalled_address}"),
            format!("ws://{healthy_address}"),
        ],
        room_id: "100".into(),
        room_name: "anchor".into(),
    };

    let (mut socket, _) =
        super::transport::connect_any(&session, std::time::Duration::from_millis(20))
            .await
            .unwrap();
    socket.close(None).await.unwrap();
    healthy_task.await.unwrap();
    stalled_task.abort();
}

#[test]
fn maps_official_danmu_and_gift_commands_into_domain_events() {
    let danmu = event_mapper::map_command(r#"{
        "cmd":"LIVE_OPEN_PLATFORM_DM",
        "data":{"room_id":100,"uid":0,"open_id":"39b8fedb-60a5-4e29-ac75-b16955f7e632","uname":"alice","msg":"hello","msg_id":"dm-1","fans_medal_level":8,"fans_medal_name":"粉丝牌","fans_medal_wearing_status":true,"guard_level":3,"timestamp":1710000000,"is_admin":1}
    }"#.as_bytes()).unwrap().unwrap();
    match danmu {
        LiveEvent::Danmu { base, content } => {
            assert_eq!(base.event_id, "dm-1");
            assert_eq!(base.room_id.as_str(), "100");
            assert_eq!(base.occurred_at, 1_710_000_000_000);
            assert_eq!(
                base.audience.user_uid.as_str(),
                "39b8fedb-60a5-4e29-ac75-b16955f7e632"
            );
            assert_eq!(base.audience.username, "alice");
            assert_eq!(base.audience.medal.as_ref().unwrap().level, Some(8));
            assert_eq!(
                base.audience.tags,
                [IdentityTag::Admin, IdentityTag::Captain]
            );
            assert_eq!(content, "hello");
        }
        _ => panic!("expected danmu"),
    }

    let gift = event_mapper::map_command(r#"{
        "cmd":"LIVE_OPEN_PLATFORM_SEND_GIFT",
        "data":{"room_id":100,"uid":0,"open_id":"gift-user","uname":"bob","gift_id":31036,"gift_name":"小花花","gift_num":3,"price":2000,"gift_icon":"https://i0.hdslb.com/bfs/live/gift.png","fans_medal_level":0,"fans_medal_name":"","fans_medal_wearing_status":false,"guard_level":0,"timestamp":1710000001,"msg_id":"gift-1"}
    }"#.as_bytes()).unwrap().unwrap();
    match gift {
        LiveEvent::Gift {
            base,
            gift_id,
            gift_name,
            gift_icon_url,
            quantity,
            value_minor,
        } => {
            assert_eq!(base.event_id, "gift-1");
            assert_eq!(gift_id, "31036");
            assert_eq!(gift_name, "小花花");
            assert_eq!(
                gift_icon_url.as_deref(),
                Some("https://i0.hdslb.com/bfs/live/gift.png")
            );
            assert_eq!(quantity, 3);
            assert_eq!(value_minor, 600);
        }
        _ => panic!("expected gift"),
    }
}

#[test]
fn maps_guard_and_enter_commands_and_ignores_unknown_commands() {
    let guard = event_mapper::map_command(r#"{
        "cmd":"LIVE_OPEN_PLATFORM_GUARD",
        "data":{"user_info":{"uid":0,"open_id":"guard-user","uname":"carol"},"guard_level":1,"guard_num":1,"guard_unit":"月","price":198000,"fans_medal_level":24,"fans_medal_name":"舰队","fans_medal_wearing_status":true,"timestamp":1710000002,"room_id":100,"msg_id":"guard-1"}
    }"#.as_bytes()).unwrap().unwrap();
    match guard {
        LiveEvent::Captain {
            base,
            action,
            level,
            value_minor,
        } => {
            assert_eq!(base.event_id, "guard-1");
            assert_eq!(action, CaptainAction::Purchase);
            assert_eq!(level, CaptainLevel::Governor);
            assert_eq!(value_minor, 19_800);
        }
        _ => panic!("expected captain"),
    }

    let enter = event_mapper::map_command(
        br#"{
        "cmd":"LIVE_OPEN_PLATFORM_LIVE_ROOM_ENTER",
        "data":{"room_id":100,"uid":0,"open_id":"enter-user","uname":"dave","timestamp":1710000003}
    }"#,
    )
    .unwrap()
    .unwrap();
    match enter {
        LiveEvent::Enter { base } => {
            assert_eq!(base.room_id.as_str(), "100");
            assert_eq!(base.audience.username, "dave");
            assert_eq!(base.event_id, "enter:100:enter-user:1710000003");
        }
        _ => panic!("expected enter"),
    }

    assert!(
        event_mapper::map_command(br#"{"cmd":"SOMETHING_NEW","data":{}}"#)
            .unwrap()
            .is_none()
    );
}

/// PRD 9.1.1: DM_MIRROR repeats DM without sender fields, so it must not be
/// collected even though it carries danmaku text.
#[test]
fn ignores_the_danmaku_mirror_command() {
    let mirror = br#"{
        "cmd":"LIVE_OPEN_PLATFORM_DM_MIRROR",
        "data":{"room_id":100,"uid":0,"open_id":"","uname":"","timestamp":1710000004,"msg":"mirrored","msg_id":"mirror-1"}
    }"#;

    assert!(event_mapper::map_command(mirror).unwrap().is_none());
}

#[tokio::test]
async fn sends_signed_start_heartbeat_and_end_requests() {
    let responses = vec![
        r#"{"code":0,"message":"ok","request_id":"start-request","data":{"game_info":{"game_id":"game-1"},"websocket_info":{"auth_body":"{\"roomid\":100}","wss_link":["ws://127.0.0.1:9000"]},"anchor_info":{"room_id":100,"uname":"anchor","uface":"","uid":1}}}"#.to_owned(),
        r#"{"code":0,"message":"ok","request_id":"heartbeat-request","data":{}}"#.to_owned(),
        r#"{"code":0,"message":"ok","request_id":"end-request","data":{}}"#.to_owned(),
    ];
    let (base_url, mut requests) = spawn_http_server(responses).await;
    let api = OpenLiveApi::new(
        OpenLiveConfig::from_values("1716751199246", "test-key", "test-secret").unwrap(),
        base_url,
    );

    let session = api.start("identity-code").await.unwrap();
    assert_eq!(session.game_id, "game-1");
    assert_eq!(session.auth_body, r#"{"roomid":100}"#);
    assert_eq!(session.wss_links, ["ws://127.0.0.1:9000"]);
    assert_eq!(session.room_id, "100");
    assert_eq!(session.room_name, "anchor");
    api.heartbeat("game-1").await.unwrap();
    api.end("game-1").await.unwrap();

    let start = requests.recv().await.unwrap();
    assert_eq!(start.path, "/v2/app/start");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&start.body).unwrap(),
        serde_json::json!({"code":"identity-code","app_id":1_716_751_199_246_u64})
    );
    assert_eq!(start.headers.get("accept").unwrap(), "application/json");
    assert_eq!(
        start.headers.get("content-type").unwrap(),
        "application/json"
    );
    assert_eq!(
        start.headers.get("x-bili-signature-method").unwrap(),
        "HMAC-SHA256"
    );
    assert_eq!(
        start.headers.get("x-bili-signature-version").unwrap(),
        "1.0"
    );
    assert_eq!(start.headers.get("authorization").unwrap().len(), 64);
    assert_eq!(
        start.headers.get("x-bili-content-md5").unwrap(),
        &signing::sign_request(
            start.body.as_bytes(),
            "test-key",
            "test-secret",
            "unused",
            0
        )
        .content_md5
    );

    let heartbeat = requests.recv().await.unwrap();
    assert_eq!(heartbeat.path, "/v2/app/heartbeat");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&heartbeat.body).unwrap(),
        serde_json::json!({"game_id":"game-1"})
    );
    let end = requests.recv().await.unwrap();
    assert_eq!(end.path, "/v2/app/end");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&end.body).unwrap(),
        serde_json::json!({"app_id":1_716_751_199_246_u64,"game_id":"game-1"})
    );
}

#[tokio::test]
async fn authenticates_websocket_forwards_events_and_ends_the_session() {
    let ws_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let ws_address = ws_listener.local_addr().unwrap();
    let (heartbeat_seen_sender, heartbeat_seen) = oneshot::channel();
    let ws_task = tokio::spawn(async move {
        let (stream, _) = ws_listener.accept().await.unwrap();
        let mut socket = accept_async(stream).await.unwrap();
        let auth = socket.next().await.unwrap().unwrap().into_data();
        let packets = protocol::decode_packets(&auth).unwrap();
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].operation, 7);
        assert_eq!(packets[0].body, br#"{"roomid":100}"#);
        socket
            .send(Message::Binary(protocol::encode_packet(
                0,
                8,
                1,
                br#"{"code":0}"#,
            )))
            .await
            .unwrap();

        let danmu = r#"{"cmd":"LIVE_OPEN_PLATFORM_DM","data":{"room_id":100,"open_id":"viewer-open-id","uname":"viewer","msg":"from websocket","msg_id":"dm-ws","timestamp":1710000004}}"#;
        socket
            .send(Message::Binary(protocol::encode_packet(
                0,
                5,
                2,
                danmu.as_bytes(),
            )))
            .await
            .unwrap();
        while let Some(message) = socket.next().await {
            let bytes = message.unwrap().into_data();
            if protocol::decode_packets(&bytes)
                .unwrap()
                .iter()
                .any(|packet| packet.operation == 2)
            {
                let _ = heartbeat_seen_sender.send(());
                break;
            }
        }
        while socket.next().await.is_some() {}
    });

    let responses = vec![
        format!(
            r#"{{"code":0,"message":"ok","request_id":"start","data":{{"game_info":{{"game_id":"game-ws"}},"websocket_info":{{"auth_body":"{{\"roomid\":100}}","wss_link":["ws://{ws_address}"]}},"anchor_info":{{"room_id":100,"uname":"anchor","uface":"","uid":1}}}}}}"#
        ),
        r#"{"code":0,"message":"ok","request_id":"heartbeat","data":{}}"#.to_owned(),
        r#"{"code":0,"message":"ok","request_id":"end","data":{}}"#.to_owned(),
    ];
    let (base_url, mut requests) = spawn_http_server(responses).await;
    let api = OpenLiveApi::new(
        OpenLiveConfig::from_values("1716751199246", "test-key", "test-secret").unwrap(),
        base_url,
    );
    let transport = BilibiliOpenLiveTransport::with_api_and_heartbeat(
        api,
        std::time::Duration::from_millis(10),
    );

    let mut connection = transport.connect("identity-code").await.unwrap();
    assert_eq!(connection.room_id.as_str(), "100");
    assert_eq!(connection.room_name.as_deref(), Some("anchor"));
    let event = tokio::time::timeout(std::time::Duration::from_secs(1), connection.events.recv())
        .await
        .unwrap()
        .unwrap();
    match event {
        LiveEvent::Danmu { base, content } => {
            assert_eq!(base.event_id, "dm-ws");
            assert_eq!(content, "from websocket");
        }
        _ => panic!("expected danmu"),
    }
    assert_eq!(requests.recv().await.unwrap().path, "/v2/app/start");
    tokio::time::timeout(std::time::Duration::from_secs(1), heartbeat_seen)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(requests.recv().await.unwrap().path, "/v2/app/heartbeat");
    drop(connection.events);
    let end = tokio::time::timeout(std::time::Duration::from_secs(1), requests.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(end.path, "/v2/app/end");
    ws_task.abort();
}
