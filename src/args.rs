use std::{fmt::Display, path::PathBuf};

pub use anyhow::{Context, Ok, Result};
pub use clap::{command, CommandFactory, Parser};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(name = "astro-obsidian", version, about, long_about = None)]
/// A blazingly fast utility for building a json graph from obsidian files.
pub struct CliArgs {
    /// Path to obsidian folder you want to parse
    pub path: std::path::PathBuf,
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
    pub fn is_valid(&self) -> Result<(), ValidationErrors> {
        self.path
            .exists()
            .then(|| ())
            .ok_or_else(|| ValidationErrors::BadPath(self.path.clone()))
    }
}
