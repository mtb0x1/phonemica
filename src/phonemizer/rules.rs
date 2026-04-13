use crate::types::PhonemeRule;

/// Result of matching a single rule at a position
#[derive(Debug, Clone)]
pub struct RuleMatch {
    pub phonemes: String,
    pub advance: usize,
    pub score: i32,
}

/// Trait for matching phoneme rules at specific positions
pub trait RuleMatcher {
    /// Try to match a rule at the given position in the word
    fn match_rule_at_pos(
        &self,
        rule: &PhonemeRule,
        chars: &[char],
        pos: usize,
        word: &str,
    ) -> Option<RuleMatch>;
}

/// Context matching for rule left and right contexts
pub trait ContextMatcher {
    /// Check if the left context of a rule matches at the given position
    fn check_left_context(
        &self,
        rule: &PhonemeRule,
        chars: &[char],
        match_pos: usize,
    ) -> bool;

    /// Check if the right context of a rule matches at the given position
    /// Returns tuple of (matched, forward_deletion_count)
    fn check_right_context(
        &self,
        rule: &PhonemeRule,
        chars: &[char],
        match_end: usize,
        word: &str,
    ) -> (bool, i32);
}

/// Scoring logic for rule matches
/// Higher scores indicate better matches and are prioritized
pub struct MatchScorer;

impl MatchScorer {
    /// Calculate a match score based on multiple factors
    pub fn score_match(
        match_len: usize,
        pos: usize,
        total_len: usize,
        rule: &PhonemeRule,
        has_left_context: bool,
        has_right_context: bool,
    ) -> i32 {
        // Base score: 1 point + 21 for each extra character beyond the first
        let extra_chars = (match_len as i32 - 1).max(0) * 21;
        let mut score = 1 + extra_chars;

        // Priority: 2-character matches strong advantage
        if match_len >= 2 {
            score += 100;
        }

        // Bonus for matching at word start
        if pos == 0 {
            score += 5;
        }

        // Bonus for matching at word end
        if pos + match_len >= total_len {
            score += 3;
        }

        // Bonus for left context
        if has_left_context {
            if rule.left_ctx.contains('_') {
                score += 20; // Word boundary is strong signal
            } else {
                score += 10; // Specific context is good signal
            }
        }

        // Bonus for right context
        if has_right_context {
            if rule.right_ctx.contains('_') {
                score += 20;
            } else if !rule.right_ctx.contains('S') {
                score += 8;
            }
        }

        score
    }
    
    /// Check if a score represents a valid match
    pub fn is_valid_match(score: i32) -> bool {
        score >= 0
    }
}

/// Application of phoneme rules
pub struct RuleApplier;

impl RuleApplier {
    /// Check if a phoneme is a vowel-like character in phoneme notation
    pub fn is_vowel_phoneme(phoneme: char) -> bool {
        matches!(
            phoneme,
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'A' | 'E' | 'I' | 'O' | 'U' | '@' | '3' | '0'
        )
    }

    /// Check if a character represents a vowel letter
    pub fn is_vowel_letter(c: char) -> bool {
        let c = c.to_ascii_lowercase();
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
    }

    /// Convert character to lowercase ASCII if applicable
    pub fn to_lower_ascii(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_ascii() && c.is_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c
                }
            })
            .collect()
    }

    /// Trim whitespace from a string
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }

    /// Split string by whitespace
    pub fn split_ws(s: &str) -> Vec<String> {
        s.split_whitespace().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowel_phoneme() {
        assert!(RuleApplier::is_vowel_phoneme('a'));
        assert!(RuleApplier::is_vowel_phoneme('@'));
        assert!(RuleApplier::is_vowel_phoneme('3'));
        assert!(!RuleApplier::is_vowel_phoneme('b'));
    }

    #[test]
    fn test_vowel_letter() {
        assert!(RuleApplier::is_vowel_letter('a'));
        assert!(RuleApplier::is_vowel_letter('E'));
        assert!(RuleApplier::is_vowel_letter('y'));
        assert!(!RuleApplier::is_vowel_letter('b'));
    }

    #[test]
    fn test_to_lower_ascii() {
        assert_eq!(RuleApplier::to_lower_ascii("Hello"), "hello");
        assert_eq!(RuleApplier::to_lower_ascii("WORLD"), "world");
    }

    #[test]
    fn test_trim() {
        assert_eq!(RuleApplier::trim("  hello  "), "hello");
        assert_eq!(RuleApplier::trim("world"), "world");
    }

    #[test]
    fn test_split_ws() {
        let result = RuleApplier::split_ws("hello world  test");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "hello");
        assert_eq!(result[1], "world");
        assert_eq!(result[2], "test");
    }
}
