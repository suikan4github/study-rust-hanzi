use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io::{self, Write};
use study_rust_kanji::{format_pinyin_output,format_tone_output, group_by_pinyin, group_by_tone, read_hanzi_file};

/// Hanzi learning program
#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

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

fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

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

