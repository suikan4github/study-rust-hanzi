//! # Hanzi Analysis Library
//!
//! This library provides data structures and functions for analyzing Chinese characters (Hanzi)
//! based on their pinyin pronunciation, including onset-rime analysis and various grouping
//! and formatting operations.
//!
//! ## Core Data Structures
//!
//! - [`HanziRecord`]: Represents a single Chinese character with all its linguistic properties
//! - [`HanziOnset`]: Enumeration of pinyin onset sounds (initial consonants)
//! - [`HanziRime`]: Enumeration of pinyin rime sounds (vowels and final consonants)
//!
//! ## Main Functions
//!
//! - [`read_hanzi_file`]: Reads character data from TSV files
//! - [`group_by_pinyin`]: Groups characters by pinyin pronunciation
//! - [`group_by_tone`]: Groups characters by specific pinyin and tone
//! - [`format_pinyin_output`]: Formats pinyin grouping results for display
//! - [`format_tone_output`]: Formats tone grouping results for display
//!
//! ## Linguistic Analysis
//!
//! - [`set_hanzi_onsets`]: Analyzes and sets onset information for characters
//! - [`set_hanzi_rime`]: Analyzes and sets rime information for characters

pub mod analysis;
pub mod grouping;
pub mod io;
pub mod types;

// Re-export the types module for public API
pub use crate::types::{HanziOnset, HanziRecord, HanziRime};

// Re-export the io module functions for backward compatibility
pub use crate::io::read_hanzi_file;

// Re-export the grouping module functions for backward compatibility
pub use crate::grouping::{
    format_onset_output, format_pinyin_output, format_tone_output, group_by_onset, group_by_pinyin,
    group_by_tone,
};

// Re-export the analysis module functions for backward compatibility
pub use crate::analysis::{set_hanzi_onsets, set_hanzi_rime};

#[cfg(test)]
mod tests {
    use super::*;

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
            onset: HanziOnset::N,
            rime: HanziRime::V,
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
