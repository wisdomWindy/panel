use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use super::state::RecordingReconnectWatchdog;
use super::{
    default_recording_config, FfmpegLocator, MediaIoError, PlayUrlResolver, RecorderLaunch,
    RecordingRuntime, ResolvedStream,
};
use crate::domain::media::FfmpegSource;
use crate::infrastructure::media::LocalFfmpegProcessPort;
use crate::domain::media::RecordingState;

/// PRD 9.12 real-drop verification. A local HTTP server serves a real FLV:
/// the first connection is cut after ~30% (the "stream dropped" case), the
/// second serves the whole file. The watchdog must notice the dead recorder,
/// re-resolve through the resolver and restart into a continuation file.
///
/// RECORDING_FFMPEG=<path to ffmpeg> cargo test --lib --features free \
///   drops_the_recording_stream_and_reconnects -- --ignored --nocapture
#[tokio::test]
#[ignore = "requires ffmpeg; set RECORDING_FFMPEG"]
async fn drops_the_recording_stream_and_reconnects_to_a_continuation_file() {
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpListener;

    let ffmpeg = std::env::var("RECORDING_FFMPEG").expect("RECORDING_FFMPEG must point at ffmpeg");
    let directory = tempfile::tempdir().unwrap();
    // Produce a genuine FLV so FFmpeg accepts the stream at all.
    let sample = directory.path().join("sample.flv");
    let build = tokio::process::Command::new(&ffmpeg)
        .args([
            "-hide_banner", "-loglevel", "error", "-f", "lavfi", "-i",
            "testsrc=size=320x240:rate=15:duration=12",
        ])
        .args(["-c:v", "libx264", "-preset", "ultrafast", "-f", "flv", "-y"])
        .arg(&sample)
        .status()
        .await
        .unwrap();
    assert!(build.success(), "sample FLV could not be generated");
    let bytes = std::fs::read(&sample).unwrap();
    let cut = bytes.len() * 3 / 10;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let connections = Arc::new(AtomicUsize::new(0));
    let server_connections = connections.clone();
    let server_body = bytes.clone();
    tokio::spawn(async move {
        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                return;
            };
            let index = server_connections.fetch_add(1, Ordering::SeqCst);
            let body = server_body.clone();
            tokio::spawn(async move {
                if index == 0 {
                    // First attempt: a bounded body that stops early, which is
                    // how a real drop looks to FFmpeg.
                    let header = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: video/x-flv\r\nContent-Length: {}\r\n\r\n",
                        body.len()
                    );
                    let _ = socket.write_all(header.as_bytes()).await;
                    let _ = socket.write_all(&body[..cut]).await;
                    let _ = socket.shutdown().await;
                } else {
                    // Reconnected attempt: no Content-Length and the socket is
                    // kept open, so FFmpeg blocks for more data exactly like a
                    // live stream instead of hitting a clean EOF.
                    let header = "HTTP/1.1 200 OK\r\nContent-Type: video/x-flv\r\nConnection: close\r\n\r\n";
                    let _ = socket.write_all(header.as_bytes()).await;
                    let _ = socket.write_all(&body).await;
                    let _ = socket.flush().await;
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                }
            });
        }
    });

    struct FixedResolver(String);
    #[async_trait::async_trait]
    impl PlayUrlResolver for FixedResolver {
        async fn resolve(
            &self,
            _: &str,
            _: crate::domain::media::RecordingQuality,
        ) -> Result<ResolvedStream, MediaIoError> {
            Ok(ResolvedStream {
                url: self.0.clone(),
                available_qualities: Vec::new(),
            })
        }
    }

    struct FixedFfmpeg(PathBuf);
    impl FfmpegLocator for FixedFfmpeg {
        fn locate(
            &self,
            _: FfmpegSource,
            _: Option<&std::path::Path>,
        ) -> Result<PathBuf, MediaIoError> {
            Ok(self.0.clone())
        }
    }

    let port = Arc::new(LocalFfmpegProcessPort);
    let config = default_recording_config();
    let runtime = RecordingRuntime::new(config.clone(), port);
    let stream_url = format!("http://{address}/live.flv");
    let first_output = directory.path().join("part1.flv");
    let arguments = crate::domain::media::recording_arguments(
        &crate::domain::media::RecordingPlanInput {
            stream_url: &stream_url,
            output: &first_output,
            format: config.format,
            quality: config.quality,
        },
    )
    .unwrap();
    runtime
        .start(
            "1",
            RecorderLaunch {
                executable: PathBuf::from(&ffmpeg),
                arguments,
                working_directory: Some(directory.path().to_path_buf()),
            },
        )
        .await
        .unwrap();

    tokio::spawn(
        RecordingReconnectWatchdog {
            recording: runtime.clone(),
            play_url: Arc::new(FixedResolver(stream_url)),
            ffmpeg: Arc::new(FixedFfmpeg(PathBuf::from(&ffmpeg))),
            granted_executable: None,
            ffmpeg_source: FfmpegSource::Bundled,
            room_id: "1".to_owned(),
            directory: directory.path().to_path_buf(),
            quality: config.quality,
            format: config.format,
        }
        .run(),
    );

    // The first connection dies immediately; the watchdog polls every 2s.
    let mut reconnected = false;
    for _ in 0..20 {
        tokio::time::sleep(std::time::Duration::from_millis(1_000)).await;
        let snapshot = runtime.snapshot().await;
        if connections.load(Ordering::SeqCst) >= 2
            && snapshot.state == RecordingState::Recording
            && !runtime.has_exited().await
        {
            reconnected = true;
            break;
        }
    }

    let parts: Vec<_> = std::fs::read_dir(directory.path())
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|name| name.contains("part"))
        .collect();
    println!(
        "connections={}, state={:?}, parts={parts:?}",
        connections.load(Ordering::SeqCst),
        runtime.snapshot().await.state
    );

    let snapshot = runtime.snapshot().await;
    runtime.stop().await.ok();
    assert!(
        reconnected,
        "recorder was never restarted after the drop (state={:?}, parts={parts:?})",
        snapshot.state
    );
    assert!(
        parts.iter().any(|name| name.contains("part2")),
        "no continuation file was written: {parts:?}"
    );
}
