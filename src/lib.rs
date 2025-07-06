use std::io::BufRead;

// Ennumeration of Hanzi Onsets.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HanziOnset {
    B,
    P,
    M,
    F,
    D,
    T,
    N,
    Z,
    C,
    S,
    L,
    Zh,
    Ch,
    Sh,
    R,
    J,
    Q,
    X,
    G,
    K,
    H,
    Y,
    W,
    None,
}
pub struct HanziRecord {
    pub frequency: u32,
    pub simplified: String,
    pub traditional: String,
    pub pinyin: String,
    pub pinyin_without_tone: String,
    pub tone: u32,
    pub onset: HanziOnset,
}

/// Read a specified TSV file and return a vector of HanziRecord
pub fn read_hanzi_file(file_path: &str) -> std::io::Result<Vec<HanziRecord>> {
    let mut records = Vec::new();
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 6 {
            continue; // Skip lines that do not have enough fields
        }
        let record = HanziRecord {
            frequency: parts[0].parse().unwrap_or(0),
            simplified: parts[1].to_string(),
            traditional: parts[2].to_string(),
            pinyin: parts[3].to_string(),
            pinyin_without_tone: parts[4].to_string(),
            tone: parts[5].parse().unwrap_or(0),
            onset: HanziOnset::None, // 初期値として設定
        };
        records.push(record);
    }
    Ok(records)
}

/// Vec<HanziRecord> を受け取り、すべてのレコードのpinyin_without_toneを調べる。
/// もし、そのフィールドがHanziOnsetの文字表現のいずれかで始まるならば、
/// そのHanziOnsetの値をonsetフィールドに設定する。どれにも適合しなければnoneを設定する。
pub fn set_hanzi_onsets(records: &mut Vec<HanziRecord>) {
    for record in records.iter_mut() {
        let pinyin = &record.pinyin_without_tone;
        record.onset = if pinyin.starts_with("zh") {
            HanziOnset::Zh
        } else if pinyin.starts_with("ch") {
            HanziOnset::Ch
        } else if pinyin.starts_with("sh") {
            HanziOnset::Sh
        } else if pinyin.starts_with("b") {
            HanziOnset::B
        } else if pinyin.starts_with("p") {
            HanziOnset::P
        } else if pinyin.starts_with("m") {
            HanziOnset::M
        } else if pinyin.starts_with("f") {
            HanziOnset::F
        } else if pinyin.starts_with("d") {
            HanziOnset::D
        } else if pinyin.starts_with("t") {
            HanziOnset::T
        } else if pinyin.starts_with("n") {
            HanziOnset::N
        } else if pinyin.starts_with("z") {
            HanziOnset::Z
        } else if pinyin.starts_with("c") {
            HanziOnset::C
        } else if pinyin.starts_with("s") {
            HanziOnset::S
        } else if pinyin.starts_with("l") {
            HanziOnset::L
        } else if pinyin.starts_with("r") {
            HanziOnset::R
        } else if pinyin.starts_with("j") {
            HanziOnset::J
        } else if pinyin.starts_with("q") {
            HanziOnset::Q
        } else if pinyin.starts_with("x") {
            HanziOnset::X
        } else if pinyin.starts_with("g") {
            HanziOnset::G
        } else if pinyin.starts_with("k") {
            HanziOnset::K
        } else if pinyin.starts_with("h") {
            HanziOnset::H
        } else if pinyin.starts_with("y") {
            HanziOnset::Y
        } else if pinyin.starts_with("w") {
            HanziOnset::W
        } else {
            HanziOnset::None
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_hanzi_file_length() {
        let result = read_hanzi_file("hanzi.tsv");
        assert!(result.is_ok(), "Failed to read hanzi.tsv file");

        let records = result.unwrap();
        assert_eq!(
            records.len(),
            5000,
            "Expected 5000 records in hanzi.tsv, got {}",
            records.len()
        );
    }

    #[test]
    fn test_read_hanzi_file_tenth_element() {
        let result = read_hanzi_file("hanzi.tsv");
        assert!(result.is_ok(), "Failed to read hanzi.tsv file");

        let records = result.unwrap();
        assert!(
            records.len() >= 10,
            "Not enough records in hanzi.tsv, need at least 10"
        );

        let tenth_record = &records[9]; // 10番目の要素（インデックス9）
        assert_eq!(
            tenth_record.frequency, 10,
            "Expected frequency 10, got {}",
            tenth_record.frequency
        );
        assert_eq!(
            tenth_record.simplified, "他",
            "Expected simplified '他', got '{}'",
            tenth_record.simplified
        );
        assert_eq!(
            tenth_record.traditional, "他",
            "Expected traditional '他', got '{}'",
            tenth_record.traditional
        );
        assert_eq!(
            tenth_record.pinyin, "tā",
            "Expected pinyin 'tā', got '{}'",
            tenth_record.pinyin
        );
        assert_eq!(
            tenth_record.pinyin_without_tone, "ta",
            "Expected pinyin_without_tone 'ta', got '{}'",
            tenth_record.pinyin_without_tone
        );
        assert_eq!(
            tenth_record.tone, 1,
            "Expected tone 1, got {}",
            tenth_record.tone
        );
    }

    #[test]
    fn test_read_hanzi_file_last_element() {
        let result = read_hanzi_file("hanzi.tsv");
        assert!(result.is_ok(), "Failed to read hanzi.tsv file");

        let records = result.unwrap();
        assert_eq!(records.len(), 5000, "Expected exactly 5000 records");

        let last_record = &records[4999]; // 5000番目の要素（インデックス4999）
        assert_eq!(
            last_record.frequency, 5000,
            "Expected frequency 5000, got {}",
            last_record.frequency
        );
        assert_eq!(
            last_record.simplified, "鸨",
            "Expected simplified '鸨', got '{}'",
            last_record.simplified
        );
        assert_eq!(
            last_record.traditional, "鴇",
            "Expected traditional '鴇', got '{}'",
            last_record.traditional
        );
        assert_eq!(
            last_record.pinyin, "bǎo",
            "Expected pinyin 'bǎo', got '{}'",
            last_record.pinyin
        );
        assert_eq!(
            last_record.pinyin_without_tone, "bao",
            "Expected pinyin_without_tone 'bao', got '{}'",
            last_record.pinyin_without_tone
        );
        assert_eq!(
            last_record.tone, 3,
            "Expected tone 3, got {}",
            last_record.tone
        );
    }

    #[test]
    fn test_set_hanzi_onsets() {
        let result = read_hanzi_file("hanzi.tsv");
        assert!(result.is_ok(), "Failed to read hanzi.tsv file");

        let mut records = result.unwrap();
        set_hanzi_onsets(&mut records);

        // none以外の全てのHanziOnsetの値が出現するはずである
        use std::collections::HashSet;
        let mut found_onsets = HashSet::new();

        for record in &records {
            found_onsets.insert(&record.onset);
        }

        // none以外の全てのHanziOnsetを定義
        let expected_onsets = vec![
            HanziOnset::B,
            HanziOnset::P,
            HanziOnset::M,
            HanziOnset::F,
            HanziOnset::D,
            HanziOnset::T,
            HanziOnset::N,
            HanziOnset::Z,
            HanziOnset::C,
            HanziOnset::S,
            HanziOnset::L,
            HanziOnset::Zh,
            HanziOnset::Ch,
            HanziOnset::Sh,
            HanziOnset::R,
            HanziOnset::J,
            HanziOnset::Q,
            HanziOnset::X,
            HanziOnset::G,
            HanziOnset::K,
            HanziOnset::H,
            HanziOnset::Y,
            HanziOnset::W,
        ];

        for expected_onset in &expected_onsets {
            assert!(
                found_onsets.contains(expected_onset),
                "HanziOnset::{:?} was not found in any record",
                expected_onset
            );
        }
    }
}
