#![feature(iterator_try_collect)]

pub mod channel;
pub mod subtitle;
pub mod video;

use std::path::{Path, PathBuf};

use anyhow::Result;
use channel::Channel;
use clap::Parser;
use futures::{stream::FuturesUnordered, TryStreamExt};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::fs;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt};

#[derive(Parser, Debug)]
struct Args {
    /// Input file containing links of YouTube channel
    #[arg(short, long)]
    input: PathBuf,

    /// Directory to save processed video segment
    #[arg(id = "output-dir", short, long)]
    output_dir: PathBuf,

    /// Number of videos to process for each channel, process all videos if not specified
    #[arg(short, long)]
    amount: Option<usize>,

    /// Wether to run script in parallel, the default is false
    #[arg(short, long, default_value_t = false)]
    parallel: bool,
}

#[tracing::instrument(err)]
async fn process_channel(
    url: String,
    output_dir: &Path,
    amount: Option<usize>,
    parallel: bool,
) -> Result<()> {
    let channel = Channel::new(url).await?;
    let channel_dir = &channel.get_channel_dir(output_dir);
    fs::create_dir_all(channel_dir).await?;

    let videos = channel.get_videos(amount).await?;

    tracing::info!("Finish fetching video datas");

    let video_paths = videos
        .iter()
        .map(|video| video.download(channel_dir))
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?;

    tracing::info!("Finish downloading videos");

    if parallel {
        video_paths
            .par_iter()
            .try_for_each(|video_path| subtitle::generate_and_save(video_path))?;
    } else {
        video_paths
            .iter()
            .try_for_each(|video_path| subtitle::generate_and_save(video_path))?;
    }

    tracing::info!("Finish generate subtitle");

    Ok(())
 }

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let args = Args::parse();
    let input = args.input.as_path();
    let output_dir = args.output_dir.as_path();
    let channel_urls_raw = std::fs::read_to_string(input)?;
    let channel_urls_raw = channel_urls_raw.trim();

    channel_urls_raw
        .split('\n')
        .map(|url| process_channel(url.to_string(), output_dir, args.amount, args.parallel))
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?;

    Ok(())
}
