use clap::Parser;
use study_rust_kanji::read_hanzi_file;
use std::collections::HashMap;

/// Hanzi learning program
#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version)]
struct Args {
    /// Sort by pinyin
    #[arg(short = 'p', long, group = "mode")]
    by_pinyin: bool,
    
    /// Show characters by tone for specified pinyin
    #[arg(short = 't', long, group = "mode")]
    by_tone: Option<String>,
}

fn process_by_pinyin() {
    match read_hanzi_file("hanzi.tsv") {
        Ok(records) => {
            // Group characters by pinyin_without_tone and collect them in frequency order
            let mut pinyin_groups: HashMap<&str, Vec<&str>> = HashMap::new();
            for record in &records {
                pinyin_groups.entry(&record.pinyin_without_tone)
                    .or_insert_with(Vec::new)
                    .push(&record.simplified);
            }
            
            // Sort by frequency (descending) and then by pinyin (ascending)
            let mut sorted_pinyins: Vec<_> = pinyin_groups.iter().collect();
            sorted_pinyins.sort_by(|a, b| {
                b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0))
            });
            
            for (pinyin, characters) in sorted_pinyins {
                let char_list = characters.join("");
                println!("{:<5}: {:3} {}", pinyin, characters.len(), char_list);
            }
        }
        Err(e) => {
            eprintln!("Error reading hanzi.tsv: {}", e);
            std::process::exit(1);
        }
    }
}

fn process_by_tone(target_pinyin: &str) {
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
                    let entry = tone_groups.entry(record.tone)
                        .or_insert_with(|| (Vec::new(), &record.pinyin));
                    entry.0.push(&record.simplified);
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

fn main() {
    let args = Args::parse();
    
    if args.by_pinyin {
        process_by_pinyin();
    }
    
    if let Some(target_pinyin) = args.by_tone {
        process_by_tone(&target_pinyin);
    }
}
