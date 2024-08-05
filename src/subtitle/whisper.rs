use std::{path::Path, process::Command};

use anyhow::Result;

#[tracing::instrument(err)]
pub fn run(audio_path: &Path) -> Result<()> {
    let mut name = format!(
        "{}",
        audio_path
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_string_lossy()
    );
    let output = Command::new("whisper")
        .current_dir(audio_path.parent().unwrap())
        .arg(name)
        .args(["--word_timestamps", "True"])
        .args(["-f", "json"])
        .args(["--language", "vi"])
        .args(["--model", "large-v3"])
        .output()?;

    let output = String::from_utf8(output.stdout)?;
    tracing::info!(output);

    Ok(())
}
