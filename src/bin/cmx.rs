// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2021-2025, Harbers Bik LLC

use anyhow::{Context, Result};
use clap::Parser;
use cmx::profile::{Profile, RawProfile};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

// CMX - Color Management eXtensions
// cargo install cmx
// locally: cargo install --path .
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ICC profile to process
    #[arg(value_name = "PROFILE")]
    profile: PathBuf,

    /// Output file (defaults to stdout if not provided)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Read the ICC profile
    let profile_bytes = fs::read(&cli.profile)
        .with_context(|| format!("Failed to read profile from {:?}", cli.profile))?;

    // Parse the profile
    let raw_profile = RawProfile::from_bytes(&profile_bytes)
        .with_context(|| format!("Failed to parse profile {:?}", cli.profile))?;

    let profile = Profile::Raw(raw_profile);
    let toml_output = profile.to_string();

    // Output based on -o flag
    if let Some(output_path) = cli.output {
        fs::write(&output_path, toml_output)
            .with_context(|| format!("Failed to write to {output_path:?}"))?;
    } else {
        io::stdout().write_all(toml_output.as_bytes())?;
    }

    Ok(())
}
