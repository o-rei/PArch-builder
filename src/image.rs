use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};


pub struct LoopDevice {
    path: PathBuf,
    detach_on_drop: bool,
}


impl LoopDevice {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn keep(mut self) -> PathBuf {
        self.detach_on_drop = false;
        self.path.clone()
    }
}


fn create_loop_device(
        image_path: impl AsRef<Path>
    ) -> anyhow::Result<LoopDevice> {

    let output =
        Command::new("losetup")
            .args(["--find", "--show"])
            .arg(image_path.as_ref())
            .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "losetup failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let device_path = String::from_utf8_lossy(&output.stdout);
    let device_path = device_path.trim();

    // To succeed reaching the end of the function, must have device path
    if device_path.is_empty() {

        // Bail out, ie exit gracefully with a message, if there is no path
        anyhow::bail!("losetup completed, but failed to provide a device path")

    // To reach here, `losetup` had to succeed and return a device path
    } else {
        // Return a new loop device handle, specifying whether or not to persist
        Ok(LoopDevice {
            path: PathBuf::from(device_path),
            detach_on_drop: true
        })
    }
}


fn detach_loop_device(device: &mut LoopDevice) {
    match
        Command::new("losetup")
              .arg("--detach")
              .arg(device.path.clone())  // loop device, eg /dev/loop0
              .status()
    {

        Ok(status) if status.success() => {}

        Ok(status) => eprintln!(
            "losetup failed to detach {}: {status}",
            device.path.display()
        ),

        Err(error) => eprintln!(
            "could not run losetup for {}: {error}",
            device.path.display()
        ),
    }
}


impl Drop for LoopDevice {

    fn drop(&mut self) {

        if self.detach_on_drop {
            detach_loop_device(self)
        }
    }
}


#[cfg(target_os = "linux")]
pub fn mock_sd(
        mock_image_path: Option<&Path>,
        size_mib: Option<u64>
    ) -> anyhow::Result<LoopDevice> {

    // Create a mock operating system image .img file at the specified path
    let image_path = mock_image_path.unwrap_or(Path::new("mock.img"));
    let image = File::create(image_path)?;

    // Set the logical size of the .img; Linux only allocates when actually used
    // -- first, deal with optionally missing size parameter
    let size_mib = size_mib.unwrap_or(512);
    // -- second, set the logical length of the data block
    image.set_len(size_mib * 1024 * 1024)?;

    // Create and return the LoopDevice loaded with the mock .img
    Ok(create_loop_device(image_path)?)
}


// *** TEST SUITE ***
//
#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::FileTypeExt;

    #[cfg(target_os = "linux")]
    #[test]
    fn mock_sd_image_created() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let image_path = dir.path().join("virtsd.img");

        let device = mock_sd(Some(&image_path), None)?;

        assert!(
            std::fs::metadata(device.path())?
                .file_type()
                .is_block_device()
            );

        Ok(())
    }
}
