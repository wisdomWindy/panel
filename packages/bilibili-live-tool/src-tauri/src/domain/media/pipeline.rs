//! FFmpeg launch-line construction for live recording.
//!
//! This is deliberately a pure function: the argument list is the most
//! failure-prone part of recording (headers, container flags, output format),
//! and it must be assertable without spawning a process.

use std::path::Path;

use super::{MediaDomainError, RecordingFormat, RecordingQuality};

/// Bilibili's CDN rejects requests without a browser-like identity and referer.
pub const RECORDING_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
pub const RECORDING_REFERER: &str = "https://live.bilibili.com/";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RecordingPlanInput<'a> {
    pub stream_url: &'a str,
    pub output: &'a Path,
    pub format: RecordingFormat,
    pub quality: RecordingQuality,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HighlightClipInput<'a> {
    /// Recording file currently being written.
    pub source: &'a Path,
    /// Where the extracted clip should be written.
    pub output: &'a Path,
    /// Marker position inside the recording, in seconds.
    pub position_seconds: u64,
    pub before_seconds: u32,
    pub after_seconds: u32,
}

/// Window that PRD 9.12 asks for: the last `before` seconds plus `after` seconds
/// of tail so the reaction right after the marker is kept too.
pub fn highlight_clip_window(input: &HighlightClipInput<'_>) -> (u64, u64) {
    let before = u64::from(input.before_seconds);
    // `position_seconds` already excludes the tail, so add it back before
    // walking backwards over the look-behind window.
    let start = input
        .position_seconds
        .saturating_add(u64::from(input.after_seconds))
        .saturating_sub(before);
    (start, before + u64::from(input.after_seconds))
}

/// Builds the `ffmpeg` argument vector that remuxes a highlight clip out of an
/// in-progress recording. Stream copy only: no re-encode, so it stays cheap.
pub fn highlight_clip_arguments(
    input: &HighlightClipInput<'_>,
) -> Result<Vec<String>, MediaDomainError> {
    if input.source.as_os_str().is_empty() {
        return Err(MediaDomainError::InvalidField("highlightSource"));
    }
    if input.output.as_os_str().is_empty() || input.output.parent().is_none() {
        return Err(MediaDomainError::InvalidField("highlightOutput"));
    }
    let source = input
        .source
        .to_str()
        .ok_or(MediaDomainError::InvalidField("highlightSource"))?;
    let output = input
        .output
        .to_str()
        .ok_or(MediaDomainError::InvalidField("highlightOutput"))?;
    let (start, duration) = highlight_clip_window(input);

    Ok(vec![
        "-hide_banner".to_owned(),
        "-loglevel".to_owned(),
        "error".to_owned(),
        "-nostdin".to_owned(),
        "-ss".to_owned(),
        start.to_string(),
        "-i".to_owned(),
        source.to_owned(),
        "-t".to_owned(),
        duration.to_string(),
        "-c".to_owned(),
        "copy".to_owned(),
        "-y".to_owned(),
        output.to_owned(),
    ])
}

/// Builds the exact `ffmpeg` argument vector used to record a live stream.
///
/// The stream is copied without re-encoding (`-c copy`), which keeps CPU usage
/// near zero and preserves the platform's original quality.
pub fn recording_arguments(input: &RecordingPlanInput<'_>) -> Result<Vec<String>, MediaDomainError> {
    let url = input.stream_url.trim();
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err(MediaDomainError::InvalidField("recordingStreamUrl"));
    }
    if input.output.as_os_str().is_empty() || input.output.parent().is_none() {
        return Err(MediaDomainError::InvalidField("recordingOutput"));
    }
    let output = input
        .output
        .to_str()
        .ok_or(MediaDomainError::InvalidField("recordingOutput"))?;
    if output.chars().any(char::is_control) {
        return Err(MediaDomainError::InvalidField("recordingOutput"));
    }

    let mut arguments = vec![
        "-hide_banner".to_owned(),
        "-loglevel".to_owned(),
        "warning".to_owned(),
        "-nostdin".to_owned(),
        "-user_agent".to_owned(),
        RECORDING_USER_AGENT.to_owned(),
        "-headers".to_owned(),
        format!("Referer: {RECORDING_REFERER}\r\n"),
        "-i".to_owned(),
        url.to_owned(),
        "-c".to_owned(),
        "copy".to_owned(),
        "-f".to_owned(),
        container_name(input.format).to_owned(),
    ];
    if input.format == RecordingFormat::Mp4 {
        // A live copy is not seekable; fragment the MP4 so the file stays
        // playable even if the process is killed mid-recording.
        arguments.extend(["-movflags".to_owned(), "frag_keyframe+empty_moov".to_owned()]);
    }
    arguments.extend(["-y".to_owned(), output.to_owned()]);
    Ok(arguments)
}

pub const fn container_name(format: RecordingFormat) -> &'static str {
    match format {
        RecordingFormat::Flv => "flv",
        RecordingFormat::Mp4 => "mp4",
    }
}

pub const fn file_extension(format: RecordingFormat) -> &'static str {
    match format {
        RecordingFormat::Flv => "flv",
        RecordingFormat::Mp4 => "mp4",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        file_extension, highlight_clip_arguments, highlight_clip_window, recording_arguments,
        HighlightClipInput, RecordingPlanInput,
    };
    use crate::domain::media::{MediaDomainError, RecordingFormat, RecordingQuality};
    use std::path::Path;

    fn input<'a>(url: &'a str, output: &'a Path) -> RecordingPlanInput<'a> {
        RecordingPlanInput {
            stream_url: url,
            output,
            format: RecordingFormat::Flv,
            quality: RecordingQuality::Original,
        }
    }

    #[test]
    fn copies_the_stream_without_re_encoding() {
        let output = Path::new("/tmp/room-1.flv");
        let arguments = recording_arguments(&input("https://cdn.example/live.flv", output)).unwrap();

        assert!(arguments.windows(2).any(|pair| pair == ["-c", "copy"]));
        assert!(arguments.windows(2).any(|pair| pair == ["-f", "flv"]));
        assert_eq!(arguments.last().unwrap(), "/tmp/room-1.flv");
        assert!(arguments.iter().any(|argument| argument == "-y"));
    }

    #[test]
    fn sends_the_referer_and_user_agent_the_cdn_requires() {
        let output = Path::new("/tmp/room-1.flv");
        let arguments = recording_arguments(&input("https://cdn.example/live.flv", output)).unwrap();
        let joined = arguments.join(" ");

        assert!(joined.contains("Referer: https://live.bilibili.com/"));
        assert!(joined.contains("Mozilla/5.0"));
        assert!(!joined.contains("U+FFFD"));
    }

    #[test]
    fn fragments_mp4_output_so_a_kill_still_leaves_a_playable_file() {
        let output = Path::new("/tmp/room-1.mp4");
        let arguments = recording_arguments(&RecordingPlanInput {
            stream_url: "https://cdn.example/live.flv",
            output,
            format: RecordingFormat::Mp4,
            quality: RecordingQuality::High,
        })
        .unwrap();

        assert!(arguments.windows(2).any(|pair| pair == ["-f", "mp4"]));
        assert!(arguments
            .windows(2)
            .any(|pair| pair == ["-movflags", "frag_keyframe+empty_moov"]));
    }

    #[test]
    fn rejects_a_non_http_stream_url() {
        let output = Path::new("/tmp/room-1.flv");
        assert_eq!(
            recording_arguments(&input("ftp://cdn.example/live.flv", output)),
            Err(MediaDomainError::InvalidField("recordingStreamUrl"))
        );
        assert_eq!(
            recording_arguments(&input("", output)),
            Err(MediaDomainError::InvalidField("recordingStreamUrl"))
        );
    }

    #[test]
    fn rejects_an_output_without_a_parent_directory() {
        assert!(recording_arguments(&input("https://cdn.example/live.flv", Path::new(""))).is_err());
    }

    #[test]
    fn maps_formats_to_their_container_and_extension() {
        assert_eq!(file_extension(RecordingFormat::Flv), "flv");
        assert_eq!(file_extension(RecordingFormat::Mp4), "mp4");
        assert_eq!(super::container_name(RecordingFormat::Mp4), "mp4");
    }

    fn clip<'a>(source: &'a Path, output: &'a Path, position: u64) -> HighlightClipInput<'a> {
        HighlightClipInput {
            source,
            output,
            position_seconds: position,
            before_seconds: 60,
            after_seconds: 5,
        }
    }

    #[test]
    fn highlight_window_keeps_the_last_sixty_seconds_plus_the_tail() {
        // Marker at 300s, stored position excludes the 5s tail -> 295s.
        let (start, duration) = highlight_clip_window(&clip(
            Path::new("C:/rec/1.flv"),
            Path::new("C:/rec/1-h.flv"),
            295,
        ));
        assert_eq!(start, 240);
        assert_eq!(duration, 65);
    }

    #[test]
    fn highlight_window_clamps_at_the_start_of_the_recording() {
        let (start, duration) = highlight_clip_window(&clip(
            Path::new("C:/rec/1.flv"),
            Path::new("C:/rec/1-h.flv"),
            10,
        ));
        assert_eq!(start, 0);
        // The duration still covers before+after; ffmpeg stops at end-of-file.
        assert_eq!(duration, 65);
    }

    #[test]
    fn builds_a_stream_copy_argument_list_for_the_clip() {
        let arguments = highlight_clip_arguments(&clip(
            Path::new("C:/rec/1.flv"),
            Path::new("C:/rec/1-h.flv"),
            295,
        ))
        .unwrap();

        assert_eq!(arguments[arguments.len() - 10..], [
            "-ss".to_owned(),
            "240".to_owned(),
            "-i".to_owned(),
            "C:/rec/1.flv".to_owned(),
            "-t".to_owned(),
            "65".to_owned(),
            "-c".to_owned(),
            "copy".to_owned(),
            "-y".to_owned(),
            "C:/rec/1-h.flv".to_owned(),
        ]);
        // No re-encode: the clip must stay as cheap as the recording itself.
        assert!(!arguments.iter().any(|argument| argument == "libx264"));
    }

    #[test]
    fn rejects_a_clip_without_source_or_output() {
        assert!(highlight_clip_arguments(&clip(
            Path::new(""),
            Path::new("C:/rec/1-h.flv"),
            10
        ))
        .is_err());
        assert!(highlight_clip_arguments(&clip(
            Path::new("C:/rec/1.flv"),
            Path::new(""),
            10
        ))
        .is_err());
    }
}
