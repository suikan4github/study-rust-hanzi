//! # Grouping Module
//!
//! This module provides functions for grouping and formatting Chinese character data
//! based on pinyin pronunciation and tones. It handles the organization and display
//! of character collections for analysis purposes.

use crate::analysis::set_hanzi_onsets;
use crate::types::{HanziOnset, HanziRecord};
use std::collections::HashMap;

/// Groups Hanzi records by pinyin without tone marks
///
/// Takes a slice of HanziRecord and groups them by their pinyin_without_tone field.
/// Returns a vector of tuples containing the pinyin and a vector of characters.
/// The results are sorted by frequency (descending) and then by pinyin (ascending).
///
/// # Arguments
///
/// * `records` - A slice of HanziRecord to group
/// * `use_traditional` - Whether to use traditional characters instead of simplified
///
/// # Returns
///
/// A vector of tuples where each tuple contains:
/// - The pinyin without tone as a String
/// - A vector of character strings corresponding to that pinyin
///
/// # Sorting Order
///
/// Results are sorted by:
/// 1. Number of characters (descending) - most common pinyin first
/// 2. Pinyin alphabetically (ascending) - consistent ordering for same frequency
///
/// # Examples
///
/// ```rust
/// # use study_rust_hanzi::{HanziRecord, HanziOnset, HanziRime, group_by_pinyin};
/// # let records = vec![]; // Placeholder for actual records
/// let grouped = group_by_pinyin(&records, false); // Use simplified characters
/// // Result: [("de", vec!["的", "得", "地"]), ("yi", vec!["一", "以"]), ...]
/// ```
pub fn group_by_pinyin(
    records: &[HanziRecord],
    use_traditional: bool,
) -> Vec<(String, Vec<String>)> {
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

/// Formats pinyin grouping data for display with optional line folding
///
/// Takes grouped pinyin data and formats it for display, with optional line folding
/// for long character lists. Each line shows the pinyin, character count, and characters.
///
/// # Arguments
///
/// * `grouped_data` - A slice of tuples containing pinyin and character vectors
/// * `fold_size` - Optional width for line folding. If provided, long character lists
///   will be folded to this width with continuation lines
///
/// # Returns
///
/// A vector of formatted strings ready for display
///
/// # Output Format
///
/// Without folding:
/// ```text
/// pinyin  :  42 characters_here
/// ```
///
/// With folding (fold_size = 10):
/// ```text
/// pinyin  :  42 first_10_ch
///               next_chars
/// ```
///
/// # Formatting Details
///
/// - Pinyin is left-aligned in an 8-character field
/// - Character count is right-aligned in a 3-character field
/// - Continuation lines are indented with 14 spaces to align with characters
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
                    output_lines.push(format!("              {chunk_str}"));
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

/// Groups Hanzi records by tone for a specific pinyin
///
/// Filters records by the target pinyin and groups them by tone.
/// Returns None if no matching records are found.
///
/// # Arguments
///
/// * `records` - A slice of HanziRecord to search through
/// * `target_pinyin` - The pinyin (without tone) to filter by
/// * `use_traditional` - Whether to use traditional characters instead of simplified
///
/// # Returns
///
/// An optional vector of tuples where each tuple contains:
/// - The tone number (u32): 1-4 for standard tones, 5 for neutral tone
/// - The pinyin with tone marks as a String
/// - A vector of character strings for that tone
///
/// Returns `None` if no characters match the target pinyin.
///
/// # Tone Sorting
///
/// Results are sorted by tone number (1, 2, 3, 4, 5) in ascending order.
///
/// # Examples
///
/// ```rust
/// # use study_rust_hanzi::{HanziRecord, HanziOnset, HanziRime, group_by_tone};
/// # let records = vec![]; // Placeholder for actual records
/// if let Some(tone_groups) = group_by_tone(&records, "ma", false) {
///     // tone_groups: [(1, "mā", vec!["妈"]), (3, "mǎ", vec!["马"]), ...]
/// }
/// ```
pub fn group_by_tone(
    records: &[HanziRecord],
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

/// Formats tone grouping data for display
///
/// Takes grouped tone data and formats it for display. Each line shows the pinyin
/// with tone marks followed by the corresponding characters for that tone.
///
/// # Arguments
///
/// * `tone_groups` - A slice of tuples containing tone data where each tuple has:
///   - Tone number (u32): 1-4 for standard tones, 5 for neutral tone
///   - Pinyin with tone marks (String): e.g., "jī", "jí", "jǐ", "jì"
///   - Character vector (`Vec<String>`): characters with that pinyin and tone
///
/// # Returns
///
/// A vector of formatted strings ready for display, one per tone group
///
/// # Output Format
///
/// Each line follows the pattern:
/// ```text
/// pinyin_with_tone: characters
/// ```
///
/// # Examples
///
/// ```rust
/// # use study_rust_hanzi::format_tone_output;
/// let tone_data = vec![
///     (1, "mā".to_string(), vec!["妈".to_string()]),
///     (3, "mǎ".to_string(), vec!["马".to_string(), "码".to_string()]),
/// ];
/// let output = format_tone_output(&tone_data);
/// // Result: ["mā: 妈", "mǎ: 马码"]
/// ```
///
/// # Usage with group_by_tone
///
/// This function is typically used in conjunction with [`group_by_tone`]:
/// ```rust,no_run
/// # use study_rust_hanzi::{group_by_tone, format_tone_output};
/// # let records = vec![]; // Placeholder
/// if let Some(tone_groups) = group_by_tone(&records, "ma", false) {
///     let formatted = format_tone_output(&tone_groups);
///     for line in formatted {
///         println!("{}", line);
///     }
/// }
/// ```
pub fn format_tone_output(tone_groups: &[(u32, String, Vec<String>)]) -> Vec<String> {
    tone_groups
        .iter()
        .map(|(_tone, pinyin, characters)| {
            let char_list = characters.join("");
            format!("{pinyin}: {char_list}")
        })
        .collect()
}

/// Groups Hanzi records by onset and returns count for each onset type
///
/// This function first applies onset analysis to the given records using
/// `analysis::set_hanzi_onsets()`, then counts the number of HanziRecord elements
/// for each HanziOnset type. Returns a vector of tuples containing onset and count,
/// sorted by count in descending order.
///
/// # Arguments
///
/// * `records` - A slice of HanziRecord to analyze and group
///
/// # Returns
///
/// An optional vector of tuples where each tuple contains:
/// - The HanziOnset type
/// - The count of records with that onset (u32)
///
/// Returns `None` if the input records slice is empty.
/// The vector is sorted by count in descending order (most frequent onsets first).
///
/// # Examples
///
/// ```rust
/// # use study_rust_hanzi::{HanziRecord, HanziOnset, HanziRime, group_by_onset};
/// # let records = vec![]; // Placeholder for actual records
/// if let Some(onset_counts) = group_by_onset(&records) {
///     // onset_counts: [(HanziOnset::N, 1500), (HanziOnset::L, 1200), ...]
///     for (onset, count) in onset_counts {
///         println!("{:?}: {}", onset, count);
///     }
/// }
/// ```
pub fn group_by_onset(records: &[HanziRecord]) -> Option<Vec<(HanziOnset, u32)>> {
    if records.is_empty() {
        return None;
    }

    // Create a mutable copy of records to apply onset analysis
    let mut records_copy: Vec<HanziRecord> = records.to_vec();

    // Apply onset analysis
    set_hanzi_onsets(&mut records_copy);

    // Count records by onset type
    let mut onset_counts: HashMap<HanziOnset, u32> = HashMap::new();
    for record in &records_copy {
        *onset_counts.entry(record.onset.clone()).or_insert(0) += 1;
    }

    // Convert to vector and sort by count in descending order
    let mut result: Vec<(HanziOnset, u32)> = onset_counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending

    Some(result)
}

/// Formats onset grouping data for display
///
/// Takes grouped onset data and formats it for display. Each line shows the onset
/// type followed by the count of characters with that onset.
///
/// # Arguments
///
/// * `onset_counts` - A slice of tuples containing onset data where each tuple has:
///   - HanziOnset: The onset type (e.g., HanziOnset::J, HanziOnset::M)
///   - u32: The count of records with that onset
///
/// # Returns
///
/// A vector of formatted strings ready for display, one per onset group
///
/// # Output Format
///
/// Each line follows the pattern:
/// ```text
/// onset_name: count
/// ```
///
/// # Examples
///
/// ```rust
/// # use study_rust_hanzi::{HanziOnset, format_onset_output};
/// let onset_data = vec![
///     (HanziOnset::J, 150),
///     (HanziOnset::M, 120),
///     (HanziOnset::None, 80),
/// ];
/// let output = format_onset_output(&onset_data);
/// // Result: ["j: 150", "m: 120", "none: 80"]
/// ```
///
/// # Usage with group_by_onset
///
/// This function is typically used in conjunction with [`group_by_onset`]:
/// ```rust,no_run
/// # use study_rust_hanzi::{group_by_onset, format_onset_output};
/// # let records = vec![]; // Placeholder
/// if let Some(onset_counts) = group_by_onset(&records) {
///     let formatted = format_onset_output(&onset_counts);
///     for line in formatted {
///         println!("{}", line);
///     }
/// }
/// ```
pub fn format_onset_output(onset_counts: &[(HanziOnset, u32)]) -> Vec<String> {
    onset_counts
        .iter()
        .map(|(onset, count)| {
            let onset_name = onset.as_str();
            format!("{}: {}", onset_name, count)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HanziOnset, HanziRime};

    fn create_test_records() -> Vec<HanziRecord> {
        vec![
            HanziRecord {
                frequency: 1,
                simplified: "机".to_string(),
                traditional: "機".to_string(),
                pinyin: "jī".to_string(),
                pinyin_without_tone: "ji".to_string(),
                tone: 1,
                onset: HanziOnset::J,
                rime: HanziRime::I,
            },
            HanziRecord {
                frequency: 2,
                simplified: "计".to_string(),
                traditional: "計".to_string(),
                pinyin: "jì".to_string(),
                pinyin_without_tone: "ji".to_string(),
                tone: 4,
                onset: HanziOnset::J,
                rime: HanziRime::I,
            },
            HanziRecord {
                frequency: 3,
                simplified: "马".to_string(),
                traditional: "馬".to_string(),
                pinyin: "mǎ".to_string(),
                pinyin_without_tone: "ma".to_string(),
                tone: 3,
                onset: HanziOnset::M,
                rime: HanziRime::A,
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
            !output[1].trim().is_empty(),
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

        // Should be sorted in tone order
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
            onset: HanziOnset::M,
            rime: HanziRime::A,
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
    fn test_group_by_onset() {
        let records = create_test_records();
        let result = group_by_onset(&records);

        assert!(result.is_some());
        let onset_counts = result.unwrap();

        // Should have onset counts for the test data
        assert!(!onset_counts.is_empty());

        // Verify that counts are sorted in descending order
        for i in 1..onset_counts.len() {
            assert!(
                onset_counts[i - 1].1 >= onset_counts[i].1,
                "Onset counts should be sorted in descending order"
            );
        }

        // Check that we have the expected onsets from our test data
        let onset_map: std::collections::HashMap<HanziOnset, u32> =
            onset_counts.iter().cloned().collect();

        // From create_test_records: ji (2 records) and ma (1 record)
        // After onset analysis: J onset should have 2, M onset should have 1
        assert!(onset_map.contains_key(&HanziOnset::J));
        assert!(onset_map.contains_key(&HanziOnset::M));
    }

    #[test]
    fn test_group_by_onset_empty() {
        let empty_records: Vec<HanziRecord> = vec![];
        let result = group_by_onset(&empty_records);

        assert!(result.is_none());
    }

    #[test]
    fn test_format_onset_output() {
        let test_data = vec![
            (HanziOnset::J, 150),
            (HanziOnset::M, 120),
            (HanziOnset::Zh, 90),
            (HanziOnset::None, 80),
        ];

        let output = format_onset_output(&test_data);

        assert_eq!(output.len(), 4);
        assert_eq!(output[0], "j: 150");
        assert_eq!(output[1], "m: 120");
        assert_eq!(output[2], "zh: 90");
        assert_eq!(output[3], "none: 80");
    }

    #[test]
    fn test_format_onset_output_empty() {
        let test_data = vec![];
        let output = format_onset_output(&test_data);

        assert!(output.is_empty());
    }

    #[test]
    fn test_format_onset_output_with_group_by_onset() {
        let records = create_test_records();

        if let Some(onset_counts) = group_by_onset(&records) {
            let formatted = format_onset_output(&onset_counts);

            // Should have formatted output for each onset
            assert!(!formatted.is_empty());

            // Each line should contain a colon separator
            for line in &formatted {
                assert!(line.contains(":"), "Each line should contain ':'");
                let parts: Vec<&str> = line.split(':').collect();
                assert_eq!(parts.len(), 2, "Each line should have exactly one ':'");

                // The second part should be a number
                let count_str = parts[1].trim();
                assert!(
                    count_str.parse::<u32>().is_ok(),
                    "Count should be a valid number"
                );
            }
        } else {
            panic!("group_by_onset should return Some for non-empty records");
        }
    }
}
