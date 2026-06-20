#![warn(clippy::pedantic)]

use std::env;
use std::error::Error;
use std::path::PathBuf;

use chrono::Local;
use clap::{Parser, Subcommand};

use crate::close::close;

mod close;

#[cfg(test)]
mod test_fixtures;
#[cfg(test)]
mod test_utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the monthly closing process
    Close {
        /// the budget configuration file in TOML format
        budget_config_file: PathBuf,
        /// The spreadsheet export file from the accounting software
        accounts_file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    configure_the_environment();
    let cli = Cli::parse();
    let ts = &ts_now();
    match &cli.command {
        Commands::Close {
            budget_config_file,
            accounts_file,
        } => close(budget_config_file, accounts_file, ts),
    }
}

pub fn configure_the_environment() {
    unsafe {
        env::set_var("POLARS_FMT_TABLE_ROUNDED_CORNERS", "1");
        env::set_var("POLARS_FMT_MAX_COLS", "70");
        env::set_var("POLARS_FMT_MAX_ROWS", "100");
        env::set_var("POLARS_FMT_STR_LEN", "50");
    };
}

/// Returns the current timestamp in format `<yyyymmdd_HHMMSS>`.
fn ts_now() -> String {
    Local::now().format("%Y%m%d_%H%M%S").to_string()
}

/// Derives the month from the account filename (e.g. `konten_<yyyymm>.xls` -> `<yyyymm>`)
/// # Errors
/// Will return `Err` if `file` does not provide the information on the month.
pub fn derive_month_from_accounts(file: Option<&str>, extension: &str) -> Result<String, String> {
    derive_month_from(file, "konten", extension)
}

/// Derive the month from the file starting with specified prefix
fn derive_month_from(file: Option<&str>, prefix: &str, extension: &str) -> Result<String, String> {
    let min = format!("{prefix}_yyyymm.{extension}");
    let underscore_index = min.find('_').unwrap();
    let static_part = &min[0..=underscore_index];
    let Some(filename) = file else {
        return Err("Unable to derive filename for intermediate file.".into());
    };
    if filename.len() < min.len() {
        Err(format!(
            "Filename '{filename}' should have at least {} characters ({min}), but has only {}.",
            min.len(),
            filename.len()
        ))
    } else if !filename.starts_with(static_part) {
        Err(format!("Filename must start with '{static_part}'."))
    } else {
        let path = std::path::Path::new(filename);
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
        {
            let start_index = underscore_index + 1;
            let end_index = start_index + 5;
            Ok(filename[start_index..=end_index].into())
        } else {
            Err(format!("Filename must have extension .{extension}.").to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn verify_cli() {
        use ::clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_ts_new() {
        let ts = ts_now();
        println!("timestamp: {ts}");
        assert!(&ts[..8].parse::<i64>().is_ok());
        assert_eq!("_", &ts[8..9]);
        assert!(&ts[9..].parse::<i64>().is_ok());
    }

    #[rstest]
    #[case(Some("konten_202409.xlsx"), "xlsx", "202409")]
    #[case(Some("konten_202410_20241113090027.xlsx"), "xlsx", "202410")]
    #[case(Some("konten_202410.xls"), "xls", "202410")]
    fn test_derive_month_from_accounts(
        #[case] input: Option<&str>,
        #[case] extension: &str,
        #[case] expected: String,
    ) {
        let result = derive_month_from_accounts(input, extension);
        match result {
            Ok(month) => assert_eq!(month, expected),
            Err(msg) => assert_eq!(msg, expected),
        }
    }
}
