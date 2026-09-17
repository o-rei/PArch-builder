use std::path::{Path,PathBuf};

pub fn create_image(
        sbc_model: &str,
        target_drive: impl AsRef<Path>
    ) -> anyhow::Result<PathBuf> { // , Box<dyn std::error::Error>> {

    // XXX Nonsense line below to get it to compile nicely but do nada
    let target_drive = target_drive.as_ref().join(sbc_model);
    Ok(target_drive)
}


pub fn makevirtsd() -> anyhow::Result<PathBuf> {

    // This one needs to be constructed bc right now there's no
    // inputs with type information: target_drive converted by
    // `as_ref` is a Path, then a Path can easily be converted to
    // a PathBuf?
    Ok(PathBuf::from(""))
}


// *** TEST SUITE ***
//
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;


    #[cfg(target_os = "linux")]
    #[test]
    fn mock_filesystem_image_created() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let image_path = dir.path().join("virtsd.img");

        let device_path = makevirtsd(&image_path, 512)?;
        // let virtsd: PathBuf = PathBuf::from("");

        // nonsense expected not to pass but compiles nicely
        // assert!(device_path == virtsd);

        Ok(())
    }
}
