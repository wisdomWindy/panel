use std::{
    fs::File,
    io::BufReader,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread::JoinHandle,
    time::Duration,
};

use rodio::{source::SineWave, Decoder, OutputStream, Sink, Source};

use crate::application::media::{AudioOutputPort, AudioPlayback, AudioRequest, MediaIoError};

#[derive(Default)]
pub struct RodioAudioOutput;

struct RodioPlayback {
    stop: mpsc::Sender<()>,
    finished: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl AudioPlayback for RodioPlayback {
    fn stop(&self) {
        let _ = self.stop.send(());
    }

    fn is_finished(&self) -> bool {
        self.finished.load(Ordering::Acquire)
    }
}

impl Drop for RodioPlayback {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

#[async_trait::async_trait]
impl AudioOutputPort for RodioAudioOutput {
    async fn play(&self, request: AudioRequest) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
        validate_volume(request.volume)?;
        tokio::task::spawn_blocking(move || start_audio_worker(request))
            .await
            .map_err(|_| MediaIoError::operation("audio worker failed", true))?
    }

    async fn play_alert(&self, volume: f64) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
        validate_volume(volume)?;
        tokio::task::spawn_blocking(move || start_alert_worker(volume))
            .await
            .map_err(|_| MediaIoError::operation("audio worker failed", true))?
    }
}

fn start_audio_worker(request: AudioRequest) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
    start_worker(move |stop, finished, ready| run_audio(request, stop, finished, ready))
}

fn start_alert_worker(volume: f64) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
    start_worker(move |stop, finished, ready| run_alert(volume, stop, finished, ready))
}

fn start_worker(
    run: impl FnOnce(mpsc::Receiver<()>, Arc<AtomicBool>, mpsc::SyncSender<Result<(), MediaIoError>>)
        + Send
        + 'static,
) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
    let (stop_tx, stop_rx) = mpsc::channel();
    let (ready_tx, ready_rx) = mpsc::sync_channel(1);
    let finished = Arc::new(AtomicBool::new(false));
    let worker_finished = finished.clone();
    let worker = std::thread::spawn(move || {
        run(stop_rx, worker_finished, ready_tx);
    });
    match ready_rx.recv() {
        Ok(Ok(())) => Ok(Box::new(RodioPlayback {
            stop: stop_tx,
            finished,
            worker: Some(worker),
        })),
        Ok(Err(error)) => {
            let _ = worker.join();
            Err(error)
        }
        Err(_) => {
            let _ = worker.join();
            Err(MediaIoError::operation("audio worker stopped", true))
        }
    }
}

fn run_audio(
    request: AudioRequest,
    stop: mpsc::Receiver<()>,
    finished: Arc<AtomicBool>,
    ready: mpsc::SyncSender<Result<(), MediaIoError>>,
) {
    let setup = (|| {
        let file = File::open(request.source)
            .map_err(|_| MediaIoError::unavailable("audio source is unavailable"))?;
        let decoder = Decoder::new(BufReader::new(file))
            .map_err(|_| MediaIoError::invalid("audio source is not valid MP3/WAV"))?;
        let (stream, handle) = OutputStream::try_default()
            .map_err(|_| MediaIoError::unavailable("audio output is unavailable"))?;
        let sink = Sink::try_new(&handle)
            .map_err(|_| MediaIoError::unavailable("audio output is unavailable"))?;
        Ok::<_, MediaIoError>((stream, sink, decoder))
    })();
    let (_stream, sink, decoder) = match setup {
        Ok(setup) => setup,
        Err(error) => {
            let _ = ready.send(Err(error));
            return;
        }
    };
    sink.set_volume(request.volume as f32);
    sink.append(decoder);
    if ready.send(Ok(())).is_err() {
        sink.stop();
        return;
    }
    while !sink.empty() {
        if stop.recv_timeout(Duration::from_millis(20)).is_ok() {
            sink.stop();
            break;
        }
    }
    finished.store(true, Ordering::Release);
}

fn run_alert(
    volume: f64,
    stop: mpsc::Receiver<()>,
    finished: Arc<AtomicBool>,
    ready: mpsc::SyncSender<Result<(), MediaIoError>>,
) {
    let setup = (|| {
        let (stream, handle) = OutputStream::try_default()
            .map_err(|_| MediaIoError::unavailable("audio output is unavailable"))?;
        let sink = Sink::try_new(&handle)
            .map_err(|_| MediaIoError::unavailable("audio output is unavailable"))?;
        Ok::<_, MediaIoError>((stream, sink))
    })();
    let (_stream, sink) = match setup {
        Ok(setup) => setup,
        Err(error) => {
            let _ = ready.send(Err(error));
            return;
        }
    };
    sink.set_volume(volume as f32);
    for &(frequency, duration_millis) in alert_tone_sequence() {
        sink.append(
            SineWave::new(frequency)
                .take_duration(Duration::from_millis(duration_millis))
                .amplify(0.16),
        );
    }
    if ready.send(Ok(())).is_err() {
        sink.stop();
        return;
    }
    while !sink.empty() {
        if stop.recv_timeout(Duration::from_millis(20)).is_ok() {
            sink.stop();
            break;
        }
    }
    finished.store(true, Ordering::Release);
}

const ALERT_TONE_SEQUENCE: [(f32, u64); 4] =
    [(523.25, 70), (659.25, 70), (783.99, 90), (1_046.50, 180)];

fn alert_tone_sequence() -> &'static [(f32, u64)] {
    &ALERT_TONE_SEQUENCE
}

fn validate_volume(volume: f64) -> Result<(), MediaIoError> {
    if !volume.is_finite() || !(0.0..=1.0).contains(&volume) {
        return Err(MediaIoError::invalid("audio volume is invalid"));
    }
    Ok(())
}

pub struct UnavailableAudioOutput;

#[async_trait::async_trait]
impl AudioOutputPort for UnavailableAudioOutput {
    async fn play(&self, _: AudioRequest) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
        Err(MediaIoError::unavailable("audio output is unavailable"))
    }

    async fn play_alert(&self, _: f64) -> Result<Box<dyn AudioPlayback>, MediaIoError> {
        Err(MediaIoError::unavailable("audio output is unavailable"))
    }
}

#[cfg(test)]
mod tests {
    use super::alert_tone_sequence;

    #[test]
    fn built_in_gift_alert_is_a_short_ascending_four_note_cue() {
        let notes = alert_tone_sequence();
        assert_eq!(notes.len(), 4);
        assert!(notes.windows(2).all(|pair| pair[0].0 < pair[1].0));
        assert!(notes.iter().map(|(_, millis)| millis).sum::<u64>() <= 600);
    }
}
