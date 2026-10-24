use std::{
    sync::atomic::{AtomicUsize, Ordering},
    sync::{Arc, Mutex as StdMutex},
    time::Duration,
};

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::domain::live_data::{
    normalize_identity, AudienceIdentityInput, LiveEvent, LiveEventBase, SensitiveMatchType,
    SensitiveRule, SensitiveWordCategory,
};

use super::{
    AcceptedLiveEventConsumer, AcceptedLiveEventFanout, ConnectionStatus, LiveDataState,
    LiveEventSink, LiveRepository, LiveRuntimeError, RoomConnection, RoomSnapshot, RoomTransport,
    UnavailableRoomTransport,
};

struct FakeTransport {
    receiver: StdMutex<Option<mpsc::Receiver<LiveEvent>>>,
}

#[derive(Default)]
struct RetryableTransport {
    attempts: AtomicUsize,
}

#[async_trait]
impl RoomTransport for RetryableTransport {
    async fn connect(&self, _: &str) -> Result<RoomConnection, LiveRuntimeError> {
        self.attempts.fetch_add(1, Ordering::SeqCst);
        Err(LiveRuntimeError::new(
            "transport_unavailable",
            "retryable transport failure",
            true,
        ))
    }
}

#[async_trait]
impl RoomTransport for FakeTransport {
    async fn connect(&self, _: &str) -> Result<RoomConnection, LiveRuntimeError> {
        Ok(RoomConnection {
            room_id: crate::domain::live_data::RoomId::parse("100").unwrap(),
            room_name: Some("room".into()),
            online_count: Some(10),
            events: self.receiver.lock().unwrap().take().unwrap(),
        })
    }
}

#[derive(Default)]
struct FakeRepository {
    recorded: StdMutex<Vec<Option<&'static str>>>,
}

#[async_trait]
impl LiveRepository for FakeRepository {
    async fn load_sensitive(&self) -> Result<Vec<SensitiveRule>, LiveRuntimeError> {
        Ok(vec![SensitiveRule::new(
            1,
            "blocked",
            SensitiveWordCategory::Custom,
            SensitiveMatchType::Exact,
            true,
        )
        .unwrap()])
    }

    async fn list_blacklist_entries(
        &self,
    ) -> Result<Vec<crate::domain::live_data::BlacklistEntry>, LiveRuntimeError> {
        Ok(vec![crate::domain::live_data::BlacklistEntry {
            uid: crate::domain::live_data::UserUid::parse("2").unwrap(),
            username: "blocked-user".into(),
            reason: Some("test".into()),
            created_at: 1,
        }])
    }

    async fn today_sensitive_hits(&self) -> Result<u64, LiveRuntimeError> {
        Ok(0)
    }

    async fn today_blacklist_additions(&self) -> Result<u64, LiveRuntimeError> {
        Ok(0)
    }

    async fn record_event(
        &self,
        _: &LiveEvent,
        filtered_reason: Option<&'static str>,
    ) -> Result<(), LiveRuntimeError> {
        self.recorded.lock().unwrap().push(filtered_reason);
        Ok(())
    }
}

#[derive(Default)]
struct RecordingSink {
    statuses: StdMutex<Vec<ConnectionStatus>>,
    events: StdMutex<Vec<LiveEvent>>,
}

impl LiveEventSink for RecordingSink {
    fn publish_room_status(&self, snapshot: &RoomSnapshot) -> Result<(), LiveRuntimeError> {
        self.statuses.lock().unwrap().push(snapshot.status);
        Ok(())
    }

    fn publish_live_event(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError> {
        self.events.lock().unwrap().push(event.clone());
        Ok(())
    }
}

#[derive(Default)]
struct FailingAcceptedConsumer {
    event_ids: StdMutex<Vec<String>>,
}

#[derive(Default)]
struct RecordingAcceptedConsumer {
    event_ids: StdMutex<Vec<String>>,
}

#[async_trait]
impl AcceptedLiveEventConsumer for RecordingAcceptedConsumer {
    async fn consume_accepted(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError> {
        self.event_ids
            .lock()
            .unwrap()
            .push(event.base().event_id.clone());
        Ok(())
    }
}

#[async_trait]
impl AcceptedLiveEventConsumer for FailingAcceptedConsumer {
    async fn consume_accepted(&self, event: &LiveEvent) -> Result<(), LiveRuntimeError> {
        self.event_ids
            .lock()
            .unwrap()
            .push(event.base().event_id.clone());
        Err(LiveRuntimeError::new(
            "media_failed",
            "media consumer failed",
            true,
        ))
    }
}

fn event(id: &str, uid: &str, content: &str) -> LiveEvent {
    LiveEvent::Danmu {
        base: LiveEventBase {
            event_id: id.into(),
            room_id: crate::domain::live_data::RoomId::parse("100").unwrap(),
            occurred_at: 1_000,
            audience: normalize_identity(AudienceIdentityInput {
                user_uid: uid.into(),
                username: format!("user-{uid}"),
                user_level: None,
                medal_name: None,
                medal_level: None,
                is_admin: false,
                captain_level: None,
                is_vip: false,
            })
            .unwrap(),
        },
        content: content.into(),
    }
}

#[tokio::test]
async fn accepted_event_fanout_isolates_a_failed_consumer() {
    let failing = Arc::new(FailingAcceptedConsumer::default());
    let recording = Arc::new(RecordingAcceptedConsumer::default());
    let fanout = AcceptedLiveEventFanout::new(vec![failing.clone(), recording.clone()]);

    fanout
        .consume_accepted(&event("fanout-1", "2", "hello"))
        .await
        .unwrap();

    assert_eq!(failing.event_ids.lock().unwrap().as_slice(), ["fanout-1"]);
    assert_eq!(recording.event_ids.lock().unwrap().as_slice(), ["fanout-1"]);
}

#[tokio::test]
async fn production_unavailable_transport_never_reports_connected() {
    let repository = Arc::new(FakeRepository::default());
    let sink = Arc::new(RecordingSink::default());
    let state = Arc::new(LiveDataState::new(
        Arc::new(UnavailableRoomTransport),
        repository,
        sink.clone(),
    ));
    state.load_safety().await.unwrap();

    state.connect_room("100").await.unwrap();
    tokio::task::yield_now().await;

    assert_eq!(state.room_snapshot().await.status, ConnectionStatus::Error);
    assert!(!sink
        .statuses
        .lock()
        .unwrap()
        .contains(&ConnectionStatus::Connected));
}

#[tokio::test]
async fn pipeline_records_all_inputs_but_only_publishes_and_aggregates_accepted_events() {
    let (sender, receiver) = mpsc::channel(8);
    sender.send(event("1", "2", "hello")).await.unwrap();
    sender.send(event("2", "3", "blocked text")).await.unwrap();
    sender.send(event("3", "4", "allowed")).await.unwrap();
    drop(sender);
    let repository = Arc::new(FakeRepository::default());
    let sink = Arc::new(RecordingSink::default());
    let consumer = Arc::new(FailingAcceptedConsumer::default());
    let state = Arc::new(
        LiveDataState::new(
            Arc::new(FakeTransport {
                receiver: StdMutex::new(Some(receiver)),
            }),
            repository.clone(),
            sink.clone(),
        )
        .with_accepted_consumer(consumer.clone()),
    );
    state.load_safety().await.unwrap();

    state.connect_room("100").await.unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            if repository.recorded.lock().unwrap().len() == 3 {
                return;
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .unwrap();
    state.disconnect_room().await.unwrap();

    assert_eq!(
        repository.recorded.lock().unwrap().as_slice(),
        [Some("blacklist"), Some("sensitive"), None]
    );
    assert_eq!(sink.events.lock().unwrap().len(), 1);
    assert_eq!(consumer.event_ids.lock().unwrap().as_slice(), ["3"]);
    assert_eq!(state.stat_snapshot().await.danmu_count, 1);
    assert_eq!(state.stat_snapshot().await.filtered_count, 2);
    assert_eq!(
        state.room_snapshot().await.status,
        ConnectionStatus::Disconnected
    );
}

#[tokio::test(start_paused = true)]
async fn retries_on_a_fixed_five_second_interval_and_stops_after_ten_attempts() {
    let transport = Arc::new(RetryableTransport::default());
    let state = Arc::new(LiveDataState::new(
        transport.clone(),
        Arc::new(FakeRepository::default()),
        Arc::new(RecordingSink::default()),
    ));
    state.load_safety().await.unwrap();
    state.connect_room("100").await.unwrap();
    tokio::task::yield_now().await;
    assert_eq!(transport.attempts.load(Ordering::SeqCst), 1);

    tokio::time::advance(Duration::from_millis(4_999)).await;
    tokio::task::yield_now().await;
    assert_eq!(transport.attempts.load(Ordering::SeqCst), 1);
    tokio::time::advance(Duration::from_millis(1)).await;
    tokio::task::yield_now().await;
    assert_eq!(transport.attempts.load(Ordering::SeqCst), 2);

    for _ in 0..8 {
        tokio::time::advance(Duration::from_secs(5)).await;
        tokio::task::yield_now().await;
    }
    assert_eq!(transport.attempts.load(Ordering::SeqCst), 10);
    assert_eq!(state.room_snapshot().await.status, ConnectionStatus::Error);
}
