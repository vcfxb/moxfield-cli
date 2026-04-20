//! Refresh the local copy of scryfall's bulk data export.

use futures::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::LevelFilter;
use moxfield_cli::scryfall::client::ScryfallClient;
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use url::Url;

static MULTIPROGRESS: LazyLock<MultiProgress> = LazyLock::new(|| MultiProgress::new());

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let logger = env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .build();
    let bridge = indicatif_log_bridge::LogWrapper::new(MULTIPROGRESS.clone(), logger);
    bridge.try_init()?;

    let scryfall_client = ScryfallClient::new();
    let bulk_data = scryfall_client.bulk_data().await?;
    let all_cards = bulk_data["data"]
        .as_array()
        .expect("data is array")
        .iter()
        .filter_map(|v| v.as_object())
        .find(|v| v["type"] == "all_cards")
        .expect("found all_cards");

    let uri = all_cards["download_uri"].as_str().unwrap().parse::<Url>()?;
    let file_name = uri.path_segments().unwrap().rev().next().unwrap();
    let size = all_cards["size"].as_i64().unwrap();

    static DOWNLOAD_STYLE: LazyLock<ProgressStyle> = LazyLock::new(|| {
        ProgressStyle::with_template("[{decimal_bytes} / {decimal_total_bytes} {elapsed}] {msg} {wide_bar} {decimal_bytes_per_sec}").unwrap()
    });

    let progress = MULTIPROGRESS.add(ProgressBar::new(size as u64));
    progress.set_style(DOWNLOAD_STYLE.clone());
    progress.set_message("pulling bulk data");

    let response = reqwest::get(uri.clone()).await?;
    let mut byte_stream = response.bytes_stream();

    if !PathBuf::from("scryfall_pull").exists() {
        tokio::fs::create_dir("scryfall_pull").await?;
    }

    let file = File::create(format!("scryfall_pull/{file_name}")).await?;
    let mut writer = BufWriter::new(file);

    while let Some(next) = byte_stream.next().await {
        let bytes_written = writer.write(next?.as_ref()).await?;
        progress.inc(bytes_written as u64);
    }

    progress.finish();

    Ok(())
}
