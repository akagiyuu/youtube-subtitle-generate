use std::path::Path;

mod post_processing;
mod pre_processing;
mod whisper;

use anyhow::Result;

pub fn generate_and_save(video_path: &Path) -> Result<()> {
    let audio_path = pre_processing::extract_audio(video_path)?;

    whisper::run(&audio_path)?;

    post_processing::save_segments(video_path)
}
