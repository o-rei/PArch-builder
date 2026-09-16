//! Builder for Arch Linux on Pi-style Platforms.
use clap::{Parser, Subcommand};

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
    },

    /// Build one or more targets from a manifest
    Build {
        /// Manifest target, such as rpi5
        sbc_model: String,

        /// Show the plan without modifying anything
        #[arg(long)]
        dryrun: bool,
    },

    /// List available build targets
    List,
}


fn main() -> anyhow::Result<()> {

    let cli = Cli::parse();

    match cli.command {

        // *** FETCH ***
        Commands::Fetch { sbc_model, overwrite } => {

            println!("Fetching the foundation for SBC model {}...", sbc_model);

            pbuilder::read_manifest_and_fetch_foundation(
                &sbc_model, overwrite
            )?;

            Ok(())
        }

        // *** BUILD CARTÕES ***
        Commands::Build { sbc_model, dryrun } => {
            println!("target: {sbc_model}");
            println!("dry run: {dryrun}");

            Ok(())
        }

        // *** LIST ***
        Commands::List => {
            println!("rpi5");
            println!("rpi2w");
            println!("opi3");

            Ok(())
        }
    }
}

