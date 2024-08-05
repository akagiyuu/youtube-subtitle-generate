use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use ffmpeg_sidecar::{command::FfmpegCommand, event::{FfmpegEvent, LogLevel}};

pub fn extract_audio(video_path: &Path) -> Result<PathBuf> {
    let audio_path = video_path.with_extension("wav");

    let mut output = FfmpegCommand::new()
        .input(video_path.to_str().context("Invalid video path")?)
        .no_video()
        .duration("00:00:15")
        .args(["-ar", "16000"])
        .args(["-y", audio_path.to_str().context("Invalid audio path")?])
        .spawn()
        .unwrap();

    output.iter().unwrap().for_each(|e| {
        if let FfmpegEvent::Log(LogLevel::Error, e) = e {
            tracing::error!(e)
        }
    });

    Ok(audio_path)
}
