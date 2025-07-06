use clap::Parser;
use study_rust_kanji::read_hanzi_file;
use std::collections::HashMap;

/// Hanzi learning program
#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version)]
struct Args {
    /// Sort by pinyin
    #[arg(short = 'p', long)]
    by_pinyin: bool,
}

fn main() {
    let args = Args::parse();
    
    if args.by_pinyin {
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
                    println!("{:<5} {:3} {}", pinyin, characters.len(), char_list);
                }
            }
            Err(e) => {
                eprintln!("Error reading hanzi.tsv: {}", e);
                std::process::exit(1);
            }
        }
    }
}
