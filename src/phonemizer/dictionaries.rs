use std::collections::{HashMap, HashSet};
use crate::phonemizer::Result;

/// A unified structure containing all dictionary-related data
#[derive(Debug, Clone)]
pub struct DictionarySet {
    /// Main general dictionary word -> phonemes
    pub general: HashMap<String, String>,
    /// Verbs dictionary
    pub verbs: HashMap<String, String>,
    /// Nouns dictionary  
    pub nouns: HashMap<String, String>,
    /// Words that should be at start of utterance
    pub at_start: HashMap<String, String>,
    /// Words that should be at end of utterance
    pub at_end: HashMap<String, String>,
    /// Capitalized word pronunciations
    pub capital: HashMap<String, String>,
    /// One-syllable word dictionary
    pub onlys_bare: HashMap<String, String>,
    /// Phrase definitions (two-word phrases)
    pub phrases: HashMap<String, String>,
    /// Phrase split definitions (can split pronunciation)
    pub phrase_splits: HashMap<String, (String, String)>,
    
    // Flag sets for special handling
    pub past_form_words: HashSet<String>,
    pub noun_form_words: HashSet<String>,
    pub verb_form_words: HashSet<String>,
    pub unstressed_words: HashSet<String>,
    pub unstressed_end_words: HashSet<String>,
    pub abbreviation_words: HashSet<String>,
    pub only_ones_words: HashSet<String>,
    pub only_words: HashSet<String>,
    pub onlys_words: HashSet<String>,
    pub compound_prefixes_words: HashSet<String>,
    pub comma_stress_end2_words: HashSet<String>,
    pub u2_stress_end2_words: HashSet<String>,
    pub u_plus_secondary_words: HashSet<String>,
    pub keepu2_phrase_keys: HashSet<String>,
    pub noun_form_stress: HashSet<String>,
    pub verb_flag_words: HashSet<String>,
    
    // Special mappings
    pub stress_positions: HashMap<String, i32>, // word -> stress position (1-6)
    pub word_alt_flags: HashMap<String, i32>,   // word -> alt form flags
    pub compound_prefixes: Vec<(String, String)>, // (prefix, phonemes) sorted by length
}

impl DictionarySet {
    pub fn new() -> Self {
        Self {
            general: HashMap::new(),
            verbs: HashMap::new(),
            nouns: HashMap::new(),
            at_start: HashMap::new(),
            at_end: HashMap::new(),
            capital: HashMap::new(),
            onlys_bare: HashMap::new(),
            phrases: HashMap::new(),
            phrase_splits: HashMap::new(),
            past_form_words: HashSet::new(),
            noun_form_words: HashSet::new(),
            verb_form_words: HashSet::new(),
            unstressed_words: HashSet::new(),
            unstressed_end_words: HashSet::new(),
            abbreviation_words: HashSet::new(),
            only_ones_words: HashSet::new(),
            only_words: HashSet::new(),
            onlys_words: HashSet::new(),
            compound_prefixes_words: HashSet::new(),
            comma_stress_end2_words: HashSet::new(),
            u2_stress_end2_words: HashSet::new(),
            u_plus_secondary_words: HashSet::new(),
            keepu2_phrase_keys: HashSet::new(),
            noun_form_stress: HashSet::new(),
            verb_flag_words: HashSet::new(),
            stress_positions: HashMap::new(),
            word_alt_flags: HashMap::new(),
            compound_prefixes: Vec::new(),
        }
    }
    
    /// Clear all dictionaries
    pub fn clear(&mut self) {
        self.general.clear();
        self.verbs.clear();
        self.nouns.clear();
        self.at_start.clear();
        self.at_end.clear();
        self.capital.clear();
        self.onlys_bare.clear();
        self.phrases.clear();
        self.phrase_splits.clear();
        
        self.past_form_words.clear();
        self.noun_form_words.clear();
        self.verb_form_words.clear();
        self.unstressed_words.clear();
        self.unstressed_end_words.clear();
        self.abbreviation_words.clear();
        self.only_ones_words.clear();
        self.only_words.clear();
        self.onlys_words.clear();
        self.compound_prefixes_words.clear();
        self.comma_stress_end2_words.clear();
        self.u2_stress_end2_words.clear();
        self.u_plus_secondary_words.clear();
        self.keepu2_phrase_keys.clear();
        self.noun_form_stress.clear();
        self.verb_flag_words.clear();
        
        self.stress_positions.clear();
        self.word_alt_flags.clear();
        self.compound_prefixes.clear();
    }
    
    /// Get pronunciation from any dictionary (general lookup)
    pub fn lookup(&self, word: &str, context: DictionaryContext) -> Option<&String> {
        match context {
            DictionaryContext::General => self.general.get(word),
            DictionaryContext::Verb => self.verbs.get(word),
            DictionaryContext::Noun => self.nouns.get(word),
            DictionaryContext::AtStart => self.at_start.get(word),
            DictionaryContext::AtEnd => self.at_end.get(word),
            DictionaryContext::Capital => self.capital.get(word),
        }
    }
    
    /// Sort compound prefixes by length (longest first)
    pub fn finalize(&mut self) {
        self.compound_prefixes.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        // Remove "made" from unstressed words (special case)
        self.unstressed_words.remove("made");
    }
}

impl Default for DictionarySet {
    fn default() -> Self {
        Self::new()
    }
}

/// Context for dictionary lookups
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictionaryContext {
    General,
    Verb,
    Noun,
    AtStart,
    AtEnd,
    Capital,
}

/// Flags associated with dictionary words
#[derive(Debug, Clone, Copy)]
pub struct WordFlags {
    pub is_past_form: bool,
    pub is_noun_form: bool,
    pub is_verb_form: bool,
    pub is_unstressed: bool,
    pub is_abbreviation: bool,
    pub is_capital: bool,
    pub is_at_start: bool,
    pub is_at_end: bool,
}

impl WordFlags {
    pub fn new() -> Self {
        Self {
            is_past_form: false,
            is_noun_form: false,
            is_verb_form: false,
            is_unstressed: false,
            is_abbreviation: false,
            is_capital: false,
            is_at_start: false,
            is_at_end: false,
        }
    }
}

impl Default for WordFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Stress-related flags for words
#[derive(Debug, Clone, Copy, Default)]
pub struct StressFlags {
    pub verb_form: bool,
    pub irregular_inflection: bool,
    pub vowel_change: bool,
    pub emphasis: bool,
    pub doubling: bool,
    pub quiet: bool,
    pub pausal: bool,
}

impl StressFlags {
    pub fn from_bits(bits: i32) -> Self {
        Self {
            verb_form: (bits & 0x800) != 0,
            irregular_inflection: (bits & 0x200) != 0,
            vowel_change: (bits & 0x100) != 0,
            emphasis: (bits & 0x400) != 0,
            doubling: (bits & 0x1000) != 0,
            quiet: (bits & 0x4000) != 0,
            pausal: (bits & 0x80000) != 0,
        }
    }

    pub fn to_bits(&self) -> i32 {
        let mut bits = 0i32;
        if self.verb_form {
            bits |= 0x800;
        }
        if self.irregular_inflection {
            bits |= 0x200;
        }
        if self.vowel_change {
            bits |= 0x100;
        }
        if self.emphasis {
            bits |= 0x400;
        }
        if self.doubling {
            bits |= 0x1000;
        }
        if self.quiet {
            bits |= 0x4000;
        }
        if self.pausal {
            bits |= 0x80000;
        }
        bits
    }
}

/// Trait for loading dictionaries from various sources
pub trait DictionaryLoader {
    /// Load dictionary from a file path
    fn load_from_path(&mut self, path: &str) -> Result<()>;
    
    /// Load dictionary from bytes
    fn load_from_bytes(&mut self, bytes: &[u8]) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_set_creation() {
        let dict = DictionarySet::new();
        assert!(dict.general.is_empty());
        assert!(dict.verbs.is_empty());
        assert!(dict.unstressed_words.is_empty());
    }

    #[test]
    fn test_stress_flags_conversion() {
        let flags = StressFlags {
            verb_form: true,
            vowel_change: true,
            ..Default::default()
        };
        let bits = flags.to_bits();
        let flags2 = StressFlags::from_bits(bits);
        assert_eq!(flags.verb_form, flags2.verb_form);
        assert_eq!(flags.vowel_change, flags2.vowel_change);
    }

    #[test]
    fn test_word_flags_creation() {
        let flags = WordFlags::new();
        assert!(!flags.is_unstressed);
        assert!(!flags.is_abbreviation);
    }
}
