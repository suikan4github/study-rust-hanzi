//! # I/O Module
//!
//! This module handles file input/output operations for the Hanzi analysis library.
//! It provides functions for reading and parsing TSV files containing Chinese character data.

use crate::types::{HanziOnset, HanziRecord, HanziRime};
use std::io::BufRead;

/// Reads a TSV file containing Hanzi data and returns a vector of HanziRecord
///
/// This function parses a tab-separated values file where each line represents
/// one Chinese character with its associated data. The expected format is:
/// frequency, simplified, traditional, pinyin, pinyin_without_tone, tone
///
/// # Arguments
///
/// * `file_path` - Path to the TSV file to read
///
/// # Returns
///
/// * `Ok(Vec<HanziRecord>)` - Successfully parsed records
/// * `Err(std::io::Error)` - File I/O error occurred
///
/// # File Format
///
/// Each line should contain 6 tab-separated fields:
/// 1. Frequency rank (integer)
/// 2. Simplified character (string)
/// 3. Traditional character (string)  
/// 4. Pinyin with tone marks (string)
/// 5. Pinyin without tone marks (string)
/// 6. Tone number (integer, 1-5)
///
/// Lines with fewer than 6 fields are skipped. Invalid numbers default to 0.
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
            onset: HanziOnset::None, // Set as initial value
            rime: HanziRime::None,   // Set as initial value
        };
        records.push(record);
    }
    Ok(records)
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

        let tenth_record = &records[9]; // 10th element (index 9)
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

        let last_record = &records[4999]; // 5000th element (index 4999)
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
}
