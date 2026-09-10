//! Builder for Arch Linux on Pi-style Platforms.
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(
    name = "pb",
    version,
    about = "Build reproducible system images from platform manifests"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Fetch a source from the manifest
    Fetch {
        /// Platform name indicating yml in manifests
        platform: String,
        overwrite: bool,
    },

    /// Build one or more targets from a manifest
    Build {
        /// Manifest target, such as rpi5
        platform: String,

        /// Show the plan without modifying anything
        #[arg(long)]
        dryrun: bool,
    },

    /// List available build targets
    List,
}


fn main() {
    let cli = Cli::parse();

    match cli.command {

        // FETCH
        Commands::Fetch { platform, overwrite } => {

            println!("Going to fetch for the {} platform", platform);

            let mut overwrite_msg = "We wouldn't overwrite if it existed.";
            if overwrite {
                overwrite_msg = "We would overwrite if it existed.";
            }

            println!("{}", overwrite_msg);
        }

        // BUILD
        Commands::Build { platform, dryrun } => {
            println!("target: {platform}");
            println!("dry run: {dryrun}");
        }

        // LIST
        Commands::List => {
            println!("rpi5");
            println!("rpi2w");
            println!("opi3");
        }
    }
}
