use std::{
    fs::File,
    path::{Path, PathBuf},
};

pub fn create_image(
        sbc_model: &str,
        target_drive: impl AsRef<Path>
    ) -> anyhow::Result<PathBuf> {

    // XXX Nonsense line below to get it to compile nicely but do nada
    let target_drive = target_drive.as_ref().join(sbc_model);
    Ok(target_drive)
}

#[cfg(target_os = "linux")]
pub fn makevirtsd(
        image_path: impl AsRef<Path>,
        size_mib: u64
    ) -> anyhow::Result<PathBuf> {

    // Extract the reference from generic and create a new image file
    let image_path = image_path.as_ref();
    let image = File::create(image_path)?;
    // Set the logical size of the image file
    image.set_len(size_mib * 1024 * 1024)?;

    Ok(image_path.to_path_buf())
}


// *** TEST SUITE ***
//
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn mock_filesystem_image_created() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let image_path = dir.path().join("virtsd.img");

        let created_path = makevirtsd(&image_path, 512)?;

        assert_eq!(created_path, image_path);
        assert!(created_path.exists());
        assert_eq!(
            std::fs::metadata(&created_path)?.len(),
            512 * 1024 * 1024
        );

        Ok(())
    }
}
