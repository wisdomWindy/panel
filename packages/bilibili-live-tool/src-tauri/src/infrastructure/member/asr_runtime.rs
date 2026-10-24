use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::application::member::{AsrState, AsrStatusRecord, MemberEventSink, MemberServiceError};

const SAMPLE_RATE: usize = 16_000;
const SEGMENT_SECONDS: usize = 5;

pub struct AsrSession {
    stop: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl Drop for AsrSession {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
    }
}

impl AsrSession {
    pub async fn start(
        path: PathBuf,
        model_name: String,
        status: Arc<Mutex<Option<AsrStatusRecord>>>,
        events: Arc<dyn MemberEventSink>,
    ) -> Result<Self, MemberServiceError> {
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = stop.clone();
        let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();
        let worker = std::thread::Builder::new()
            .name("whisper-asr".to_owned())
            .spawn(move || {
                let initialized = initialize(&path);
                let (context, stream, samples, sample_rate, failed) = match initialized {
                    Ok(value) => value,
                    Err(error) => {
                        let _ = ready_tx.send(Err(error));
                        return;
                    }
                };
                let mut state = match context.create_state() {
                    Ok(state) => state,
                    Err(_) => {
                        let _ = ready_tx.send(Err(MemberServiceError::unavailable(
                            "asr_engine_initialization_failed",
                        )));
                        return;
                    }
                };
                let _stream = stream;
                *status.lock().unwrap_or_else(|error| error.into_inner()) = Some(AsrStatusRecord {
                    state: AsrState::Listening,
                    model_name: Some(model_name.clone()),
                    language: "zh".to_owned(),
                    progress_percent: 100,
                    error_code: None,
                });
                if ready_tx.send(Ok(())).is_err() {
                    return;
                }
                let result = (|| {
                    while !worker_stop.load(Ordering::Acquire) {
                        if failed.load(Ordering::Acquire) {
                            return Err("asr_audio_capture_failed");
                        }
                        let chunk = {
                            let mut buffer =
                                samples.lock().unwrap_or_else(|error| error.into_inner());
                            let length = sample_rate * SEGMENT_SECONDS;
                            if buffer.len() >= length {
                                Some(buffer.drain(..length).collect::<Vec<_>>())
                            } else {
                                None
                            }
                        };
                        let Some(chunk) = chunk else {
                            std::thread::sleep(Duration::from_millis(50));
                            continue;
                        };
                        let audio = resample(&chunk, sample_rate);
                        // Do not ask Whisper to transcribe silence (which can hallucinate text).
                        if audio.iter().map(|value| value * value).sum::<f32>()
                            / (audio.len() as f32)
                            < 0.00001
                        {
                            continue;
                        }
                        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
                        params.set_language(Some("zh"));
                        params.set_translate(false);
                        params.set_no_context(true);
                        params.set_print_special(false);
                        params.set_print_progress(false);
                        params.set_print_realtime(false);
                        params.set_print_timestamps(false);
                        let threads = std::thread::available_parallelism()
                            .map_or(2, |count| count.get().min(4));
                        params.set_n_threads(threads as i32);
                        let abort = worker_stop.clone();
                        params.set_abort_callback_safe(move || abort.load(Ordering::Acquire));
                        if state.full(params, &audio).is_err() {
                            if worker_stop.load(Ordering::Acquire) {
                                break;
                            }
                            return Err("asr_inference_failed");
                        }
                        let text = state
                            .as_iter()
                            .map(|segment| segment.to_string())
                            .collect::<String>();
                        let text = text.trim();
                        if !text.is_empty() && !worker_stop.load(Ordering::Acquire) {
                            events.publish_asr_transcript(text);
                        }
                    }
                    Ok(())
                })();
                if let Err(code) = result {
                    let next = AsrStatusRecord {
                        state: AsrState::Error,
                        model_name: Some(model_name),
                        language: "zh".to_owned(),
                        progress_percent: 100,
                        error_code: Some(code.to_owned()),
                    };
                    *status.lock().unwrap_or_else(|error| error.into_inner()) = Some(next.clone());
                    events.publish_asr_status(&next);
                }
            })
            .map_err(|_| MemberServiceError::unavailable("asr_engine_initialization_failed"))?;
        // Keep this guard alive across the await so cancellation also stops the worker.
        let session = Self {
            stop,
            worker: Some(worker),
        };
        ready_rx
            .await
            .map_err(|_| MemberServiceError::unavailable("asr_engine_initialization_failed"))??;
        Ok(session)
    }

    pub async fn stop(mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            let _ = tokio::task::spawn_blocking(move || worker.join()).await;
        }
    }
}

type Capture = (
    WhisperContext,
    cpal::Stream,
    Arc<Mutex<VecDeque<f32>>>,
    usize,
    Arc<AtomicBool>,
);

fn initialize(path: &std::path::Path) -> Result<Capture, MemberServiceError> {
    let mut parameters = WhisperContextParameters::default();
    parameters.use_gpu(false);
    let context = WhisperContext::new_with_params(path.to_string_lossy().as_ref(), parameters)
        .map_err(|_| MemberServiceError::unavailable("asr_engine_initialization_failed"))?;
    let device = cpal::default_host()
        .default_input_device()
        .ok_or_else(|| MemberServiceError::unavailable("asr_microphone_unavailable"))?;
    let config = device
        .default_input_config()
        .map_err(|_| MemberServiceError::unavailable("asr_microphone_unavailable"))?;
    let samples = Arc::new(Mutex::new(VecDeque::new()));
    let failed = Arc::new(AtomicBool::new(false));
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => capture::<f32>(
            &device,
            &config.clone().into(),
            samples.clone(),
            failed.clone(),
        ),
        cpal::SampleFormat::I16 => capture::<i16>(
            &device,
            &config.clone().into(),
            samples.clone(),
            failed.clone(),
        ),
        cpal::SampleFormat::U16 => capture::<u16>(
            &device,
            &config.clone().into(),
            samples.clone(),
            failed.clone(),
        ),
        _ => {
            return Err(MemberServiceError::unavailable(
                "asr_audio_format_unsupported",
            ))
        }
    }
    .map_err(|_| MemberServiceError::unavailable("asr_microphone_unavailable"))?;
    stream
        .play()
        .map_err(|_| MemberServiceError::unavailable("asr_microphone_unavailable"))?;
    Ok((
        context,
        stream,
        samples,
        config.sample_rate().0 as usize,
        failed,
    ))
}

fn capture<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    samples: Arc<Mutex<VecDeque<f32>>>,
    failed: Arc<AtomicBool>,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: cpal::SizedSample,
    f32: cpal::FromSample<T>,
{
    let channels = config.channels as usize;
    let capacity = config.sample_rate.0 as usize * SEGMENT_SECONDS * 2;
    device.build_input_stream(
        config,
        move |data: &[T], _| {
            if let Ok(mut buffer) = samples.try_lock() {
                for frame in data.chunks_exact(channels) {
                    let value = frame
                        .iter()
                        .map(|sample| <f32 as cpal::Sample>::from_sample(*sample))
                        .sum::<f32>()
                        / channels as f32;
                    if buffer.len() == capacity {
                        buffer.pop_front();
                    }
                    buffer.push_back(value);
                }
            }
        },
        move |error| {
            log::warn!("ASR microphone capture failed: {error}");
            failed.store(true, Ordering::Release);
        },
        None,
    )
}

fn resample(samples: &[f32], source_rate: usize) -> Vec<f32> {
    let output_length = samples.len() * SAMPLE_RATE / source_rate;
    (0..output_length)
        .map(|index| {
            let position = index as f64 * source_rate as f64 / SAMPLE_RATE as f64;
            let left = position as usize;
            let right = (left + 1).min(samples.len() - 1);
            let fraction = (position - left as f64) as f32;
            samples[left] * (1.0 - fraction) + samples[right] * fraction
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resampling_preserves_duration_and_constant_signal() {
        let output = resample(&vec![0.25; 48_000], 48_000);
        assert_eq!(output.len(), SAMPLE_RATE);
        assert!(output
            .iter()
            .all(|sample| (*sample - 0.25).abs() < f32::EPSILON));
        assert!(resample(&[], 48_000).is_empty());
    }

    #[test]
    #[ignore = "requires ASR_TEST_MODEL and ASR_TEST_WAV pointing to a Whisper model and JFK sample"]
    fn real_model_transcribes_speech() {
        let model = std::env::var("ASR_TEST_MODEL").expect("ASR_TEST_MODEL");
        let wav = std::env::var("ASR_TEST_WAV").expect("ASR_TEST_WAV");
        let mut reader = hound::WavReader::open(wav).unwrap();
        let spec = reader.spec();
        let samples = reader
            .samples::<i16>()
            .map(|sample| sample.unwrap() as f32 / 32768.0)
            .collect::<Vec<_>>();
        let mono = samples
            .chunks_exact(spec.channels as usize)
            .map(|frame| frame.iter().sum::<f32>() / spec.channels as f32)
            .collect::<Vec<_>>();
        let audio = resample(&mono, spec.sample_rate as usize);
        let mut parameters = WhisperContextParameters::default();
        parameters.use_gpu(false);
        let context = WhisperContext::new_with_params(&model, parameters).unwrap();
        let mut state = context.create_state().unwrap();
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_n_threads(4);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        state.full(params, &audio).unwrap();
        let text = state
            .as_iter()
            .map(|segment| segment.to_string())
            .collect::<String>()
            .to_lowercase();
        assert!(text.contains("country"), "unexpected transcription: {text}");
    }

    #[tokio::test]
    #[ignore = "requires an available default microphone and ASR_TEST_MODEL"]
    async fn microphone_session_starts_and_stops() {
        let model = PathBuf::from(std::env::var("ASR_TEST_MODEL").expect("ASR_TEST_MODEL"));
        let status = Arc::new(Mutex::new(None));
        let session = AsrSession::start(
            model,
            "Tiny".to_owned(),
            status.clone(),
            Arc::new(crate::application::member::NoopMemberEventSink),
        )
        .await
        .unwrap();
        assert_eq!(
            status.lock().unwrap().as_ref().unwrap().state,
            AsrState::Listening
        );
        session.stop().await;
    }
}
