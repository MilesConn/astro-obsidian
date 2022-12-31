use std::{fmt::Display, path::PathBuf};

pub use anyhow::{Context, Error, Ok, Result};
pub use clap::{command, CommandFactory, Parser};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(name = "astro-obsidian", version, about, long_about = None)]
/// A blazingly fast utility for building a json graph from obsidian files.
pub struct CliArgs {
    #[clap(short, long)]
    /// Path to obsidian folder you want to parse
    pub path: std::path::PathBuf,
    #[clap(short, long)]
    /// Path to output json to
    pub output: std::path::PathBuf,
}

#[derive(Error, Debug)]
pub enum ValidationErrors {
    BadPath(PathBuf),
}

impl Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationErrors::BadPath(path) => {
                writeln!(f, "Path: {} doesn't exist", path.to_string_lossy())
            }
        }
    }
}

impl CliArgs {
    pub fn is_valid(&self) -> Result<(), Error> {
        self.path
            .exists()
            .then(|| ())
            .ok_or_else(|| ValidationErrors::BadPath(self.path.clone()))?;

        // LOL I do not care
        // self.output.metadata()?.permissions().mode();

        Ok(())
    }
}
