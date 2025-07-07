//! # Hanzi Learning Program
//!
//! A command-line tool for analyzing and studying Chinese characters (Hanzi) based on their
//! pinyin pronunciation. This program helps Chinese language learners by providing various
//! ways to group and display characters according to their phonetic properties.
//!
//! ## Features
//!
//! - **by-pinyin**: Groups characters by their pinyin pronunciation (without tone marks)
//! - **by-tone**: Filters characters by specific pinyin and displays them grouped by tone
//! - **generate-completion**: Creates shell completion scripts for better CLI experience
//!
//! ## Examples
//!
//! ```bash
//! # List all characters grouped by pinyin with character count
//! study-rust-hanzi by-pinyin
//!
//! # List characters with line folding at 30 characters
//! study-rust-hanzi by-pinyin --fold 30
//!
//! # Show traditional characters instead of simplified
//! study-rust-hanzi by-pinyin --traditional
//!
//! # Show all characters with pinyin "ma" grouped by tone
//! study-rust-hanzi by-tone ma
//!
//! # Show traditional characters for pinyin "nv" (converted to "nü")
//! study-rust-hanzi by-tone nv --traditional
//!
//! # Generate bash completion script
//! study-rust-hanzi generate-completion bash > completion.bash
//! ```
//!
//! ## Data Source
//!
//! The program reads character data from a `hanzi.tsv` file in the current directory,
//! which should contain tab-separated values with frequency, simplified character,
//! traditional character, pinyin with tone marks, pinyin without tone marks, and tone number.

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io::{self, Write};
use study_rust_kanji::{
    format_pinyin_output, format_tone_output, group_by_pinyin, group_by_tone, read_hanzi_file,
};

/// Hanzi learning program
///
/// This program provides functionality to analyze and display Chinese characters (Hanzi)
/// based on their pinyin pronunciation. It supports grouping by pinyin without tone marks,
/// displaying characters by specific tones, and generating shell completion scripts.
#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

/// Available commands for the Hanzi learning program
///
/// This enum defines the three main operations supported by the application:
/// - Listing characters grouped by pinyin pronunciation
/// - Showing characters filtered by specific pinyin and grouped by tone
/// - Generating shell completion scripts for better CLI experience
#[derive(Subcommand)]
enum Commands {
    /// List unique pinyin with frequency and characters
    ByPinyin {
        /// Fold long lines when character count exceeds specified value (default: 50)
        #[arg(short = 'f', long, value_name = "WIDTH", default_missing_value = "50", num_args = 0..=1)]
        fold: Option<usize>,
        /// Use traditional characters instead of simplified
        #[arg(short = 'r', long)]
        traditional: bool,
    },
    /// Show characters by tone for specified pinyin
    ByTone {
        /// The pinyin (without tone marks) to search for. Use 'v' for 'ü' (e.g., 'nv' for 'nü')
        pinyin: String,
        /// Use traditional characters instead of simplified
        #[arg(short = 'r', long)]
        traditional: bool,
    },
    /// Generate shell completion scripts
    GenerateCompletion {
        /// The shell to generate completion script for
        #[arg(value_enum)]
        shell: Shell,
    },
}

/// Processes the by-pinyin command to display characters grouped by pinyin
///
/// This function reads the hanzi data file, groups characters by their pinyin pronunciation
/// (without tone marks), and displays them with optional line folding for better readability.
/// Characters are sorted by frequency (most common first) and then alphabetically by pinyin.
///
/// # Arguments
///
/// * `fold_size` - Optional width for line folding. If specified, character lists longer
///   than this width will be wrapped to multiple lines for better readability
/// * `use_traditional` - Whether to display traditional characters instead of simplified
///
/// # Behavior
///
/// - Reads hanzi data from "hanzi.tsv" file
/// - Groups characters by pinyin without tone marks
/// - Formats output with character counts and optional line folding
/// - Handles broken pipe errors gracefully (useful for piped output)
/// - Exits with error code 1 if the data file cannot be read
fn process_by_pinyin(fold_size: Option<usize>, use_traditional: bool) {
    match read_hanzi_file("hanzi.tsv") {
        Ok(records) => {
            // Separated into testable functions
            let grouped_data = group_by_pinyin(&records, use_traditional);
            let output_lines = format_pinyin_output(&grouped_data, fold_size);

            for line in output_lines {
                if writeln!(std::io::stdout(), "{line}").is_err() {
                    break; // Broken pipe handling: exit quietly when pipe is closed
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading hanzi.tsv: {e}");
            std::process::exit(1);
        }
    }
}

/// Processes the by-tone command to display characters filtered by pinyin and grouped by tone
///
/// This function takes a target pinyin (without tone marks), finds all matching characters,
/// and groups them by their tone numbers. It automatically converts 'v' to 'ü' for easier
/// typing of pinyin containing the ü sound.
///
/// # Arguments
///
/// * `target_pinyin` - The pinyin to search for (without tone marks). 'v' is automatically
///   converted to 'ü' for convenience (e.g., 'nv' becomes 'nü')
/// * `use_traditional` - Whether to display traditional characters instead of simplified
///
/// # Behavior
///
/// - Normalizes input by replacing 'v' with 'ü'
/// - Reads hanzi data from "hanzi.tsv" file
/// - Filters records matching the target pinyin
/// - Groups matching characters by tone (1, 2, 3, 4, 5 for neutral tone)
/// - Displays results with tone marks and character lists
/// - Shows "No characters found" message if no matches
/// - Exits with error code 1 if the data file cannot be read
fn process_by_tone(target_pinyin: &str, use_traditional: bool) {
    // Replace 'v' with 'ü' in pinyin input (common typing convention)
    let normalized_pinyin = target_pinyin.replace('v', "ü");

    match read_hanzi_file("hanzi.tsv") {
        Ok(records) => match group_by_tone(&records, &normalized_pinyin, use_traditional) {
            Some(tone_groups) => {
                let output_lines = format_tone_output(&tone_groups);
                for line in output_lines {
                    println!("{line}");
                }
            }
            None => {
                println!("No characters found for pinyin: {normalized_pinyin}");
            }
        },
        Err(e) => {
            eprintln!("Error reading hanzi.tsv: {e}");
            std::process::exit(1);
        }
    }
}

/// Generates and prints shell completion scripts to stdout
///
/// This function uses the clap_complete crate to generate completion scripts
/// for various shells (bash, zsh, fish, etc.). The generated script can be
/// sourced in the user's shell configuration to enable tab completion for
/// the application's commands and arguments.
///
/// # Arguments
///
/// * `gen` - A generator that implements the clap_complete::Generator trait
///   for the specific shell type
/// * `cmd` - A mutable reference to the clap Command structure used to
///   generate completions based on the application's CLI definition
///
/// # Output
///
/// Prints the completion script to stdout, which can be redirected to a file
/// or directly sourced by the shell
fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

/// Main entry point for the Hanzi learning program
///
/// This function parses command-line arguments and dispatches to the appropriate
/// handler function based on the selected subcommand. It supports three main operations:
///
/// 1. **by-pinyin**: Groups and displays characters by pinyin pronunciation
/// 2. **by-tone**: Filters characters by specific pinyin and groups by tone
/// 3. **generate-completion**: Creates shell completion scripts
///
/// The function uses the clap crate for argument parsing and provides comprehensive
/// help messages and validation for all commands and options.
fn main() {
    let args = Args::parse();

    match args.command {
        Commands::ByPinyin { fold, traditional } => {
            process_by_pinyin(fold, traditional);
        }
        Commands::ByTone {
            pinyin,
            traditional,
        } => {
            process_by_tone(&pinyin, traditional);
        }
        Commands::GenerateCompletion { shell } => {
            let mut cmd = Args::command();
            eprintln!("Generating completion file for {shell}...");
            print_completions(shell, &mut cmd);
        }
    }
}
