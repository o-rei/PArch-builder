use reqwest::blocking::Client;

pub struct Downloader {
    client: Client,
}

impl Downloader {
    pub fn new() -> Result<Self, DownloadError> {
        Ok(Self {
            client: Client::builder().build().map_err(|_| DownloadError::FailedToCreateClient)?,
        })
    }
}

#[derive(Debug,Clone)]
pub enum DownloadError {
    FailedToCreateClient,
}
