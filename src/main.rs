use anyhow::Result;
use clap::{Parser, Subcommand};
use manifest_parser_rs::Manifest;
use std::fs;
use std::path::PathBuf;

/// Manifest Parser CLI - A tool for parsing and inspecting manifest files
#[derive(Parser)]
#[command(
    name = "manifest_parser_rs",
    version,
    about,
    color = clap::ColorChoice::Always,
    help_template = "\
\x1b[1m{name}\x1b[0m v{version}
{about}

\x1b[1mUSAGE:\x1b[0m
    {usage}

{all-args}

\x1b[1mEXAMPLES:\x1b[0m
    manifest_parser_rs parse Cargo.toml
    manifest_parser_rs get-by-key Cargo.toml package version

\x1b[1mSUPPORT:\x1b[0m
    Official Repo: https://github.com/Official-Echo/manifest_parser_rs
    Docs: https://docs.rs/manifest_parser_rs
    Crate on crates.io: https://crates.io/crates/manifest_parser_rs
    Author:  Official-Echo🔊🗣️🌀🧏‍♀️
"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and display the contents of a manifest file
    #[command(visible_alias = "p")]
    Parse {
        /// Path to the manifest file to parse
        #[arg(value_name = "FILE", help_heading = "ARGUMENTS")]
        file: PathBuf,
    },

    /// Extract a specific value by section and key
    #[command(visible_alias = "get")]
    GetByKey {
        /// Path to the manifest file
        #[arg(value_name = "FILE", help_heading = "ARGUMENTS")]
        file: PathBuf,
        /// Section name to search in
        #[arg(value_name = "SECTION")]
        section: String,
        /// Key to look up
        #[arg(value_name = "KEY")]
        key: String,
    },

    /// Get all key-value pairs from a section
    #[command(visible_alias = "section")]
    GetBySection {
        /// Path to the manifest file
        #[arg(value_name = "FILE", help_heading = "ARGUMENTS")]
        file: PathBuf,
        /// Section to display
        #[arg(value_name = "SECTION")]
        section: String,
    },

    /// Show information about the authors
    #[command(visible_alias = "a")]
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
