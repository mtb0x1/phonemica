/// Stress placement configuration
#[derive(Debug, Clone, Copy)]
pub struct StressConfig {
    pub use_stress_dict: bool,
    pub use_final_stress: bool,
    pub default_to_final: bool,
}

impl StressConfig {
    pub fn new() -> Self {
        Self {
            use_stress_dict: true,
            use_final_stress: true,
            default_to_final: false,
        }
    }
}

impl Default for StressConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Stress placement types in IPA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StressPlace {
    /// Primary stress (ˈ)
    Primary,
    /// Secondary stress (ˌ)
    Secondary,
    /// Unstressed (ˏ)
    Unstressed,
    /// No stress marker
    None,
}

/// Trait for stress processing strategies
pub trait StressProcessor {
    /// Apply stress at a specific vowel position (1-based)
    fn apply_stress_position(&self, phoneme_str: &str, pos: i32) -> String;

    /// Apply stress to the final vowel
    fn apply_final_stress(&self, phoneme_str: &str) -> String;

    /// Process a phoneme string and apply appropriate stress
    fn process_phoneme_string(
        &self,
        word: &str,
        phoneme_str: &str,
        force_final_stress: bool,
    ) -> String;

    /// Insert last-resort stress on first vowel if none present
    fn insert_last_resort_stress(&self, phoneme_str: &str) -> String;
}

/// Default stress processor implementation
pub struct DefaultStressProcessor {
    #[allow(dead_code)]
    config: StressConfig,
}

impl DefaultStressProcessor {
    pub fn new(config: StressConfig) -> Self {
        Self { config }
    }

    fn is_vowel_phoneme_char(ch: char) -> bool {
        matches!(
            ch,
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'A' | 'E' | 'I' | 'O' | 'U' | '@' | '3' | '0'
        )
    }

    #[allow(dead_code)]
    fn parse_stress_marker(ch: char) -> StressPlace {
        match ch {
            '\'' => StressPlace::Primary,
            ',' => StressPlace::Secondary,
            '%' => StressPlace::Unstressed,
            _ => StressPlace::None,
        }
    }
}

impl StressProcessor for DefaultStressProcessor {
    fn apply_stress_position(&self, phoneme_str: &str, pos: i32) -> String {
        if pos <= 0 || phoneme_str.is_empty() {
            return phoneme_str.to_string();
        }

        let mut vowel_count = 0;
        let mut result = String::new();
        let mut target_vowel_pos = None;
        let chars: Vec<char> = phoneme_str.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if Self::is_vowel_phoneme_char(ch) {
                vowel_count += 1;
                if vowel_count == pos {
                    target_vowel_pos = Some(i);
                    break;
                }
            }
        }

        if target_vowel_pos.is_none() {
            return phoneme_str.to_string();
        }

        let target_pos = target_vowel_pos.unwrap();
        let mut prev_was_stress = false;

        for (i, &ch) in chars.iter().enumerate() {
            if i == target_pos {
                if prev_was_stress && (ch == '\'' || ch == ',' || ch == '%') {
                    continue;
                }
                if ch != '\'' && ch != ',' && ch != '%' {
                    result.push('\'');
                }
            }

            if ch == '\'' && i != target_pos {
                result.push(',');
                prev_was_stress = true;
            } else {
                if i == target_pos && (ch == '\'' || ch == ',' || ch == '%') {
                    prev_was_stress = false;
                    continue;
                }
                result.push(ch);
                prev_was_stress = matches!(ch, '\'' | ',' | '%');
            }
        }

        result
    }

    fn apply_final_stress(&self, phoneme_str: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = phoneme_str.chars().collect();

        let mut last_vowel_pos = None;
        for (i, &ch) in chars.iter().enumerate() {
            if Self::is_vowel_phoneme_char(ch) {
                last_vowel_pos = Some(i);
            }
        }

        if let Some(pos) = last_vowel_pos {
            for (i, &ch) in chars.iter().enumerate() {
                if i == pos && ch != '\'' && ch != ',' {
                    result.push('\'');
                }
                if i == pos && (ch == '\'' || ch == ',' || ch == '%') {
                    continue;
                }
                result.push(ch);
            }
        } else {
            result = phoneme_str.to_string();
        }

        result
    }

    fn process_phoneme_string(
        &self,
        _word: &str,
        phoneme_str: &str,
        _force_final_stress: bool,
    ) -> String {
        // Default: return as-is, to be overridden by specializations
        phoneme_str.to_string()
    }

    fn insert_last_resort_stress(&self, phoneme_str: &str) -> String {
        let chars: Vec<char> = phoneme_str.chars().collect();
        let mut result = String::new();
        let mut inserted = false;

        for &ch in chars.iter() {
            if !inserted && Self::is_vowel_phoneme_char(ch) {
                result.push('\'');
                inserted = true;
            }
            result.push(ch);
        }

        result
    }
}

impl Default for DefaultStressProcessor {
    fn default() -> Self {
        Self::new(StressConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_config_creation() {
        let config = StressConfig::new();
        assert!(config.use_stress_dict);
        assert!(config.use_final_stress);
    }

    #[test]
    fn test_stress_place_variants() {
        assert_eq!(StressPlace::Primary, StressPlace::Primary);
        assert_ne!(StressPlace::Primary, StressPlace::Secondary);
    }

    #[test]
    fn test_apply_stress_position() {
        let processor = DefaultStressProcessor::default();
        let result = processor.apply_stress_position("aeiou", 2);
        // Should place primary stress on second vowel 'e'
        assert!(result.contains('\''));
    }

    #[test]
    fn test_apply_final_stress() {
        let processor = DefaultStressProcessor::default();
        let result = processor.apply_final_stress("aeiou");
        // Should place primary stress on last vowel 'u'
        assert!(result.ends_with('u') || result.ends_with("'u"));
    }

    #[test]
    fn test_insert_last_resort_stress() {
        let processor = DefaultStressProcessor::default();
        let result = processor.insert_last_resort_stress("hello");
        assert!(result.contains('\''));
        // Stress should be inserted on first vowel ('e' at index 1)
        assert!(result.contains("h'e"));
    }
}
