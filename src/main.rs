use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::collections::HashMap;
use std::io::{self, Write};
use study_rust_kanji::read_hanzi_file;

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
                if writeln!(std::io::stdout(), "{}", line).is_err() {
                    break; // Broken pipe handling: exit quietly when pipe is closed
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
    // Replace 'v' with 'ü' in pinyin input (common typing convention)
    let normalized_pinyin = target_pinyin.replace('v', "ü");

    match read_hanzi_file("hanzi.tsv") {
        Ok(records) => match group_by_tone(&records, &normalized_pinyin, use_traditional) {
            Some(tone_groups) => {
                let output_lines = format_tone_output(&tone_groups);
                for line in output_lines {
                    println!("{}", line);
                }
            }
            None => {
                println!("No characters found for pinyin: {}", normalized_pinyin);
            }
        },
        Err(e) => {
            eprintln!("Error reading hanzi.tsv: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

// Separated into testable functions
pub fn group_by_pinyin(
    records: &[study_rust_kanji::HanziRecord],
    use_traditional: bool,
) -> Vec<(String, Vec<String>)> {
    use std::collections::HashMap;

    let mut pinyin_groups: HashMap<&str, Vec<&str>> = HashMap::new();
    for record in records {
        let character = if use_traditional {
            &record.traditional
        } else {
            &record.simplified
        };
        pinyin_groups
            .entry(&record.pinyin_without_tone)
            .or_default()
            .push(character);
    }

    // Sort by frequency (descending) and then by pinyin (ascending)
    let mut sorted_pinyins: Vec<_> = pinyin_groups.iter().collect();
    sorted_pinyins.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0)));

    sorted_pinyins
        .into_iter()
        .map(|(pinyin, characters)| {
            (
                pinyin.to_string(),
                characters.iter().map(|s| s.to_string()).collect(),
            )
        })
        .collect()
}

pub fn format_pinyin_output(
    grouped_data: &[(String, Vec<String>)],
    fold_size: Option<usize>,
) -> Vec<String> {
    let mut output_lines = Vec::new();

    for (pinyin, characters) in grouped_data {
        let char_list = characters.join("");

        if let Some(fold_size) = fold_size {
            if char_list.len() > fold_size {
                // Fold long lines: first fold_size chars on the same line as count
                let chars: Vec<char> = char_list.chars().collect();
                let first_chunk: String = chars.iter().take(fold_size).collect();

                output_lines.push(format!(
                    "{:<8}: {:3} {}",
                    pinyin,
                    characters.len(),
                    first_chunk
                ));

                // Remaining characters in chunks of fold_size
                for chunk in chars
                    .iter()
                    .skip(fold_size)
                    .collect::<Vec<_>>()
                    .chunks(fold_size)
                {
                    let chunk_str: String = chunk.iter().map(|c| **c).collect();
                    output_lines.push(format!("              {}", chunk_str));
                }
            } else {
                output_lines.push(format!(
                    "{:<8}: {:3} {}",
                    pinyin,
                    characters.len(),
                    char_list
                ));
            }
        } else {
            output_lines.push(format!(
                "{:<8}: {:3} {}",
                pinyin,
                characters.len(),
                char_list
            ));
        }
    }

    output_lines
}

// Separated into testable functions
pub fn group_by_tone(
    records: &[study_rust_kanji::HanziRecord],
    target_pinyin: &str,
    use_traditional: bool,
) -> Option<Vec<(u32, String, Vec<String>)>> {
    let matching_records: Vec<_> = records
        .iter()
        .filter(|record| record.pinyin_without_tone == target_pinyin)
        .collect();

    if matching_records.is_empty() {
        return None;
    }

    let mut tone_groups: HashMap<u32, (Vec<&str>, &str)> = HashMap::new();
    for record in matching_records {
        let character = if use_traditional {
            &record.traditional
        } else {
            &record.simplified
        };
        let entry = tone_groups
            .entry(record.tone)
            .or_insert_with(|| (Vec::new(), &record.pinyin));
        entry.0.push(character);
    }

    // Sort by tone (1, 2, 3, 4, 5 for neutral tone)
    let mut sorted_tones: Vec<_> = tone_groups.iter().collect();
    sorted_tones.sort_by_key(|&(tone, _)| *tone);

    Some(
        sorted_tones
            .into_iter()
            .map(|(tone, (characters, pinyin))| {
                (
                    *tone,
                    pinyin.to_string(),
                    characters.iter().map(|s| s.to_string()).collect(),
                )
            })
            .collect(),
    )
}

pub fn format_tone_output(tone_groups: &[(u32, String, Vec<String>)]) -> Vec<String> {
    tone_groups
        .iter()
        .map(|(_tone, pinyin, characters)| {
            let char_list = characters.join("");
            format!("{}: {}", pinyin, char_list)
        })
        .collect()
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
            eprintln!("Generating completion file for {}...", shell);
            print_completions(shell, &mut cmd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use study_rust_kanji::HanziRecord;

    fn create_test_records() -> Vec<HanziRecord> {
        vec![
            HanziRecord {
                frequency: 1,
                simplified: "机".to_string(),
                traditional: "機".to_string(),
                pinyin: "jī".to_string(),
                pinyin_without_tone: "ji".to_string(),
                tone: 1,
                onset: study_rust_kanji::HanziOnset::J,
                rime: study_rust_kanji::HanziRime::I,
            },
            HanziRecord {
                frequency: 2,
                simplified: "计".to_string(),
                traditional: "計".to_string(),
                pinyin: "jì".to_string(),
                pinyin_without_tone: "ji".to_string(),
                tone: 4,
                onset: study_rust_kanji::HanziOnset::J,
                rime: study_rust_kanji::HanziRime::I,
            },
            HanziRecord {
                frequency: 3,
                simplified: "马".to_string(),
                traditional: "馬".to_string(),
                pinyin: "mǎ".to_string(),
                pinyin_without_tone: "ma".to_string(),
                tone: 3,
                onset: study_rust_kanji::HanziOnset::M,
                rime: study_rust_kanji::HanziRime::A,
            },
        ]
    }

    #[test]
    fn test_group_by_pinyin_simplified() {
        let records = create_test_records();
        let grouped = group_by_pinyin(&records, false);

        // ji should come first as it has more characters than ma
        assert_eq!(grouped[0].0, "ji");
        assert_eq!(grouped[0].1, vec!["机", "计"]);
        assert_eq!(grouped[1].0, "ma");
        assert_eq!(grouped[1].1, vec!["马"]);
    }

    #[test]
    fn test_group_by_pinyin_traditional() {
        let records = create_test_records();
        let grouped = group_by_pinyin(&records, true);

        // Traditional characters should be used
        assert_eq!(grouped[0].0, "ji");
        assert_eq!(grouped[0].1, vec!["機", "計"]);
        assert_eq!(grouped[1].0, "ma");
        assert_eq!(grouped[1].1, vec!["馬"]);
    }

    #[test]
    fn test_format_pinyin_output_no_fold() {
        let test_data = vec![
            ("ji".to_string(), vec!["机".to_string(), "计".to_string()]),
            ("ma".to_string(), vec!["马".to_string()]),
        ];

        let output = format_pinyin_output(&test_data, None);

        assert_eq!(output.len(), 2);
        assert!(output[0].contains("ji"));
        assert!(output[0].contains("2"));
        assert!(output[0].contains("机计"));
        assert!(output[1].contains("ma"));
        assert!(output[1].contains("1"));
        assert!(output[1].contains("马"));
    }

    #[test]
    fn test_format_pinyin_output_with_fold() {
        let test_data = vec![(
            "test".to_string(),
            vec![
                "一".to_string(),
                "二".to_string(),
                "三".to_string(),
                "四".to_string(),
                "五".to_string(),
            ],
        )];

        let output = format_pinyin_output(&test_data, Some(3));

        // fold_size is 3, so first line should have 3 characters, remaining on next line
        assert!(
            output.len() >= 2,
            "Should have at least 2 lines when folding"
        );
        assert!(output[0].contains("test"));
        assert!(output[0].contains("5")); // character count
        assert!(
            output[1].trim().len() > 0,
            "Second line should contain remaining characters"
        );
    }

    #[test]
    fn test_format_pinyin_output_alignment() {
        let test_data = vec![
            ("ji".to_string(), vec!["机".to_string()]),
            ("longpinyin".to_string(), vec!["长".to_string()]),
        ];

        let output = format_pinyin_output(&test_data, None);

        // Test output format
        for line in &output {
            assert!(line.contains(":"), "Each line should contain ':'");
            let parts: Vec<&str> = line.split(':').collect();
            assert_eq!(parts.len(), 2, "Each line should have exactly one ':'");
        }
    }

    #[test]
    fn test_group_by_tone_found() {
        let records = create_test_records();
        let result = group_by_tone(&records, "ji", false);

        assert!(result.is_some());
        let tone_groups = result.unwrap();

        // ji has 2 characters (tone 1: 机, tone 4: 计)
        assert_eq!(tone_groups.len(), 2);

        // Should be sorted by tone order
        assert_eq!(tone_groups[0].0, 1); // tone 1
        assert_eq!(tone_groups[0].2, vec!["机"]); // 机

        assert_eq!(tone_groups[1].0, 4); // tone 4
        assert_eq!(tone_groups[1].2, vec!["计"]); // 计
    }

    #[test]
    fn test_group_by_tone_traditional() {
        let records = create_test_records();
        let result = group_by_tone(&records, "ji", true);

        assert!(result.is_some());
        let tone_groups = result.unwrap();

        // Traditional characters should be used
        assert_eq!(tone_groups[0].2, vec!["機"]); // 機 (traditional)
        assert_eq!(tone_groups[1].2, vec!["計"]); // 計 (traditional)
    }

    #[test]
    fn test_group_by_tone_not_found() {
        let records = create_test_records();
        let result = group_by_tone(&records, "nonexistent", false);

        assert!(result.is_none());
    }

    #[test]
    fn test_group_by_tone_pinyin_with_tone_marks() {
        let records = create_test_records();
        let result = group_by_tone(&records, "ji", false);

        assert!(result.is_some());
        let tone_groups = result.unwrap();

        // pinyin should contain tone marks
        assert_eq!(tone_groups[0].1, "jī"); // tone 1
        assert_eq!(tone_groups[1].1, "jì"); // tone 4
    }

    #[test]
    fn test_format_tone_output() {
        let test_data = vec![
            (1, "jī".to_string(), vec!["机".to_string()]),
            (
                4,
                "jì".to_string(),
                vec!["计".to_string(), "记".to_string()],
            ),
        ];

        let output = format_tone_output(&test_data);

        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "jī: 机");
        assert_eq!(output[1], "jì: 计记");
    }

    #[test]
    fn test_format_tone_output_empty() {
        let test_data = vec![];
        let output = format_tone_output(&test_data);

        assert!(output.is_empty());
    }

    #[test]
    fn test_tone_sorting() {
        let mut records = create_test_records();
        // Additional test data: tone 5 (neutral tone)
        records.push(HanziRecord {
            frequency: 4,
            simplified: "吗".to_string(),
            traditional: "嗎".to_string(),
            pinyin: "ma".to_string(),
            pinyin_without_tone: "ma".to_string(),
            tone: 5, // neutral tone
            onset: study_rust_kanji::HanziOnset::M,
            rime: study_rust_kanji::HanziRime::A,
        });

        let result = group_by_tone(&records, "ma", false);
        assert!(result.is_some());
        let tone_groups = result.unwrap();

        // Should contain tone 3 (马) and tone 5 (吗)
        assert_eq!(tone_groups.len(), 2);
        assert_eq!(tone_groups[0].0, 3); // tone 3 comes first
        assert_eq!(tone_groups[1].0, 5); // tone 5 comes after
    }

    #[test]
    fn test_pinyin_v_to_u_replacement() {
        // Test that 'v' in pinyin input gets replaced with 'ü'
        let records = vec![HanziRecord {
            frequency: 1,
            simplified: "女".to_string(),
            traditional: "女".to_string(),
            pinyin: "nǚ".to_string(),
            pinyin_without_tone: "nü".to_string(),
            tone: 3,
            onset: study_rust_kanji::HanziOnset::N,
            rime: study_rust_kanji::HanziRime::V,
        }];

        // Search with 'v' should not find characters with 'ü' at the low level
        let result = group_by_tone(&records, "nv", false);
        assert!(
            result.is_none(),
            "Direct search with 'v' should not find 'ü' characters"
        );

        // But the normalized search should work
        let result_with_u = group_by_tone(&records, "nü", false);
        assert!(
            result_with_u.is_some(),
            "Search with 'ü' should find characters"
        );
    }
}
