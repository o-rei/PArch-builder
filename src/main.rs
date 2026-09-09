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
    /// Build one or more targets from a manifest
    Build {
        /// Manifest target, such as rpi5
        target: String,

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
        Commands::Build { target, dryrun } => {
            println!("target: {target}");
            println!("dry run: {dryrun}");
        }

        Commands::List => {
            println!("rpi5");
            println!("rpi2w");
            println!("opi3");
        }
    }
}
