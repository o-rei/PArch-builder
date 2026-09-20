use std::path::{Path, PathBuf};

use anyhow::Context;
use directories::BaseDirs;

use crate::block_device::{BlockDevice,mock_sd};

const DEFAULT_BOOT_SIZE_MIB: u64 = 200;


fn image_cache_dir() -> anyhow::Result<PathBuf> {

    let dirs = BaseDirs::new()
        .context("could not determine user directories")?;

    let image_dir = dirs
        .cache_dir()
        .join("parch-builder")
        .join("images");

    std::fs::create_dir_all(&image_dir)?;

    Ok(image_dir)
}


/// Create a .img of selected Arch Linux ARM foundation
pub fn create_foundation_img(
        sbc_model: &str,
        foundation_path: impl AsRef<Path>,
        overwrite: bool,
        boot_size_mib: u64,
    ) -> anyhow::Result<PathBuf> {

    let foundation_path = foundation_path.as_ref();

    if !foundation_path.is_file() {
        anyhow::bail!(
            "foundation archive not found: {}",
            foundation_path.display()
        )
    }

    let image_path = image_cache_dir()?
        .join(format!("foundation-{sbc_model}.img"));

    if image_path.exists() && !overwrite {
        eprintln!("Using cached image: {}", image_path.display());
        return Ok(image_path);
    }

    // Create a mock block storage device to copy the OS filesystem
    let mut device = mock_sd(
        &image_path,
        DEFAULT_BOOT_SIZE_MIB,
    )?;

    device.create_partitions(boot_size_mib)?;

    todo!();
    // return Ok(PathBuf::new());
}
