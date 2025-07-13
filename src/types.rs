//! # Type Definitions Module
//!
//! This module contains the core type definitions for the Hanzi analysis library,
//! including enumerations for onset and rime sounds in Mandarin Chinese pinyin,
//! and the main data structure for representing Chinese characters.
//!
//! ## Types
//!
//! - [`HanziRecord`]: Represents a single Chinese character with all its linguistic properties
//! - [`HanziOnset`]: Enumeration of pinyin onset sounds (initial consonants)
//! - [`HanziRime`]: Enumeration of pinyin rime sounds (vowels and final consonants)

use std::str::FromStr;

/// Enumeration of Hanzi onset sounds (initial consonants)
///
/// This enum represents all possible onset sounds in Mandarin Chinese pinyin.
/// Onsets are the initial consonant sounds that begin a syllable. Some syllables
/// may have no onset (represented by `None`).
///
/// # Examples of onsets
///
/// - `B`: as in "bā" (八)
/// - `Zh`: as in "zhōng" (中)
/// - `None`: as in "ā" (啊) - syllables starting with vowels
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

impl HanziOnset {
    /// Returns the kebab-case string representation of the onset
    ///
    /// This method converts the onset to a kebab-case string format,
    /// where compound onsets (like `Zh`, `Ch`, `Sh`) are represented
    /// in lowercase with no separators, and `None` is represented as "none".
    ///
    /// # Returns
    ///
    /// A string slice representing the onset in kebab-case format
    ///
    /// # Examples
    ///
    /// ```
    /// use study_rust_hanzi::HanziOnset;
    ///
    /// assert_eq!(HanziOnset::B.as_str(), "b");
    /// assert_eq!(HanziOnset::Zh.as_str(), "zh");
    /// assert_eq!(HanziOnset::None.as_str(), "none");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            HanziOnset::B => "b",
            HanziOnset::P => "p",
            HanziOnset::M => "m",
            HanziOnset::F => "f",
            HanziOnset::D => "d",
            HanziOnset::T => "t",
            HanziOnset::N => "n",
            HanziOnset::Z => "z",
            HanziOnset::C => "c",
            HanziOnset::S => "s",
            HanziOnset::L => "l",
            HanziOnset::Zh => "zh",
            HanziOnset::Ch => "ch",
            HanziOnset::Sh => "sh",
            HanziOnset::R => "r",
            HanziOnset::J => "j",
            HanziOnset::Q => "q",
            HanziOnset::X => "x",
            HanziOnset::G => "g",
            HanziOnset::K => "k",
            HanziOnset::H => "h",
            HanziOnset::Y => "y",
            HanziOnset::W => "w",
            HanziOnset::None => "none",
        }
    }
}

impl FromStr for HanziOnset {
    type Err = String;

    /// Parses a string into a HanziOnset
    ///
    /// This method converts a string representation back into a HanziOnset variant.
    /// It accepts both the exact case-sensitive variants and their lowercase equivalents.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse
    ///
    /// # Returns
    ///
    /// * `Ok(HanziOnset)` - If the string matches a valid onset
    /// * `Err(String)` - If the string doesn't match any known onset
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use study_rust_hanzi::HanziOnset;
    ///
    /// assert_eq!(HanziOnset::from_str("b"), Ok(HanziOnset::B));
    /// assert_eq!(HanziOnset::from_str("zh"), Ok(HanziOnset::Zh));
    /// assert_eq!(HanziOnset::from_str("none"), Ok(HanziOnset::None));
    /// assert!(HanziOnset::from_str("invalid").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "b" => Ok(HanziOnset::B),
            "p" => Ok(HanziOnset::P),
            "m" => Ok(HanziOnset::M),
            "f" => Ok(HanziOnset::F),
            "d" => Ok(HanziOnset::D),
            "t" => Ok(HanziOnset::T),
            "n" => Ok(HanziOnset::N),
            "z" => Ok(HanziOnset::Z),
            "c" => Ok(HanziOnset::C),
            "s" => Ok(HanziOnset::S),
            "l" => Ok(HanziOnset::L),
            "zh" => Ok(HanziOnset::Zh),
            "ch" => Ok(HanziOnset::Ch),
            "sh" => Ok(HanziOnset::Sh),
            "r" => Ok(HanziOnset::R),
            "j" => Ok(HanziOnset::J),
            "q" => Ok(HanziOnset::Q),
            "x" => Ok(HanziOnset::X),
            "g" => Ok(HanziOnset::G),
            "k" => Ok(HanziOnset::K),
            "h" => Ok(HanziOnset::H),
            "y" => Ok(HanziOnset::Y),
            "w" => Ok(HanziOnset::W),
            "none" => Ok(HanziOnset::None),
            _ => Err(format!("Invalid onset: '{s}'")),
        }
    }
}

/// Enumeration of Hanzi rime sounds (vowels and final consonants)
///
/// This enum represents all possible rime sounds in Mandarin Chinese pinyin.
/// Rimes are the vowel and optional final consonant parts of a syllable that
/// follow the onset. Every syllable must have a rime.
///
/// # Examples of rimes
///
/// - `A`: as in "mā" (妈) - simple vowel
/// - `Ang`: as in "tāng" (汤) - vowel + nasal consonant
/// - `Iang`: as in "liáng" (良) - complex vowel + nasal
/// - `V`: represents "ü" as in "nǚ" (女)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HanziRime {
    E,
    A,
    O,
    Ei,
    Ai,
    Ou,
    Ao,
    En,
    An,
    Ong,
    Eng,
    Ang,
    Er,
    I,
    Ie,
    Ia,
    Iu,
    Iao,
    In,
    Ian,
    Iong,
    Ing,
    Iang,
    U,
    Uo,
    Ua,
    Ui,
    Uai,
    Un,
    Uan,
    Uang,
    V,
    Ve,
    Ue,
    None,
}

impl HanziRime {
    /// Returns the string representation of the rime
    ///
    /// This method converts the rime to its pinyin string representation.
    /// The special case `V` is converted to "ü" and `Ve` to "üe" to represent
    /// the umlaut vowel sounds in Mandarin pinyin.
    ///
    /// # Returns
    ///
    /// A string slice representing the rime in pinyin format
    ///
    /// # Examples
    ///
    /// ```
    /// use study_rust_hanzi::HanziRime;
    ///     /// assert_eq!(HanziRime::A.as_str(), "a");
     /// assert_eq!(HanziRime::O.as_str(), "o");
     /// assert_eq!(HanziRime::Ang.as_str(), "ang");
     /// assert_eq!(HanziRime::Er.as_str(), "er");
     /// assert_eq!(HanziRime::V.as_str(), "ü");
    /// assert_eq!(HanziRime::Ve.as_str(), "üe");
    /// assert_eq!(HanziRime::Ue.as_str(), "ue");
    /// assert_eq!(HanziRime::None.as_str(), "none");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            HanziRime::E => "e",
            HanziRime::A => "a",
            HanziRime::O => "o",
            HanziRime::Ei => "ei",
            HanziRime::Ai => "ai",
            HanziRime::Ou => "ou",
            HanziRime::Ao => "ao",
            HanziRime::En => "en",
            HanziRime::An => "an",
            HanziRime::Ong => "ong",
            HanziRime::Eng => "eng",
            HanziRime::Ang => "ang",
            HanziRime::Er => "er",
            HanziRime::I => "i",
            HanziRime::Ie => "ie",
            HanziRime::Ia => "ia",
            HanziRime::Iu => "iu",
            HanziRime::Iao => "iao",
            HanziRime::In => "in",
            HanziRime::Ian => "ian",
            HanziRime::Iong => "iong",
            HanziRime::Ing => "ing",
            HanziRime::Iang => "iang",
            HanziRime::U => "u",
            HanziRime::Uo => "uo",
            HanziRime::Ua => "ua",
            HanziRime::Ui => "ui",
            HanziRime::Uai => "uai",
            HanziRime::Un => "un",
            HanziRime::Uan => "uan",
            HanziRime::Uang => "uang",
            HanziRime::V => "ü",
            HanziRime::Ve => "üe",
            HanziRime::Ue => "ue",
            HanziRime::None => "none",
        }
    }
}

impl FromStr for HanziRime {
    type Err = String;

    /// Parses a string into a HanziRime
    ///
    /// This method converts a string representation back into a HanziRime variant.
    /// It accepts both the exact pinyin representation and handles special cases
    /// like "ü" and "üe" for the V and Ve variants.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse
    ///
    /// # Returns
    ///
    /// * `Ok(HanziRime)` - If the string matches a valid rime
    /// * `Err(String)` - If the string doesn't match any known rime
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use study_rust_hanzi::HanziRime;
    ///     /// assert_eq!(HanziRime::from_str("a"), Ok(HanziRime::A));
     /// assert_eq!(HanziRime::from_str("o"), Ok(HanziRime::O));
     /// assert_eq!(HanziRime::from_str("ang"), Ok(HanziRime::Ang));
     /// assert_eq!(HanziRime::from_str("er"), Ok(HanziRime::Er));
     /// assert_eq!(HanziRime::from_str("ü"), Ok(HanziRime::V));
    /// assert_eq!(HanziRime::from_str("üe"), Ok(HanziRime::Ve));
    /// assert_eq!(HanziRime::from_str("ue"), Ok(HanziRime::Ue));
    /// assert_eq!(HanziRime::from_str("none"), Ok(HanziRime::None));
    /// assert!(HanziRime::from_str("invalid").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "e" => Ok(HanziRime::E),
            "a" => Ok(HanziRime::A),
            "o" => Ok(HanziRime::O),
            "ei" => Ok(HanziRime::Ei),
            "ai" => Ok(HanziRime::Ai),
            "ou" => Ok(HanziRime::Ou),
            "ao" => Ok(HanziRime::Ao),
            "en" => Ok(HanziRime::En),
            "an" => Ok(HanziRime::An),
            "ong" => Ok(HanziRime::Ong),
            "eng" => Ok(HanziRime::Eng),
            "ang" => Ok(HanziRime::Ang),
            "er" => Ok(HanziRime::Er),
            "i" => Ok(HanziRime::I),
            "ie" => Ok(HanziRime::Ie),
            "ia" => Ok(HanziRime::Ia),
            "iu" => Ok(HanziRime::Iu),
            "iao" => Ok(HanziRime::Iao),
            "in" => Ok(HanziRime::In),
            "ian" => Ok(HanziRime::Ian),
            "iong" => Ok(HanziRime::Iong),
            "ing" => Ok(HanziRime::Ing),
            "iang" => Ok(HanziRime::Iang),
            "u" => Ok(HanziRime::U),
            "uo" => Ok(HanziRime::Uo),
            "ua" => Ok(HanziRime::Ua),
            "ui" => Ok(HanziRime::Ui),
            "uai" => Ok(HanziRime::Uai),
            "un" => Ok(HanziRime::Un),
            "uan" => Ok(HanziRime::Uan),
            "uang" => Ok(HanziRime::Uang),
            "ü" => Ok(HanziRime::V),
            "üe" => Ok(HanziRime::Ve),
            "ue" => Ok(HanziRime::Ue),
            "v" => Ok(HanziRime::V),   // Alternative representation for ü
            "ve" => Ok(HanziRime::Ve), // Alternative representation for üe
            "none" => Ok(HanziRime::None),
            _ => Err(format!("Invalid rime: '{s}'")),
        }
    }
}

/// Represents a single Chinese character with all its linguistic and frequency data
///
/// This structure contains comprehensive information about a Chinese character,
/// including both simplified and traditional forms, pinyin pronunciation data,
/// frequency information, and phonetic analysis (onset/rime breakdown).
///
/// # Fields
///
/// * `frequency` - Frequency rank of the character (lower numbers = more common)
/// * `simplified` - Simplified Chinese character form
/// * `traditional` - Traditional Chinese character form  
/// * `pinyin` - Complete pinyin with tone marks (e.g., "mā")
/// * `pinyin_without_tone` - Pinyin without tone marks (e.g., "ma")
/// * `tone` - Tone number (1-4 for tones, 5 for neutral tone)
/// * `onset` - Initial consonant sound classification
/// * `rime` - Vowel and final consonant sound classification
#[derive(Debug, Clone)]
pub struct HanziRecord {
    pub frequency: u32,
    pub simplified: String,
    pub traditional: String,
    pub pinyin: String,
    pub pinyin_without_tone: String,
    pub tone: u32,
    pub onset: HanziOnset,
    pub rime: HanziRime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanzi_onset_as_str() {
        // Test single character onsets
        assert_eq!(HanziOnset::B.as_str(), "b");
        assert_eq!(HanziOnset::P.as_str(), "p");
        assert_eq!(HanziOnset::M.as_str(), "m");
        assert_eq!(HanziOnset::F.as_str(), "f");
        assert_eq!(HanziOnset::D.as_str(), "d");
        assert_eq!(HanziOnset::T.as_str(), "t");
        assert_eq!(HanziOnset::N.as_str(), "n");
        assert_eq!(HanziOnset::Z.as_str(), "z");
        assert_eq!(HanziOnset::C.as_str(), "c");
        assert_eq!(HanziOnset::S.as_str(), "s");
        assert_eq!(HanziOnset::L.as_str(), "l");
        assert_eq!(HanziOnset::R.as_str(), "r");
        assert_eq!(HanziOnset::J.as_str(), "j");
        assert_eq!(HanziOnset::Q.as_str(), "q");
        assert_eq!(HanziOnset::X.as_str(), "x");
        assert_eq!(HanziOnset::G.as_str(), "g");
        assert_eq!(HanziOnset::K.as_str(), "k");
        assert_eq!(HanziOnset::H.as_str(), "h");
        assert_eq!(HanziOnset::Y.as_str(), "y");
        assert_eq!(HanziOnset::W.as_str(), "w");

        // Test compound onsets (kebab-case format)
        assert_eq!(HanziOnset::Zh.as_str(), "zh");
        assert_eq!(HanziOnset::Ch.as_str(), "ch");
        assert_eq!(HanziOnset::Sh.as_str(), "sh");

        // Test None case
        assert_eq!(HanziOnset::None.as_str(), "none");
    }

    #[test]
    fn test_onset_from_str() {
        // Test valid single-character onsets
        assert_eq!(HanziOnset::from_str("b"), Ok(HanziOnset::B));
        assert_eq!(HanziOnset::from_str("B"), Ok(HanziOnset::B));
        assert_eq!(HanziOnset::from_str("p"), Ok(HanziOnset::P));
        assert_eq!(HanziOnset::from_str("m"), Ok(HanziOnset::M));

        // Test valid multi-character onsets
        assert_eq!(HanziOnset::from_str("zh"), Ok(HanziOnset::Zh));
        assert_eq!(HanziOnset::from_str("Zh"), Ok(HanziOnset::Zh));
        assert_eq!(HanziOnset::from_str("ZH"), Ok(HanziOnset::Zh));
        assert_eq!(HanziOnset::from_str("ch"), Ok(HanziOnset::Ch));
        assert_eq!(HanziOnset::from_str("sh"), Ok(HanziOnset::Sh));

        // Test special case
        assert_eq!(HanziOnset::from_str("none"), Ok(HanziOnset::None));
        assert_eq!(HanziOnset::from_str("None"), Ok(HanziOnset::None));
        assert_eq!(HanziOnset::from_str("NONE"), Ok(HanziOnset::None));

        // Test invalid inputs
        assert!(HanziOnset::from_str("invalid").is_err());
        assert!(HanziOnset::from_str("").is_err());
        assert!(HanziOnset::from_str("zz").is_err());

        // Test error message
        let result = HanziOnset::from_str("invalid");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid onset: 'invalid'");
    }

    #[test]
    fn test_hanzi_rime_as_str() {
        // Test simple vowel rimes
        assert_eq!(HanziRime::E.as_str(), "e");
        assert_eq!(HanziRime::A.as_str(), "a");
        assert_eq!(HanziRime::O.as_str(), "o");
        assert_eq!(HanziRime::I.as_str(), "i");
        assert_eq!(HanziRime::U.as_str(), "u");

        // Test compound vowel rimes
        assert_eq!(HanziRime::Ei.as_str(), "ei");
        assert_eq!(HanziRime::Ai.as_str(), "ai");
        assert_eq!(HanziRime::Ou.as_str(), "ou");
        assert_eq!(HanziRime::Ao.as_str(), "ao");

        // Test nasal ending rimes
        assert_eq!(HanziRime::En.as_str(), "en");
        assert_eq!(HanziRime::An.as_str(), "an");
        assert_eq!(HanziRime::Eng.as_str(), "eng");
        assert_eq!(HanziRime::Ang.as_str(), "ang");
        assert_eq!(HanziRime::Ong.as_str(), "ong");
        assert_eq!(HanziRime::Er.as_str(), "er");

        // Test complex compound rimes
        assert_eq!(HanziRime::Ie.as_str(), "ie");
        assert_eq!(HanziRime::Ia.as_str(), "ia");
        assert_eq!(HanziRime::Iu.as_str(), "iu");
        assert_eq!(HanziRime::Iao.as_str(), "iao");
        assert_eq!(HanziRime::In.as_str(), "in");
        assert_eq!(HanziRime::Ian.as_str(), "ian");
        assert_eq!(HanziRime::Iong.as_str(), "iong");
        assert_eq!(HanziRime::Ing.as_str(), "ing");
        assert_eq!(HanziRime::Iang.as_str(), "iang");

        // Test u-compound rimes
        assert_eq!(HanziRime::Uo.as_str(), "uo");
        assert_eq!(HanziRime::Ua.as_str(), "ua");
        assert_eq!(HanziRime::Ui.as_str(), "ui");
        assert_eq!(HanziRime::Uai.as_str(), "uai");
        assert_eq!(HanziRime::Un.as_str(), "un");
        assert_eq!(HanziRime::Uan.as_str(), "uan");
        assert_eq!(HanziRime::Uang.as_str(), "uang");

        // Test special ü rimes
        assert_eq!(HanziRime::V.as_str(), "ü");
        assert_eq!(HanziRime::Ve.as_str(), "üe");
        assert_eq!(HanziRime::Ue.as_str(), "ue");

        // Test None case
        assert_eq!(HanziRime::None.as_str(), "none");
    }

    #[test]
    fn test_rime_from_str() {
        // Test valid simple vowel rimes
        assert_eq!(HanziRime::from_str("e"), Ok(HanziRime::E));
        assert_eq!(HanziRime::from_str("a"), Ok(HanziRime::A));
        assert_eq!(HanziRime::from_str("o"), Ok(HanziRime::O));
        assert_eq!(HanziRime::from_str("i"), Ok(HanziRime::I));
        assert_eq!(HanziRime::from_str("u"), Ok(HanziRime::U));

        // Test valid compound vowel rimes
        assert_eq!(HanziRime::from_str("ei"), Ok(HanziRime::Ei));
        assert_eq!(HanziRime::from_str("ai"), Ok(HanziRime::Ai));
        assert_eq!(HanziRime::from_str("ou"), Ok(HanziRime::Ou));
        assert_eq!(HanziRime::from_str("ao"), Ok(HanziRime::Ao));

        // Test valid nasal ending rimes
        assert_eq!(HanziRime::from_str("en"), Ok(HanziRime::En));
        assert_eq!(HanziRime::from_str("an"), Ok(HanziRime::An));
        assert_eq!(HanziRime::from_str("eng"), Ok(HanziRime::Eng));
        assert_eq!(HanziRime::from_str("ang"), Ok(HanziRime::Ang));
        assert_eq!(HanziRime::from_str("ong"), Ok(HanziRime::Ong));
        assert_eq!(HanziRime::from_str("er"), Ok(HanziRime::Er));

        // Test valid complex compound rimes
        assert_eq!(HanziRime::from_str("ie"), Ok(HanziRime::Ie));
        assert_eq!(HanziRime::from_str("ia"), Ok(HanziRime::Ia));
        assert_eq!(HanziRime::from_str("iu"), Ok(HanziRime::Iu));
        assert_eq!(HanziRime::from_str("iao"), Ok(HanziRime::Iao));
        assert_eq!(HanziRime::from_str("in"), Ok(HanziRime::In));
        assert_eq!(HanziRime::from_str("ian"), Ok(HanziRime::Ian));
        assert_eq!(HanziRime::from_str("iong"), Ok(HanziRime::Iong));
        assert_eq!(HanziRime::from_str("ing"), Ok(HanziRime::Ing));
        assert_eq!(HanziRime::from_str("iang"), Ok(HanziRime::Iang));

        // Test valid u-compound rimes
        assert_eq!(HanziRime::from_str("uo"), Ok(HanziRime::Uo));
        assert_eq!(HanziRime::from_str("ua"), Ok(HanziRime::Ua));
        assert_eq!(HanziRime::from_str("ui"), Ok(HanziRime::Ui));
        assert_eq!(HanziRime::from_str("uai"), Ok(HanziRime::Uai));
        assert_eq!(HanziRime::from_str("un"), Ok(HanziRime::Un));
        assert_eq!(HanziRime::from_str("uan"), Ok(HanziRime::Uan));
        assert_eq!(HanziRime::from_str("uang"), Ok(HanziRime::Uang));

        // Test special ü rimes - both ü and v representations
        assert_eq!(HanziRime::from_str("ü"), Ok(HanziRime::V));
        assert_eq!(HanziRime::from_str("v"), Ok(HanziRime::V));
        assert_eq!(HanziRime::from_str("üe"), Ok(HanziRime::Ve));
        assert_eq!(HanziRime::from_str("ve"), Ok(HanziRime::Ve));
        assert_eq!(HanziRime::from_str("ue"), Ok(HanziRime::Ue));

        // Test case insensitivity
        assert_eq!(HanziRime::from_str("ANG"), Ok(HanziRime::Ang));
        assert_eq!(HanziRime::from_str("Iang"), Ok(HanziRime::Iang));

        // Test special case
        assert_eq!(HanziRime::from_str("none"), Ok(HanziRime::None));
        assert_eq!(HanziRime::from_str("None"), Ok(HanziRime::None));
        assert_eq!(HanziRime::from_str("NONE"), Ok(HanziRime::None));

        // Test invalid inputs
        assert!(HanziRime::from_str("invalid").is_err());
        assert!(HanziRime::from_str("").is_err());
        assert!(HanziRime::from_str("xyz").is_err());

        // Test error message
        let result = HanziRime::from_str("invalid");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid rime: 'invalid'");
    }
}
