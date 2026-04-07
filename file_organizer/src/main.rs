use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, Context};

mod organizer;
mod utils;

use organizer::FileOrganizer;

#[derive(Parser)]
#[command(name = "file_organizer")]
#[command(about = "A production-ready file organizer that sorts files by type")]
#[command(version = "0.1.0")]
struct Args {
    /// The source directory to organize
    #[arg(short, long, value_name = "SOURCE_DIR")]
    source: PathBuf,

    /// The target directory where organized files will be placed (optional, defaults to source/organized)
    #[arg(short, long, value_name = "TARGET_DIR")]
    target: Option<PathBuf>,

    /// Dry run mode - show what would be moved without actually moving files
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate source directory
    if !args.source.exists() {
        eprintln!("Error: Source directory does not exist: {:?}", args.source);
        std::process::exit(1);
    }

    if !args.source.is_dir() {
        eprintln!("Error: Source path is not a directory: {:?}", args.source);
        std::process::exit(1);
    }

    // Determine target directory
    let target_dir = args.target.unwrap_or_else(|| args.source.join("organized"));

    println!("File Organizer");
    println!("Source: {:?}", args.source);
    println!("Target: {:?}", target_dir);
    println!("Dry run: {}", args.dry_run);
    println!();

    if args.dry_run {
        println!("Dry run mode - no files will be moved.");
        let organizer = FileOrganizer::new(args.source, target_dir).with_dry_run(true);
        organizer.organize()
            .with_context(|| "Failed to organize files")?;
    } else {
        let organizer = FileOrganizer::new(args.source, target_dir);
        organizer.organize()
            .with_context(|| "Failed to organize files")?;
    }

    Ok(())
}
