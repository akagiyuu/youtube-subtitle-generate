use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::{Path, PathBuf};
use youtube_dl::YoutubeDl;

use crate::video::Video;

#[derive(Debug)]
pub struct Channel {
    pub url: String,
    pub name: String,
}

impl Channel {
    #[tracing::instrument(err)]
    pub async fn new(url: String) -> Result<Self> {
        let ytdlp_output = YoutubeDl::new(&url).playlist_items(1).run_async().await?;

        let playlist = ytdlp_output
            .into_playlist()
            .context("Expect output to contain playlist")?;

        let video = &playlist
            .entries
            .context("Expect playlist to contain 1 entry")?[0];

        let name = video
            .channel
            .clone()
            .context("Expect channel to have a name")?;

        Ok(Self { url, name })
    }

    pub fn get_channel_dir(&self, output_dir: &Path) -> PathBuf {
        output_dir.join(&self.name)
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
