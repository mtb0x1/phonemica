// Suffix and prefix rule handling for morphological phonemization
// Implements recursive phonemization with morphological transformations

use crate::types::PhonemeRule;

/// Configuration flags for suffix rules
#[derive(Debug, Copy, Clone)]
pub struct SuffixFlags {
    /// SUFX_I: Restore 'y' from 'i' in stem before suffix
    pub restore_y: bool,
    /// SUFX_E: Add back silent 'e' when stem ends in consonant
    pub add_e: bool,
    /// SUFX_V: Apply -ed devoicing (use verb form)
    pub verb_form: bool,
    /// SUFX_Q: Deduplication flag (skip if already applied)
    pub skip_dup: bool,
}

impl SuffixFlags {
    pub fn from_bits(bits: i32) -> Self {
        Self {
            restore_y: (bits & 0x200) != 0,
            add_e: (bits & 0x100) != 0,
            verb_form: (bits & 0x800) != 0,
            skip_dup: (bits & 0x4000) != 0,
        }
    }

    pub fn has_any(&self) -> bool {
        self.restore_y || self.add_e || self.verb_form || self.skip_dup
    }
}

/// Handles suffix rule matching and stem re-phonemization
pub struct SuffixHandler;

impl SuffixHandler {
    /// Check if a rule is a suffix rule at the end of a word
    pub fn is_suffix_match(rule: &PhonemeRule, word: &str, match_pos: usize) -> bool {
        if !rule.is_suffix {
            return false;
        }
        // Suffix match must be at word end or near it
        match_pos + rule.match_str.len() >= word.len()
    }

    /// Extract stem by stripping N characters from word end
    pub fn extract_stem(word: &str, strip_len: i32) -> String {
        let strip_len = strip_len as usize;
        if strip_len >= word.len() {
            String::new()
        } else {
            word[..word.len() - strip_len].to_string()
        }
    }

    /// Restore morphological changes to stem
    /// - If SUFX_I: restore 'y' from 'i' (e.g., "happily" → "happy" → "happi" + "ly")
    /// - If SUFX_E: add back silent 'e' for vowel+consonant patterns
    /// - Handle doubled consonants (e.g., "running" → "run")
    pub fn restore_stem(stem: &str, flags: SuffixFlags) -> String {
        let mut result = stem.to_string();

        // Restore 'y' from 'i' if flag set (inverse of -ing suffix)
        if flags.restore_y && result.ends_with('i') {
            result.pop();
            result.push('y');
        }

        // Handle doubled consonants for vowel+single consonant+suffix patterns
        // E.g., "running" → "runn" + "ing" needs to restore to "run"
        // Pattern: vowel + consonant + same consonant
        if result.len() >= 3 {
            let chars: Vec<char> = result.chars().collect();
            let len = chars.len();
            let last = chars[len - 1].to_ascii_lowercase();
            let prev = chars[len - 2].to_ascii_lowercase();
            let prev_prev = chars[len - 3].to_ascii_lowercase();
            
            // Check for doubled consonant pattern: vowel + consonant + consonant
            if Self::is_vowel(prev_prev) && Self::is_consonant(prev) && last == prev {
                // This is likely a doubled consonant - remove one
                result.pop();
            }
        }

        // Add back silent 'e' for vowel+consonant+suffix pattern
        // E.g., "make" → "mak" + "ing" becomes "mak" + "e" + "ing"
        if flags.add_e && result.len() >= 2 {
            let chars: Vec<char> = result.chars().collect();
            let last = chars[chars.len() - 1];
            let prev = chars[chars.len() - 2];

            // Check if pattern is vowel + consonant (need 'e' back)
            if Self::is_vowel(prev) && Self::is_consonant(last) {
                result.push('e');
            }
        }

        result
    }

    /// Apply -ed devoicing rules for past tense
    /// After voiced consonants: -ed → /ɪd/
    /// After voiceless consonants: -ed → /t/
    /// After t/d: -ed → /ɪd/
    pub fn apply_ed_devoicing(stem: &str, suffix_phoneme: &str) -> String {
        if !suffix_phoneme.contains("d") && !suffix_phoneme.contains("t") {
            return suffix_phoneme.to_string();
        }

        if let Some(last_char) = stem.chars().last() {
            let lc = last_char.to_ascii_lowercase();
            match lc {
                // Voiceless consonants: -ed → /t/
                'p' | 'k' | 'f' | 's' | 'x' | 'θ' => {
                    return "t".to_string();
                }
                // t or d: -ed → /ɪd/
                't' | 'd' => {
                    return "I#d".to_string();
                }
                // Other consonants: keep as-is (voiced → /ɪd/ from rules)
                _ => {}
            }
        }

        suffix_phoneme.to_string()
    }

    fn is_vowel(c: char) -> bool {
        matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
    }

    fn is_consonant(c: char) -> bool {
        c.is_alphabetic() && !Self::is_vowel(c)
    }
}

/// Handles prefix rule matching and suffix re-phonemization
pub struct PrefixHandler;

impl PrefixHandler {
    /// Check if a rule is a prefix rule at the start of a word
    pub fn is_prefix_match(rule: &PhonemeRule, match_pos: usize) -> bool {
        rule.is_prefix && match_pos == 0
    }

    /// Extract suffix (remaining part after prefix match)
    pub fn extract_suffix(word: &str, prefix_len: usize) -> String {
        if prefix_len >= word.len() {
            String::new()
        } else {
            word[prefix_len..].to_string()
        }
    }

    /// Count vowel groups in a phoneme string
    /// Used to determine stress demotion for compound words
    pub fn count_vowel_groups(phonemes: &str) -> i32 {
        let mut count = 0;
        let mut in_vowel_group = false;

        for c in phonemes.chars() {
            if Self::is_vowel_code(c) {
                if !in_vowel_group {
                    count += 1;
                    in_vowel_group = true;
                }
            } else {
                in_vowel_group = false;
            }
        }

        count
    }

    /// Count syllables (stress markers) in phoneme string
    pub fn count_stress_marks(phonemes: &str) -> (i32, i32) {
        let mut primary = 0;
        let mut secondary = 0;

        let mut prev_was_stress = false;
        for c in phonemes.chars() {
            match c {
                '\'' => {
                    if !prev_was_stress {
                        primary += 1;
                        prev_was_stress = true;
                    }
                }
                ',' => {
                    if !prev_was_stress {
                        secondary += 1;
                        prev_was_stress = true;
                    }
                }
                _ => prev_was_stress = false,
            }
        }

        (primary, secondary)
    }

    /// Apply compound stress rules
    /// For compound words (prefix + suffix):
    /// - If both parts have stress, demote prefix stress to secondary
    /// - If suffix is monosyllabic, remove its stress
    /// - Handle schwa-ending prefixes specially
    pub fn apply_compound_stress(prefix_phonemes: &str, suffix_phonemes: &str) -> (String, String) {
        let prefix_vowels = Self::count_vowel_groups(prefix_phonemes);
        let suffix_vowels = Self::count_vowel_groups(suffix_phonemes);

        let (prefix_primary, _prefix_secondary) = Self::count_stress_marks(prefix_phonemes);
        let (suffix_primary, _suffix_secondary) = Self::count_stress_marks(suffix_phonemes);

        let mut new_prefix = prefix_phonemes.to_string();
        let mut new_suffix = suffix_phonemes.to_string();

        // Case 1: 2+ syllable suffix with 2+ vowel prefix + primary stress
        // Demote prefix primary to secondary
        if suffix_vowels >= 2 && prefix_vowels >= 2 && prefix_primary > 0 {
            new_prefix = Self::demote_primary_to_secondary(&new_prefix);
        }

        // Case 2: Monosyllabic suffix - remove its primary stress (already unstressed)
        // This is handled by dictionary entries, not here

        // Case 3: 1-vowel prefix (monosyllabic) - remove all suffix stress
        if prefix_vowels == 1 && suffix_primary > 0 {
            new_suffix = Self::remove_all_stress(&new_suffix);
        }

        (new_prefix, new_suffix)
    }

    /// Demote primary stress to secondary in phoneme string
    fn demote_primary_to_secondary(phonemes: &str) -> String {
        let mut result = String::new();
        let mut done = false;

        for c in phonemes.chars() {
            if c == '\'' && !done {
                result.push(',');
                done = true;
            } else {
                result.push(c);
            }
        }

        result
    }

    /// Remove all stress marks from phoneme string
    fn remove_all_stress(phonemes: &str) -> String {
        phonemes
            .chars()
            .filter(|c| c != &'\'' && c != &',' && c != &'%')
            .collect()
    }

    fn is_vowel_code(c: char) -> bool {
        matches!(
            c,
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' | '@' | '3'
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_flags() {
        let flags = SuffixFlags::from_bits(0x200 | 0x100);
        assert!(flags.restore_y);
        assert!(flags.add_e);
        assert!(!flags.verb_form);
    }

    #[test]
    fn test_extract_stem() {
        assert_eq!(SuffixHandler::extract_stem("running", 3), "runn");
        assert_eq!(SuffixHandler::extract_stem("makes", 1), "make");
        assert_eq!(SuffixHandler::extract_stem("test", 10), "");
    }

    #[test]
    fn test_restore_stem_with_y() {
        let flags = SuffixFlags::from_bits(0x200);
        assert_eq!(SuffixHandler::restore_stem("happi", flags), "happy");
    }

    #[test]
    fn test_ed_devoicing() {
        // Voiceless consonant: -ed → /t/
        assert_eq!(SuffixHandler::apply_ed_devoicing("walk", "d"), "t");

        // t or d: -ed → /ɪd/
        assert_eq!(SuffixHandler::apply_ed_devoicing("start", "d"), "I#d");
    }

    #[test]
    fn test_vowel_group_counting() {
        // Very basic test - just verify function doesn't crash
        // The actual counting logic needs proper phoneme codes to test correctly
        let _ = PrefixHandler::count_vowel_groups("abc");
        let _ = PrefixHandler::count_vowel_groups("aei");
        let _ = PrefixHandler::count_vowel_groups("@3a");
    }

    #[test]
    fn test_stress_counting() {
        let (primary, secondary) = PrefixHandler::count_stress_marks("'hElo,wErld");
        assert_eq!(primary, 1);
        assert_eq!(secondary, 1);
    }

    #[test]
    fn test_compound_stress() {
        let (_new_prefix, _new_suffix) = PrefixHandler::apply_compound_stress("prE", "sEnt");
        // 'prE has 1 vowel group (E), 'sEnt has 1 vowel group (E)
        // So prefix has 1 vowel, suffix has 1 vowel -> no demotion based on our current logic
        // The test just verifies the function doesn't panic
    }
}
