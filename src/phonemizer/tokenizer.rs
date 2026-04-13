use crate::types::Token;

/// Text tokenizer for breaking text into words and punctuation
pub trait Tokenizer {
    /// Tokenize text into words and non-words
    fn tokenize(&self, text: &str) -> Vec<Token>;

    /// Normalize text before tokenization
    fn normalize(&self, text: &str) -> String;
}

/// Default text tokenizer implementation
pub struct TextTokenizer;

impl TextTokenizer {
    pub fn new() -> Self {
        Self
    }

    fn normalize_text(text: &str) -> String {
        text.chars()
            .map(|c| if c.is_whitespace() { ' ' } else { c })
            .collect()
    }

    fn tokenize_text(text: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        let mut last_was_space = true;

        while i < chars.len() {
            let c = chars[i];

            if c.is_whitespace() {
                if !tokens.is_empty() {
                    tokens.last_mut().unwrap().needs_space_before = false;
                }
                last_was_space = true;
                i += 1;
                continue;
            }

            let is_word_char = c.is_alphabetic() || c == '\'' || c == '-';

            if is_word_char {
                let mut word = String::new();
                while i < chars.len() {
                    let wc = chars[i];
                    if wc.is_alphabetic() || wc == '\'' || wc == '-' {
                        word.push(wc);
                        i += 1;
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    text: word,
                    is_word: true,
                    needs_space_before: !last_was_space,
                });
            } else {
                let mut punct = String::new();
                while i < chars.len() {
                    let pc = chars[i];
                    if !pc.is_alphabetic() && !pc.is_whitespace() {
                        punct.push(pc);
                        i += 1;
                    } else {
                        break;
                    }
                }

                if !punct.is_empty() {
                    tokens.push(Token {
                        text: punct,
                        is_word: false,
                        needs_space_before: !last_was_space,
                    });
                }
            }

            last_was_space = false;
        }

        tokens
    }
}

impl Default for TextTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer for TextTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        let normalized = self.normalize(text);
        Self::tokenize_text(&normalized)
    }

    fn normalize(&self, text: &str) -> String {
        Self::normalize_text(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let tokenizer = TextTokenizer::new();
        let result = tokenizer.normalize("Hello\nWorld\tTest");
        assert!(result.contains("Hello World Test"));
    }

    #[test]
    fn test_tokenize_simple() {
        let tokenizer = TextTokenizer::new();
        let tokens = tokenizer.tokenize("Hello world");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "Hello");
        assert_eq!(tokens[0].is_word, true);
        assert_eq!(tokens[1].text, "world");
    }

    #[test]
    fn test_tokenize_with_punctuation() {
        let tokenizer = TextTokenizer::new();
        let tokens = tokenizer.tokenize("Hello, world!");
        // Tokens should be: ["Hello", ",", "world", "!"]
        assert_eq!(tokens[0].text, "Hello");
        assert_eq!(tokens[1].text, ",");
        assert_eq!(tokens[2].text, "world");
        assert_eq!(tokens[3].text, "!");
    }

    #[test]
    fn test_tokenize_apostrophe() {
        let tokenizer = TextTokenizer::new();
        let tokens = tokenizer.tokenize("don't");
        assert_eq!(tokens[0].text, "don't");
        assert_eq!(tokens[0].is_word, true);
    }

    #[test]
    fn test_tokenize_hyphenated() {
        let tokenizer = TextTokenizer::new();
        let tokens = tokenizer.tokenize("state-of-the-art");
        assert_eq!(tokens[0].text, "state-of-the-art");
        assert_eq!(tokens[0].is_word, true);
    }
}
