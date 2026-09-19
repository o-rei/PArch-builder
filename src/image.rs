use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio}
};

use anyhow::Context;



pub trait BlockDevice {
    fn path(&self) -> &Path;

    #[cfg(target_os = "linux")]
    fn initialize(&mut self, boot_size_mib: u64,) -> anyhow::Result<()> {

        // Create ad-hoc config for the sfdisk
        let sfdisk_config = format!(
            "label: dos\n\nsize={boot_size_mib}MiB, type=c, bootable\ntype=83\n"
        );

        let mut sfdisk = Command::new("sfdisk")
            .arg(self.path())
            .stdin(Stdio::piped())
            .spawn()?;

        let mut stdin = sfdisk.stdin.take().unwrap();
        stdin.write_all(sfdisk_config.as_bytes())?;

        drop(stdin);

        let status = sfdisk.wait()?;

        if !status.success() {
            anyhow::bail!("sfdisk failed")
        }

        Ok(())
    }
}


pub struct LoopDevice {
    path: PathBuf,
    detach_on_drop: bool,
}

impl BlockDevice for LoopDevice {
    fn path(&self) -> &Path {
        &self.path
    }
}


impl LoopDevice {

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn keep(mut self) -> PathBuf {
        self.detach_on_drop = false;
        self.path.clone()
    }

    pub fn detach(&mut self) {

        match detach_loop_device(self) {
            // On success, remove guard for Drop trait
            Ok(()) => {
                self.detach_on_drop = false;
            }

            Err(error) => {
                eprintln!(
                    "Error encountered gracefully detaching from loop device: {}",
                    error
                );
            }
        };
    }
}


fn create_loop_device(
        image_path: impl AsRef<Path>
    ) -> anyhow::Result<LoopDevice> {

    let image_path = image_path.as_ref();

    let output =
        Command::new("losetup")
            .args(["--find", "--show", "--partscan"])
            .arg(image_path)
            .output()
            .context(
                format!("Could not attach {} to a loop device",
                        image_path.display())
            )?;

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


fn detach_loop_device(device: &mut LoopDevice) -> anyhow::Result<()> {
    match
        Command::new("losetup")
              .arg("--detach")
              .arg(device.path.clone())  // loop device, eg /dev/loop0
              .status()
    {

        Ok(status) if status.success() => { Ok(()) }

        Ok(status) => {
            eprintln!("losetup failed to detach {}: {status}",
                      device.path.display());
            Ok(())
        }

        Err(error) => {

            eprintln!(
                "could not run losetup for {}: {error}",
                device.path.display()
            );

            Err(anyhow::Error::new(error))
        }
    }
}


impl Drop for LoopDevice {

    fn drop(&mut self) {

        if self.detach_on_drop {
            self.detach();
        }
    }

}


#[cfg(target_os = "linux")]
pub fn mock_sd(
        image_path: &Path,
        size_mib: u64
    ) -> anyhow::Result<LoopDevice> {

    // Create a mock operating system image .img file at the specified path
    let image = File::create(image_path)?;

    // set the logical length of the data block
    image.set_len(size_mib * 1024 * 1024)?;

    // Create and return the LoopDevice loaded with the mock .img
    Ok(
        create_loop_device(image_path)
            .context(
                format!("Loop device creation failed on {}",
                        image_path.display())
            )?
    )
}


// *** TEST SUITE ***
//
#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::FileTypeExt;

    // Mock SD image on loop device created successfully
    #[cfg(target_os = "linux")]
    #[test]
    fn mock_sd_image_created() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let image_path = dir.path().join("virtsd.img");

        let device = mock_sd(&image_path, 4096)?;

        assert!(
            std::fs::metadata(device.path())?
                .file_type()
                .is_block_device()
        );

        Ok(())
    }

    // Check that boot and root filesystems
    #[cfg(target_os = "linux")]
    #[test]
    fn mock_partitions_created() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let image_path = dir.path().join("mocksd.img");

        // Create mock SD on loop with 500MiB allotted
        let mut device = mock_sd(&image_path, 500)?;

        device.initialize(200)?;

        let output = Command::new("lsblk")
            .args(["--raw", "--noheadings", "--output", "TYPE"])
            .arg(device.path())
            .output()?;

        assert!(output.status.success());

        let partition_count = String::from_utf8(output.stdout)?
            .lines()
            .filter(|device_type| *device_type == "part")
            .count();

        assert_eq!(partition_count, 2);

        Ok(())
    }
}


