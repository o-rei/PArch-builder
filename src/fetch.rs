use std::path::{Path};
use std::io::Write;
use std::fs::File;


pub async fn fetch(
        url: reqwest::Url,
        savedir: impl AsRef<Path>,
        overwrite: bool
    ) -> anyhow::Result<(), anyhow::Error> {

    eprintln!("Fetching {url:?}...");

    // Now split URL to take last part (filename) and append to savepath
    let filename =
        url.path_segments()
           .and_then(|segments| segments.last())
           .expect("Reading filename from download URL failed. Check URL.");

    // Construct system save path from save dir and the file name from the URL
    let savepath = savedir.as_ref().join(filename);

    // If the save path exists already and overwrite is disabled, do nothing
    if savepath.exists() && !overwrite {
        eprintln!("File exists and overwrite set to false, exiting.");
        return Ok(());
    }

    // Get the payload to write to file
    let payload = &reqwest::get(url.as_str()).await?.bytes().await?;

    // Create file and write payload
    File::create(savepath)?.write_all(payload)?;

    Ok(())
}

