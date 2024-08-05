use std::path::Path;
use std::{fs, str::FromStr};

use anyhow::{anyhow, Context, Result};
use ffmpeg_sidecar::{
    command::FfmpegCommand,
    event::{FfmpegEvent, LogLevel},
};
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::Deserialize;

#[tracing::instrument(err)]
fn cut_and_save_video(
    start: &str,
    end: &str,
    output_path: &Path,
    original_path: &Path,
) -> Result<()> {
    let original_path = original_path.to_str().context("Invalid segment path")?;
    let output_path = output_path.to_str().context("Invalid segment path")?;

    let mut output = FfmpegCommand::new()
        .input(original_path)
        .seek(start)
        .to(end)
        .args(["-y", output_path])
        .spawn()
        .unwrap();

    output.iter().unwrap().for_each(|e| {
        if let FfmpegEvent::Log(LogLevel::Error, e) = e {
            tracing::error!(e)
        }
    });

    Ok(())
}

fn to_timestamp(sec: f64) -> String {
    let msec = (sec.fract() * 1000.).round() as u64;
    let mut sec = sec.round() as u64;
    let hour = sec / (60 * 60);
    sec -= hour * 60 * 60;
    let min = sec / 60;
    sec -= min * 60;

    format!("{:02}:{:02}:{:02}.{:03}", hour, min, sec, msec)
}

// "start": 1.26,
// "end": 3.7,
// "text": " L\u1ea7n n\u00e0y thay v\u00ec ng\u1ed3i trong xe th\u00ec m\u00ecnh \u0111\u00e3 \u0111\u01b0\u1ee3c ng\u1ed3i trong nh\u00e0",
#[derive(Deserialize)]
struct Segment {
    start: f64,
    end: f64,
    text: String,
}

impl Segment {
    fn save(&self, segment_base_path: &Path, video_path: &Path) -> Result<()> {
        let start = to_timestamp(self.start);
        let end = to_timestamp(self.end);

        let segment_video_path = segment_base_path.with_extension("mp4");
        cut_and_save_video(&start, &end, &segment_video_path, video_path)?;

        let segment_text_path = segment_base_path.with_extension("txt");
        fs::write(segment_text_path, &self.text)?;

        Ok(())
    }
}

#[derive(Deserialize)]
struct Segments {
    segments: Vec<Segment>,
}

#[tracing::instrument(err)]
pub fn save_segments(video_path: &Path) -> Result<()> {
    let subtitle_path = video_path.with_extension("json");

    let subtitled_segments_path = video_path.with_extension("");
    let subtitled_segments_path = subtitled_segments_path.as_path();

    fs::create_dir_all(subtitled_segments_path)?;
    let subtitle = fs::read_to_string(subtitle_path)?;
    let segments: Segments = serde_json::from_str(&subtitle)?;

    segments
        .segments
        .iter()
        .enumerate()
        .par_bridge()
        .try_for_each(|(segment_id, segment)| {
            segment.save(
                &subtitled_segments_path.join(segment_id.to_string()),
                video_path,
            )
        })
        .into()
}
