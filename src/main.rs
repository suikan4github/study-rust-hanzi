use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use study_rust_kanji::read_hanzi_file;
use std::collections::HashMap;
use std::io::{self, Write};

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
        /// The pinyin (without tone marks) to search for
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
            // Group characters by pinyin_without_tone and collect them in frequency order
            let mut pinyin_groups: HashMap<&str, Vec<&str>> = HashMap::new();
            for record in &records {
                let character = if use_traditional { &record.traditional } else { &record.simplified };
                pinyin_groups.entry(&record.pinyin_without_tone)
                    .or_insert_with(Vec::new)
                    .push(character);
            }
            
            // Sort by frequency (descending) and then by pinyin (ascending)
            let mut sorted_pinyins: Vec<_> = pinyin_groups.iter().collect();
            sorted_pinyins.sort_by(|a, b| {
                b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0))
            });
            
            for (pinyin, characters) in sorted_pinyins {
                let char_list = characters.join("");
                
                if let Some(fold_size) = fold_size {
                    if char_list.len() > fold_size {
                        // Fold long lines: first fold_size chars on the same line as count
                        let chars: Vec<char> = char_list.chars().collect();
                        let first_chunk: String = chars.iter().take(fold_size).collect();
                        
                        if let Err(_) = writeln!(std::io::stdout(), "{:<8}: {:3} {}", pinyin, characters.len(), first_chunk) {
                            break;
                        }
                        
                        // Remaining characters in chunks of fold_size
                        for chunk in chars.iter().skip(fold_size).collect::<Vec<_>>().chunks(fold_size) {
                            let chunk_str: String = chunk.iter().map(|c| **c).collect();
                            if let Err(_) = writeln!(std::io::stdout(), "              {}", chunk_str) {
                                break;
                            }
                        }
                    } else {
                        // Normal single line output
                        if let Err(_) = writeln!(std::io::stdout(), "{:<8}: {:3} {}", pinyin, characters.len(), char_list) {
                            break; // Broken pipe handling: exit quietly when pipe is closed
                        }
                    }
                } else {
                    // Normal single line output (no folding)
                    if let Err(_) = writeln!(std::io::stdout(), "{:<8}: {:3} {}", pinyin, characters.len(), char_list) {
                        break; // Broken pipe handling: exit quietly when pipe is closed
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading hanzi.tsv: {}", e);
            std::process::exit(1);
        }
    }
}

fn process_by_tone(target_pinyin: &str, use_traditional: bool) {
    match read_hanzi_file("hanzi.tsv") {
        Ok(records) => {
            let matching_records: Vec<_> = records
                .iter()
                .filter(|record| record.pinyin_without_tone == target_pinyin)
                .collect();
            
            if matching_records.is_empty() {
                println!("No characters found for pinyin: {}", target_pinyin);
            } else {
                // Group by tone and collect both characters and representative pinyin
                let mut tone_groups: HashMap<u32, (Vec<&str>, &str)> = HashMap::new();
                for record in matching_records {
                    let character = if use_traditional { &record.traditional } else { &record.simplified };
                    let entry = tone_groups.entry(record.tone)
                        .or_insert_with(|| (Vec::new(), &record.pinyin));
                    entry.0.push(character);
                }
                
                // Sort by tone (1, 2, 3, 4, 5 for neutral tone)
                let mut sorted_tones: Vec<_> = tone_groups.iter().collect();
                sorted_tones.sort_by_key(|&(tone, _)| *tone);
                
                for (_tone, (characters, pinyin)) in sorted_tones {
                    let char_list = characters.join("");
                    println!("{}: {}", pinyin, char_list);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading hanzi.tsv: {}", e);
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
        Commands::ByTone { pinyin, traditional } => {
            process_by_tone(&pinyin, traditional);
        }
        Commands::GenerateCompletion { shell } => {
            let mut cmd = Args::command();
            eprintln!("Generating completion file for {}...", shell);
            print_completions(shell, &mut cmd);
        }
    }
}
