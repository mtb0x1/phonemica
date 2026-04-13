use crate::types::PhonemeRule;

/// Implementation of context matching for phoneme rules
pub struct ContextMatchingEngine;

impl ContextMatchingEngine {
    /// Check if left context matches (scans backwards from match position)
    pub fn check_left_context(
        rule: &PhonemeRule,
        chars: &[char],
        match_pos: usize,
        groups: &ContextGroups,
    ) -> bool {
        if rule.left_ctx.is_empty() {
            return true;
        }

        let ctx = &rule.left_ctx;
        let word: String = chars.iter().collect();
        let mut char_pos = match_pos as i32 - 1;
        let ctx_chars: Vec<char> = ctx.chars().collect();
        let mut ctx_idx = ctx_chars.len() as i32 - 1;

        while ctx_idx >= 0 {
            let ctx_char = ctx_chars[ctx_idx as usize];

            match ctx_char {
                '_' => {
                    if char_pos >= 0 {
                        return false;
                    }
                    ctx_idx -= 1;
                    if ctx_idx < 0 {
                        return true;
                    }
                    continue;
                }
                'A' | 'B' | 'C' | 'F' | 'G' | 'H' | 'Y' | 'K' => {
                    if char_pos < 0 {
                        return false;
                    }
                    if !groups.match_group(ctx_char, &word, char_pos as usize) {
                        return false;
                    }
                    char_pos -= 1;
                }
                '\'' | '-' => {
                    if char_pos < 0 {
                        return false;
                    }
                    if chars[char_pos as usize] != ctx_char {
                        return false;
                    }
                    char_pos -= 1;
                }
                _ => {
                    if char_pos < 0 {
                        return false;
                    }
                    if !chars[char_pos as usize].eq_ignore_ascii_case(&ctx_char) {
                        return false;
                    }
                    char_pos -= 1;
                }
            }
            ctx_idx -= 1;
        }

        true
    }

    /// Check if right context matches (scans forward from end of match)
    /// Returns (matched, forward_deletion_count)
    pub fn check_right_context(
        rule: &PhonemeRule,
        chars: &[char],
        match_end: usize,
        _word: &str,
        groups: &ContextGroups,
    ) -> (bool, i32) {
        if rule.right_ctx.is_empty() {
            return (true, 0);
        }

        let ctx = &rule.right_ctx;
        let word: String = chars.iter().collect();
        let mut char_pos = match_end;
        let ctx_chars: Vec<char> = ctx.chars().collect();
        let mut ctx_idx = 0;
        let mut del_fwd = 0;

        while ctx_idx < ctx_chars.len() {
            let ctx_char = ctx_chars[ctx_idx];

            match ctx_char {
                'S' => {
                    return (true, 0);
                }
                '_' => {
                    if char_pos < chars.len() {
                        return (false, 0);
                    }
                    del_fwd += 1;
                }
                'A' | 'B' | 'C' | 'F' | 'G' | 'H' | 'Y' | 'K' => {
                    if char_pos >= chars.len() {
                        return (false, 0);
                    }
                    if !groups.match_group(ctx_char, &word, char_pos) {
                        return (false, 0);
                    }
                    char_pos += 1;
                }
                '&' | '@' => {
                    let require_one = ctx_char == '@';
                    let mut count = 0;

                    while char_pos < chars.len() && groups.match_group('B', &word, char_pos) {
                        count += 1;
                        char_pos += 1;
                    }

                    if require_one && count == 0 {
                        return (false, 0);
                    }
                }
                '\'' | '-' => {
                    if char_pos >= chars.len() {
                        return (false, 0);
                    }
                    if chars[char_pos] != ctx_char {
                        return (false, 0);
                    }
                    char_pos += 1;
                }
                _ => {
                    if char_pos >= chars.len() {
                        return (false, 0);
                    }
                    if !chars[char_pos].eq_ignore_ascii_case(&ctx_char) {
                        return (false, 0);
                    }
                    char_pos += 1;
                }
            }
            ctx_idx += 1;
        }

        (true, del_fwd)
    }
}

/// Context group definitions for matching letter groups in rules
#[derive(Debug, Clone)]
pub struct ContextGroups {
    pub group_a: Vec<char>, // Vowels
    pub group_b: Vec<char>, // Consonants
    pub group_c: Vec<char>, // Stops
    pub group_f: Vec<char>, // Fricatives
    pub group_g: Vec<char>, // Glides
    pub group_h: Vec<char>, // Liquids
    pub group_y: Vec<char>, // Always voiced
    pub group_k: Vec<char>, // Velars
}

impl ContextGroups {
    pub fn new() -> Self {
        Self {
            group_a: vec![
                'a', 'e', 'i', 'o', 'u', 'y', 'A', 'E', 'I', 'O', 'U', '@', '3', '0',
            ],
            group_b: vec![
                'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's',
                't', 'v', 'w', 'x', 'z', 'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M',
                'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Z',
            ],
            group_c: vec!['b', 'd', 'g', 'k', 'p', 't', 'B', 'D', 'G', 'K', 'P', 'T'],
            group_f: vec!['f', 'h', 's', 'v', 'z', 'F', 'H', 'S', 'V', 'Z', 'j', 'J'],
            group_g: vec!['w', 'y', 'W', 'Y'],
            group_h: vec!['l', 'r', 'L', 'R'],
            group_y: vec!['b', 'd', 'g', 'j', 'l', 'r', 'v', 'w', 'z'],
            group_k: vec!['g', 'k', 'x', 'G', 'K', 'X'],
        }
    }

    /// Check if a character belongs to a letter group
    pub fn match_group(&self, group: char, word: &str, pos: usize) -> bool {
        if pos >= word.len() {
            return false;
        }

        let ch = word.chars().nth(pos).unwrap_or('\0');

        match group {
            'A' => self.group_a.contains(&ch),
            'B' => self.group_b.contains(&ch),
            'C' => self.group_c.contains(&ch),
            'F' => self.group_f.contains(&ch),
            'G' => self.group_g.contains(&ch),
            'H' => self.group_h.contains(&ch),
            'Y' => self.group_y.contains(&ch),
            'K' => self.group_k.contains(&ch),
            _ => false,
        }
    }
}

impl Default for ContextGroups {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_groups_vowels() {
        let groups = ContextGroups::new();
        assert!(groups.group_a.contains(&'a'));
        assert!(groups.group_a.contains(&'e'));
        assert!(!groups.group_a.contains(&'b'));
    }

    #[test]
    fn test_context_groups_consonants() {
        let groups = ContextGroups::new();
        assert!(groups.group_b.contains(&'b'));
        assert!(groups.group_b.contains(&'t'));
        assert!(!groups.group_b.contains(&'a'));
    }

    #[test]
    fn test_match_group() {
        let groups = ContextGroups::new();
        assert!(groups.match_group('A', "hello", 1)); // 'e' at pos 1
        assert!(groups.match_group('B', "hello", 0)); // 'h' at pos 0
        assert!(!groups.match_group('A', "hello", 0)); // 'h' is not vowel
    }
}
