use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    io::BufRead,
    path::{Path, PathBuf},
};
use tokio::fs;
use youtube_dl::YoutubeDl;

use crate::video::Video;

#[derive(Debug)]
pub struct Channel {
    pub url: String,
}

impl Channel {
    pub fn get_channel_dir(&self, output_dir: &Path) -> PathBuf {
        let name = self.url.split('/').last().expect("Expect valid link");
        output_dir.join(name)
    }

    #[tracing::instrument(err)]
    pub async fn get_videos(&self, amount: Option<usize>) -> Result<Vec<Video>> {
        let mut command = &mut YoutubeDl::new(format!("{}/videos", self.url));

        if let Some(amount) = amount {
            command = command
                .extra_arg("--playlist-end")
                .extra_arg(amount.to_string());
        }

        let output = command.run_async().await?;

        let videos = output
            .into_playlist()
            .and_then(|playlist| playlist.entries)
            .context("Expect channel to contain at least 1 videos")?;

        let videos = videos
            .par_iter()
            .map(|video| {
                let url = video
                    .webpage_url
                    .clone()
                    .context("Expect video to contain url")?;
                let title = video
                    .title
                    .clone()
                    .context("Expect video to contain title")?;

                let wrapped_video: Result<Video> = Ok(Video { url, title });

                wrapped_video
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(videos)
    }
}

#[tracing::instrument(err)]
pub async fn get_channels(input: &Path) -> Result<Vec<Channel>> {
    let channels_raw = fs::read_to_string(input).await?;

    let channels = channels_raw
        .split("\n")
        .map(|url| Channel {
            url: url.to_string(),
        })
        .collect();

    Ok(channels)
}
