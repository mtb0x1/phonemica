use std::collections::HashMap;

/// Trait for IPA conversion from phoneme codes
pub trait IpaConverter {
    /// Convert phoneme codes to IPA characters
    fn phonemes_to_ipa(&self, phoneme_str: &str) -> String;

    /// Check if a code represents a vowel
    fn is_vowel(&self, code: &str) -> bool;

    /// Get the dialect name
    fn dialect(&self) -> &str;
}

/// American English IPA converter
pub struct AmericanEnglishConverter {
    overrides: HashMap<String, String>,
}

impl AmericanEnglishConverter {
    pub fn new() -> Self {
        let mut overrides = HashMap::new();

        // General phoneme mappings
        overrides.insert("r".to_string(), "ɹ".to_string());
        overrides.insert("r-".to_string(), "ɹ".to_string());
        overrides.insert("n-".to_string(), "n̩".to_string());
        overrides.insert("m-".to_string(), "m̩".to_string());
        overrides.insert("3:r".to_string(), "ɜːɹ".to_string());
        overrides.insert("3:".to_string(), "ɜː".to_string());
        overrides.insert("@L".to_string(), "əl".to_string());
        overrides.insert("a#".to_string(), "ɐ".to_string());
        overrides.insert("e#".to_string(), "ɛ".to_string());
        overrides.insert("I#".to_string(), "ᵻ".to_string());
        overrides.insert("I2#".to_string(), "ᵻ".to_string());
        overrides.insert("w#".to_string(), "ʍ".to_string());
        overrides.insert("@2".to_string(), "ə".to_string());
        overrides.insert("@5".to_string(), "ə".to_string());
        overrides.insert("I2".to_string(), "ɪ".to_string());

        // American English specific
        overrides.insert("3".to_string(), "ɚ".to_string());
        overrides.insert("a".to_string(), "æ".to_string());
        overrides.insert("aa".to_string(), "æ".to_string());
        overrides.insert("0".to_string(), "ɑː".to_string());
        overrides.insert("0#".to_string(), "ɑː".to_string());
        overrides.insert("A#".to_string(), "ɑː".to_string());
        overrides.insert("A@".to_string(), "ɑːɹ".to_string());
        overrides.insert("A:r".to_string(), "ɑːɹ".to_string());
        overrides.insert("e@".to_string(), "ɛɹ".to_string());
        overrides.insert("e@r".to_string(), "ɛɹ".to_string());
        overrides.insert("U@".to_string(), "ʊɹ".to_string());
        overrides.insert("O@".to_string(), "ɔːɹ".to_string());
        overrides.insert("O@r".to_string(), "ɔːɹ".to_string());
        overrides.insert("o@".to_string(), "oːɹ".to_string());
        overrides.insert("o@r".to_string(), "oːɹ".to_string());
        overrides.insert("i@".to_string(), "iə".to_string());
        overrides.insert("i@3".to_string(), "ɪɹ".to_string());
        overrides.insert("i@3r".to_string(), "ɪɹ".to_string());
        overrides.insert("aI@".to_string(), "aɪə".to_string());
        overrides.insert("aI3".to_string(), "aɪɚ".to_string());
        overrides.insert("aU@".to_string(), "aɪʊɹ".to_string());
        overrides.insert("IR".to_string(), "əɹ".to_string());
        overrides.insert("VR".to_string(), "ʌɹ".to_string());
        overrides.insert("02".to_string(), "ʌ".to_string());
        overrides.insert("i".to_string(), "i".to_string());

        Self { overrides }
    }
}

impl Default for AmericanEnglishConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl IpaConverter for AmericanEnglishConverter {
    fn phonemes_to_ipa(&self, phoneme_str: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = phoneme_str.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            if c == '\'' {
                result.push('ˈ');
                i += 1;
                continue;
            }
            if c == ',' {
                result.push('ˌ');
                i += 1;
                continue;
            }
            if c == '%' {
                result.push('ˏ');
                i += 1;
                continue;
            }
            if c == ':' {
                result.push('ː');
                i += 1;
                continue;
            }
            if c == '#' {
                i += 1;
                continue;
            }
            if c == '_' {
                result.push_str(". ");
                i += 1;
                continue;
            }
            if c == '/' {
                i += 1;
                continue;
            }
            if c == '|' {
                i += 1;
                continue;
            }

            // Try multi-character matches
            let mut found_override = false;
            for len in (1..=4).rev() {
                if i + len <= chars.len() {
                    let code: String = chars[i..i + len].iter().collect();
                    if let Some(ipa) = self.overrides.get(&code) {
                        result.push_str(ipa);
                        i += len;
                        found_override = true;
                        break;
                    }
                }
            }

            if !found_override {
                result.push(c);
                i += 1;
            }
        }

        result
    }

    fn is_vowel(&self, code: &str) -> bool {
        if let Some(ipa) = self.overrides.get(code) {
            let last_char = ipa.chars().last().unwrap_or('\0');
            matches!(
                last_char,
                'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ə' | 'æ' | 'ɪ' | 'ʊ' | 'ɐ' | 'ɑ'
                    | 'ɔ' | 'ʌ' | 'ᴜ' | 'ᵻ'
            )
        } else {
            code.chars().any(|c| {
                matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
            })
        }
    }

    fn dialect(&self) -> &str {
        "en-us"
    }
}

/// British English IPA converter
pub struct BritishEnglishConverter {
    overrides: HashMap<String, String>,
}

impl BritishEnglishConverter {
    pub fn new() -> Self {
        let mut overrides = HashMap::new();

        // General phoneme mappings (same as American for shared phonemes)
        overrides.insert("r".to_string(), "ɹ".to_string());
        overrides.insert("r-".to_string(), "ɹ".to_string());
        overrides.insert("n-".to_string(), "n̩".to_string());
        overrides.insert("m-".to_string(), "m̩".to_string());
        overrides.insert("3:r".to_string(), "ɜːɹ".to_string());
        overrides.insert("3:".to_string(), "ɜː".to_string());
        overrides.insert("@L".to_string(), "əl".to_string());
        overrides.insert("a#".to_string(), "ɐ".to_string());
        overrides.insert("e#".to_string(), "ɛ".to_string());
        overrides.insert("I#".to_string(), "ᵻ".to_string());
        overrides.insert("I2#".to_string(), "ᵻ".to_string());
        overrides.insert("w#".to_string(), "ʍ".to_string());
        overrides.insert("@2".to_string(), "ə".to_string());
        overrides.insert("@5".to_string(), "ə".to_string());
        overrides.insert("I2".to_string(), "ɪ".to_string());

        // British English specific (non-rhotic)
        overrides.insert("3".to_string(), "ɜ".to_string());
        overrides.insert("a".to_string(), "a".to_string());
        overrides.insert("aa".to_string(), "a".to_string());
        overrides.insert("0".to_string(), "ɒ".to_string());
        overrides.insert("oU".to_string(), "əʊ".to_string());
        overrides.insert("A@".to_string(), "ɑː".to_string());
        overrides.insert("IR".to_string(), "əɹ".to_string());

        Self { overrides }
    }
}

impl Default for BritishEnglishConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl IpaConverter for BritishEnglishConverter {
    fn phonemes_to_ipa(&self, phoneme_str: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = phoneme_str.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            if c == '\'' {
                result.push('ˈ');
                i += 1;
                continue;
            }
            if c == ',' {
                result.push('ˌ');
                i += 1;
                continue;
            }
            if c == '%' {
                result.push('ˏ');
                i += 1;
                continue;
            }
            if c == ':' {
                result.push('ː');
                i += 1;
                continue;
            }
            if c == '#' {
                i += 1;
                continue;
            }
            if c == '_' {
                result.push_str(". ");
                i += 1;
                continue;
            }
            if c == '/' {
                i += 1;
                continue;
            }
            if c == '|' {
                i += 1;
                continue;
            }

            let mut found_override = false;
            for len in (1..=4).rev() {
                if i + len <= chars.len() {
                    let code: String = chars[i..i + len].iter().collect();
                    if let Some(ipa) = self.overrides.get(&code) {
                        result.push_str(ipa);
                        i += len;
                        found_override = true;
                        break;
                    }
                }
            }

            if !found_override {
                result.push(c);
                i += 1;
            }
        }

        result
    }

    fn is_vowel(&self, code: &str) -> bool {
        if let Some(ipa) = self.overrides.get(code) {
            let last_char = ipa.chars().last().unwrap_or('\0');
            matches!(
                last_char,
                'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ə' | 'æ' | 'ɪ' | 'ʊ' | 'ɐ' | 'ɑ'
                    | 'ɔ' | 'ʌ'
            )
        } else {
            code.chars().any(|c| {
                matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
            })
        }
    }

    fn dialect(&self) -> &str {
        "en-gb"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_american_conversion() {
        let converter = AmericanEnglishConverter::new();
        let result = converter.phonemes_to_ipa("'a");
        assert!(result.contains('ˈ')); // Primary stress marker
        assert!(result.contains('æ')); // American 'a'
    }

    #[test]
    fn test_british_conversion() {
        let converter = BritishEnglishConverter::new();
        let result = converter.phonemes_to_ipa("'a");
        assert!(result.contains('ˈ')); // Primary stress marker
    }

    #[test]
    fn test_vowel_detection_us() {
        let converter = AmericanEnglishConverter::new();
        assert!(converter.is_vowel("a"));
        assert!(converter.is_vowel("e"));
        assert!(!converter.is_vowel("b"));
    }
}
