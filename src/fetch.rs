//! Fetches and caches foundation filesystem archives.
use std::path::PathBuf;
use std::io::Write;
use std::fs::File;

use anyhow::{Context, Result};
use directories::BaseDirs;


fn foundation_cache_dir() -> Result<PathBuf> {
    let dirs = BaseDirs::new()
        .context("Could not determine user directories")?;

    Ok(
        dirs.cache_dir()
            .join("parched-builder")
            .join("foundations")
    )
}

/// Fetch the resource at the given URL and save. The user must explicitly
/// specify if they would like to overwrite it locally if it exists already.
pub fn fetch(
        url: reqwest::Url,
        overwrite: bool
    ) -> anyhow::Result<(), anyhow::Error> {

    eprintln!("Fetching {url:?}...");

    // Now split URL to take last part (filename) and append to savepath
    let filename =
        url.path_segments()
           .and_then(|mut segments| segments.next_back())
           .expect("Reading filename from download URL failed. Check URL.");

    // Construct system save path from save dir and the file name from the URL
    let savepath = foundation_cache_dir()?.join(filename);

    // If the save path exists already and overwrite is disabled, do nothing
    if savepath.exists() && !overwrite {
        eprintln!("File exists and overwrite set to false, exiting.");
        return Ok(());
    }

    // Get the payload to write to file
    let payload = &reqwest::blocking::get(url.as_str())?.bytes()?;

    // Create file and write payload
    File::create(savepath)?.write_all(payload)?;

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetches_rust_logo() -> Result<()> {
        let url = reqwest::Url::parse(
            "https://www.rust-lang.org/static/images/rust-logo-blk.svg"
        )?;

        let path = foundation_cache_dir()?
            .join("rust-logo-blk.svg");

        let existed_before = path.exists();

        fetch(url, false)?;

        assert!(path.is_file());
        assert!(path.metadata()?.len() > 0);

        // Don't remove a file that existed before this test.
        if !existed_before {
            std::fs::remove_file(path)?;
        }

        Ok(())
    }
}
