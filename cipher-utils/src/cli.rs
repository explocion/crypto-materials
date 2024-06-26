use crate::bytes::Bytes;

use base64::prelude::*;
use clap::{Parser, Subcommand};
use regex::Regex;
use std::{error::Error, fmt, path::PathBuf};

#[derive(Debug)]
pub struct IllegalCharacter(pub char);

impl fmt::Display for IllegalCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "illegal character \'{}\'", self.0)
    }
}

impl Error for IllegalCharacter {}

pub fn verified_message(message: &str) -> Result<&str, IllegalCharacter> {
    use once_cell::sync::Lazy;
    static ILLEGAL_CHARACTER: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"[^[[:alnum:]]\,\.\;\?\!\(\)]").unwrap());

    match ILLEGAL_CHARACTER.find(message) {
        Some(m) => Err(IllegalCharacter(m.as_str().chars().next().unwrap())),
        None => Ok(message),
    }
}

pub fn parse_message(message: &str) -> Result<Bytes, IllegalCharacter> {
    verified_message(message).map(|message| message.bytes().collect())
}

pub fn parse_base64(value: &str) -> Result<Bytes, base64::DecodeError> {
    BASE64_URL_SAFE.decode(value).map(|bytes| bytes.into())
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub enum Command {
    /// generate a new key for the cipher
    Generate,
    /// encrypt a secret message with the key in the given file, or with default key if not specified
    Encrypt {
        #[arg(value_parser = parse_message)]
        secret_message: Bytes,
        #[arg(short, long)]
        key: Option<PathBuf>,
    },
    /// decrypt an encrypted message with the key in the given file, or with default key if not specified
    Decrypt {
        #[arg(value_parser = parse_base64)]
        encrypted_message: Bytes,
        #[arg(short, long)]
        key: Option<PathBuf>,
    },
}

#[derive(Debug, PartialEq, Eq, Parser)]
#[command(author, version, about, long_about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

pub fn command() -> Command {
    Cli::parse().command
}
