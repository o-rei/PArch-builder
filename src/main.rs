//! Builder for Arch Linux on Pi-style Platforms.
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use glob::glob;

mod manifest;

#[derive(Parser)]
#[command(
    name = "pb",
    version,
    about = "Build reproducible system images from sbc_model manifests"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {

    /// Fetch a source from the manifest
    Fetch {
        /// sbc_model name indicating yml in manifests
        sbc_model: String,

        /// Whether to overwrite existing foundations
        #[arg(long)]
        overwrite: bool,

        /// Whether to create a .img from the fetched .tar.gz
        #[arg(long)]
        create_img: bool,

        /// Size of the /boot/ dir; remaining SD space goes to /root/
        #[arg(short='b', long, default_value_t = 200)]
        boot_size_mib: u64
    },

    /// Install a parch platform to a device
    Install {
        /// Short name for SBC, eg, rpi2w for the Raspberry Pi Zero 2W
        sbc_model: String,

        /// Device where PArch is to be installed
        device_path: PathBuf,

        /// Whether to install to a local loop device before writing to the SD
        #[arg(long)]
        dryrun: bool,

        /// Whether to persist a local loop device initialized in dry run
        #[arg(long)]
        dryrun_persist: bool
    },

    /// List available build targets
    List,
}


fn main() -> anyhow::Result<()> {

    let cli = Cli::parse();

    match cli.command {

        // *** FETCH ***
        Commands::Fetch {
            sbc_model,
            overwrite,
            create_img,
            boot_size_mib
        } => {

            println!("Fetching the foundation for SBC model {}...", sbc_model);

            let foundation_path = pbuilder::read_manifest_and_fetch_foundation(
                &sbc_model, overwrite
            )?;

            // Notify user what was done
            println!(
                "Foundation acquired for SBC model {}.", sbc_model
            );
            println!(
                "Foundation has been synced to {}", foundation_path.display()
            );

            println!("Image creation requires sudo prrivileges...");

            if create_img {
                let image_path = pbuilder::image::create_foundation_img(
                    &sbc_model,
                    &foundation_path,
                    overwrite,
                    boot_size_mib
                )?;

                println!("Arch Linux ARM image available at {}",
                    image_path.display());
            }


            Ok(())
        }

        // *** INSTALL TO CARTÕES ***
        Commands::Install { sbc_model, device_path, dryrun, dryrun_persist } => {
            println!("target: {sbc_model}");
            println!("dry run: {dryrun}");

            Ok(())
        }

        // *** LIST ***
        Commands::List => {

            let manifest_dir = manifest::manifest_dir()?;

            let manifest_path = manifest_dir.as_path();

            // Buiild glob string in three steps to respect borrowing
            let mut manifest_glob_str = String::new();
            manifest_glob_str.push_str(
                manifest_path
                    .to_str()
                    .unwrap_or_else(
                        || ".config/parch-builder/manifests"
                    )
            );
            manifest_glob_str.push_str("/*.yml");

            println!("\nAvailable SBC manifests found with glob\n{}:\n\n",
                     manifest_glob_str);

            for entry in glob(&manifest_glob_str)
                .expect("Failed to read manifest.")
            {
                match entry {
                    Ok(path) => println!("{:?}", path.display()),
                    Err(e) => eprintln!("{}", e)
                }
            }

            Ok(())
        }
    }
}

