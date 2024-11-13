use anyhow::Result;
use clap::{Parser, Subcommand};
use manifest_parser_rs::Manifest;
use std::fs;
use std::path::PathBuf;

/// Manifest Parser CLI - A tool for parsing and inspecting manifest files
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a manifest file and display its contents
    Parse {
        /// Path to the manifest file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Get a specific value from a section by key
    GetByKey {
        /// Path to the manifest file
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Section name to look up
        #[arg(value_name = "SECTION")]
        section: String,
        /// Key name within the section
        #[arg(value_name = "KEY")]
        key: String,
    },
    /// Get all key-value pairs from a section
    GetBySection {
        /// Path to the manifest file
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Section name to look up
        #[arg(value_name = "SECTION")]
        section: String,
    },
    /// Display information about authors
    Authors,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Parse { file } => {
            let content = fs::read_to_string(file)
                .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file.display(), e))?;

            let manifest = Manifest::parse(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse manifest: {}", e))?;

            println!("Parsed manifest sections:");
            for section in manifest.sections() {
                println!("- {}", section);
            }
        }
        Commands::GetByKey { file, section, key } => {
            let content = fs::read_to_string(file)
                .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file.display(), e))?;

            let manifest = Manifest::parse(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse manifest: {}", e))?;

            match manifest.get_by_key(section, key) {
                Ok(value) => println!("{} = {}", key, value),
                Err(e) => println!("Error: {}", e),
            }
        }
        Commands::GetBySection { file, section } => {
            let content = fs::read_to_string(file)
                .map_err(|e| anyhow::anyhow!("Failed to read file {}: {}", file.display(), e))?;

            let manifest = Manifest::parse(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse manifest: {}", e))?;

            match manifest.get_by_section(section) {
                Ok(section_map) => {
                    println!("Values in section [{}]:", section);
                    for (key, value) in section_map {
                        println!("{} = {}", key, value);
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Commands::Authors => {
            println!("Manifest Parser");
            println!("Created by Official-Echo");
            println!("GitHub: https://github.com/Official-Echo");
            println!("Repository: https://github.com/Official-Echo/manifest_parser_rs");
        }
    }

    Ok(())
}
