use anyhow::Result;
use std::path::{Path,PathBuf};
use std::io::Write;
use std::fs::{exists,File};
use reqwest::Url;


pub struct Downloader {
    urls: Vec<Url>,
    savedir: PathBuf
}



impl Downloader {

    pub fn new(urls: Vec<Url>, savedir: impl AsRef<Path>) -> Self {
       Self {
           urls: urls,
           savedir: savedir.as_ref().to_path_buf(),
       }
    }

    pub async fn download_resources(&self) -> Result<()> {

        for url in self.urls.iter() {

            eprintln!("Fetching {url:?}...");

            let res = reqwest::get(url.as_str()).await?;

            // Now split URL to take last part (filename) and append to savepath
            let filename =
                url.path_segments()
                   .and_then(|segments| segments.last())
                   .expect("Reading filename from download URL failed.");

            let savepath = self.savedir.join(filename);
            let mut dest = File::create(savepath)?;

            let content = res.bytes().await?;
            dest.write_all(&content)?;

            eprintln!(" {url:?}...");
        }

        Ok(())
    }
}

