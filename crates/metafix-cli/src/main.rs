use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    Preview {
        // #[arg(short, long)]
        path: PathBuf,
    },
    Apply {
        // #[arg(short, long)]
        path: PathBuf,
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            metafix_core::scan(path)?;
        }
        Commands::Preview { path } => {
            metafix_core::preview(path)?;
        }
        Commands::Apply { path } => {
            metafix_core::apply(path)?;
        }
    }

    Ok(())
}
