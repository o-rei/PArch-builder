use anyhow::Result;
use pbuilder::downloader::Downloader;
use reqwest::Url;
use std::fs;
use tempfile::tempdir;

const SAMPLE_URL: &str =
    "https://www.rust-lang.org/static/images/rust-logo-blk.svg";

#[tokio::test]
async fn downloads_sample_file() -> Result<(), anyhow::Error> {
    // let savedir = std::evn::temp_dir().join("pbuilder-downloader-test");

    // let url = Url::parse(SAMPLE_URL)?;
    // let downloader = Downloader::new(vec![url], &savedir);

    assert!(false);

    Ok(());
}

// #[tokio::test]
// async fn preserves_existing_file_when_overwrite_is_false() -> {};
