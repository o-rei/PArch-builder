//! Defines and reads platform manifests.
//!
//! Manifests describe the inputs needed to build a target platform,
//! beginning with its foundation filesystem.
use std::{fs, path::Path};

use anyhow::Result;
use serde::Deserialize;

use std::path::PathBuf;
use url::Url;


#[derive(Debug, Deserialize)]
pub struct Foundation {
    pub url: Url,
}


fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("manifests")
}


pub fn path(name: &str) -> PathBuf {
    manifest_dir().join(format!("{name}.yml"))
}


#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub source: Source,
}


#[derive(Debug, Deserialize)]
pub struct Source {
    pub url: reqwest::Url,
}


pub fn read(path: impl AsRef<Path>) -> Result<Manifest> {
    let contents = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&contents)?)
}


