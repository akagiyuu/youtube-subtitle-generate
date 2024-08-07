use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs;
use youtube_dl::YoutubeDl;

#[derive(Debug)]
pub struct Video {
    pub title: String,
    pub url: String,
}

impl Video {
    fn get_video_path(&self, channel_dir: &Path) -> PathBuf {
        channel_dir.join(&self.title).with_extension("mp4")
    }

    #[tracing::instrument(err)]
    async fn _download(&self, channel_dir: &Path) -> Result<()> {
        YoutubeDl::new(&self.url)
            .format("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best")
            .output_template(&self.title)
            .download_to_async(channel_dir)
            .await?;

        Ok(())
    }

    #[tracing::instrument(err)]
    pub async fn download(&self, channel_dir: &Path) -> Result<PathBuf> {
        let path = self.get_video_path(channel_dir);

        let is_path_exist = fs::try_exists(&path).await?;
        if !is_path_exist {
            self._download(channel_dir).await?;
        }

        Ok(path)
    }
}
