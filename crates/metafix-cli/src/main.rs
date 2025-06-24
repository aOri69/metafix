use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

#[derive(Parser, Debug)]
#[command(name = "metafix", version, about = "Batch-fix photo/video metadata")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan {
        // #[arg(short, long)]
        path: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            commands::scan(path)?;
        }
    }

    Ok(())
}
