use std::{path::Path, process::Command};

use anyhow::Result;

#[tracing::instrument(err)]
pub fn run(audio_path: &Path) -> Result<()> {
    let output = Command::new("whisper")
        .current_dir(audio_path.parent().unwrap())
        .args(audio_path)
        .args(["-f", "json"])
        .args(["--language", "vi"])
        .args(["--model", "large-v3"])
        .output()?;

    let output = String::from_utf8(output.stdout)?;
    tracing::info!(output);

    Ok(())
}
