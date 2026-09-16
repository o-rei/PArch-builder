//! Defines and reads platform manifests.
//!
//! Manifests describe the inputs needed to build a target platform,
//! beginning with its foundation filesystem.
use std::{
    fs,
    path::PathBuf,
};

use anyhow::{Context, Result};
use directories::BaseDirs;
use serde::Deserialize;
use url::Url;


#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub foundation_url: Url,
}


/// Returns the platform-standard configuration directory for manifests
fn manifest_dir() -> Result<PathBuf> {
    let dirs = BaseDirs::new()
        .context("Could not determine user directories")?;

    Ok(
        dirs.config_dir()
            .join("parched-builder")
            .join("manifests")
    )
}


/// Returns the platform-standard path for a named manifest.
///
/// Manifest files live beneath the operating system's standard user
/// configuration directory. For example, `manifest_path("rpi2w")`
/// typically resolves to:
///
/// ```text
/// Linux:  ~/.config/parched-builder/manifests/rpi2w.yml
/// macOS:  ~/Library/Application Support/parched-builder/manifests/rpi2w.yml
/// Windows: C:\Users\Alice\AppData\Roaming\parched-builder\manifests\rpi2w.yml
/// ```
///
/// The exact base directory may vary according to environment variables
/// and operating-system configuration.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use pbuilder::manifest::manifest_path;
///
/// let path = manifest_path("rpi2w")?;
///
/// assert!(
///     path.ends_with(
///         Path::new("parched-builder")
///             .join("manifests")
///             .join("rpi2w.yml")
///     )
/// );
///
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn manifest_path(name: &str) -> Result<PathBuf> {
    Ok(manifest_dir()?.join(format!("{name}.yml")))
}


pub fn read(platform_code: &str) -> Result<Manifest> {

    // Build the path to the manifest file based on given name
    let path = manifest_path(platform_code)?;

    // Read manifest YAML into memory
    let contents = fs::read_to_string(&path)
        .with_context(
            || format!(
                "Could not read manifest {}", path.display()
            )
        )?;

    // Parse manifest YAML contents
    let yaml = serde_yaml::from_str(&contents)
        .with_context(
            || format!(
                "Could not parse manifest {}",
                path.display()
            )
        );

    println!("{:?}", yaml);
    yaml
}
