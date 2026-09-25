//! Builder for Arch Linux on Pi-style Platforms.
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use glob::glob;

use pbuilder::{manifest,image};


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

    /// List available build targets
    List,

    /// Fetch a source from the manifest
    Fetch {
        /// sbc_model name indicating yml in manifests
        sbc_model: String,

        /// Whether to overwrite existing foundations
        #[arg(long, short='o')]
        overwrite: bool,
    },

    /// Build an img of the foundation for the given SBC model
    Build {
        /// Short name of the SBC model matching manifest yml, eg, rpi2w
        sbc_model: String,
        ///
        /// Size of the virtual SD card for making the .img in MiB
        #[arg(long, default_value_t=8_000)]
        mock_sd_size_mib: u64,

        /// Size of the boot partition in MiB
        #[arg(long, default_value_t=512)]
        boot_size_mib: u64,

        /// Whether to overwrite any existing foundation img
        #[arg(long, short='o')]
        overwrite: bool,
    },

    /// Compress a raw .img image with xz
    Compress {
        /// Path to the raw .img file
        image_path: PathBuf,

        /// Number of worker threads for xz comp; 0 sets number of threads automatically
        #[arg(short = 'T', long, default_value_t = 0)]
        threads: u32,

        /// xz compression level from 0 (least compression) to 9 (most)
        #[arg(short, long, default_value_t = 9, value_parser = 0..=9)]
        level: u8,

        /// Compression memory limit, set to 75% of available RAM by default
        #[arg(short = 'M', long, default_value = "75%")]
        memory_limit: String,

        /// Add xz verbosity; -v gives detailed diagnostics.
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,

        /// Flag to overwrite existing compressed file
        #[arg(long, short = 'o')]
        overwrite: bool

    },

    /// Install a parch platform to a device
    Install {
        /// Short name for SBC, eg, rpi2w for the Raspberry Pi Zero 2W
        sbc_model: String,

        //// Device where PArch is to be installed
        // device_path: PathBuf,

        //// Whether to install to a local loop device before writing to the SD
        #[arg(long)]
        dryrun: bool,

        //// Whether to persist a local loop device initialized in dry run
        // #[arg(long)]
        // dryrun_persist: bool
    },
}


fn main() -> anyhow::Result<()> {

    let cli = Cli::parse();

    match cli.command {
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

        // *** FETCH ***
        Commands::Fetch {
            sbc_model,
            overwrite,
        } => {

            println!("Fetching the foundation for SBC model {}...", sbc_model);

            let foundation_path = pbuilder::read_manifest_and_fetch_foundation(
                &sbc_model, overwrite
            )?;

            // Notify user what was done
            println!(
                "Foundation archive acquired for SBC model {}.", sbc_model
            );
            println!(
                "Foundation archive has been synced to {}", foundation_path.display()
            );

            Ok(())
        }

        // *** BUILD ***
        Commands::Build {
            sbc_model,
            mock_sd_size_mib,
            boot_size_mib,
            overwrite,
        } => {

            let image_path = image::image_cache_dir()?
                .join(format!("{sbc_model}.img"));

            if image_path.exists() && !overwrite {
                eprintln!("Using cached image: {}", image_path.display());
                return Ok(());
            }

            let image_path = pbuilder::image::create_foundation_img(
                &sbc_model,
                mock_sd_size_mib,
                boot_size_mib
            )?;

            println!("Arch Linux ARM image available at {}",
                image_path.display());

            Ok(())
        },

        Commands::Compress {
            image_path,
            threads,
            level,
            memory_limit,
            verbose,
            overwrite
        } => {

            let compressed_path = PathBuf::from(
                format!("{}.xz", image_path.display())
            );

            if compressed_path.is_file() && !overwrite {

                println!(
                    "\n\nCompressed image created at {}",
                    compressed_path.display()
                );

                return Ok(())
            }

            let compressed_path =
                pbuilder::image::compress(
                    &image_path,
                    threads,
                    level,
                    &memory_limit,
                    verbose
                )?;

            println!(
                "\n\nCompressed image created at {}",
                compressed_path.display()
            );

            Ok(())
        },

        // *** INSTALL TO CARTÕES ***
        // Commands::Install { sbc_model, device_path, dryrun, dryrun_persist } => {
        Commands::Install { sbc_model, dryrun } => {
            println!("target: {sbc_model}");
            println!("dry run: {dryrun}");

            Ok(())
        }

    }
}

