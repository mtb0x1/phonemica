use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhTokenType {
    StressPrimary,
    StressSecondary,
    Phoneme,
    Pause,
    Syllable,
}

#[derive(Debug, Clone)]
pub struct PhToken {
    pub token_type: PhTokenType,
    pub code: String,
    pub is_vowel: bool,
}

impl PhToken {
    pub fn new(token_type: PhTokenType, code: String, is_vowel: bool) -> Self {
        Self {
            token_type,
            code,
            is_vowel,
        }
    }
}

pub struct Token {
    pub text: String,
    pub is_word: bool,
    pub needs_space_before: bool,
}

#[derive(Debug, Clone)]
pub struct ReplaceRule {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Default, Clone)]
pub struct PhonemeRule {
    pub condition: i32,
    pub condition_negated: bool,
    pub left_ctx: String,
    pub match_str: String,
    pub right_ctx: String,
    pub phonemes: String,
    pub del_fwd: i32,
    pub is_prefix: bool,
    pub is_suffix: bool,
    pub suffix_strip_len: i32,
    pub suffix_flags: i32,
}
pub struct LetterGroups {
    pub group_a: HashSet<char>,
    pub group_b: HashSet<char>,
    pub group_c: HashSet<char>,
    pub group_f: HashSet<char>,
    pub group_g: HashSet<char>,
    pub group_h: HashSet<char>,
    pub group_y: HashSet<char>,
    pub group_k: HashSet<char>,
    pub lgroups: Vec<Vec<String>>,
}

impl LetterGroups {
    pub fn new() -> Self {
        let mut groups = Self {
            group_a: HashSet::new(),
            group_b: HashSet::new(),
            group_c: HashSet::new(),
            group_f: HashSet::new(),
            group_g: HashSet::new(),
            group_h: HashSet::new(),
            group_y: HashSet::new(),
            group_k: HashSet::new(),
            lgroups: vec![Vec::new(); 100],
        };
        groups.init();
        groups
    }

    pub fn init(&mut self) {
        for c in "aeiou".chars() {
            self.group_a.insert(c);
        }
        for c in "bcdfgjklmnpqstvxz".chars() {
            self.group_b.insert(c);
        }
        for c in "bcdfghjklmnpqrstvwxz".chars() {
            self.group_c.insert(c);
        }
        for c in "hlmnr".chars() {
            self.group_h.insert(c);
        }
        for c in "cfhkpqstx".chars() {
            self.group_f.insert(c);
        }
        for c in "bdgjlmnrvwyz".chars() {
            self.group_g.insert(c);
        }
        for c in "aeiouy".chars() {
            self.group_y.insert(c);
        }
        for c in "bcdfghjklmnpqrstvwxyz".chars() {
            self.group_k.insert(c);
        }
    }

    pub fn is_vowel(&self, c: char) -> bool {
        self.group_a.contains(&c.to_ascii_lowercase())
    }

    pub fn match_group(&self, group_char: char, word: &str, pos: usize) -> bool {
        if pos >= word.len() {
            return false;
        }
        let c = word
            .chars()
            .nth(pos)
            .map_or(' ', |ch| ch.to_ascii_lowercase());
        match group_char {
            'A' => self.group_a.contains(&c),
            'B' => self.group_b.contains(&c),
            'C' => self.group_c.contains(&c),
            'F' => self.group_f.contains(&c),
            'G' => self.group_g.contains(&c),
            'H' => self.group_h.contains(&c),
            'Y' => self.group_y.contains(&c),
            'K' => self.group_k.contains(&c),
            _ => false,
        }
    }
}

impl Default for LetterGroups {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RuleSet {
    pub groups: LetterGroups,
    pub replacements: Vec<ReplaceRule>,
    pub rule_groups: HashMap<String, Vec<PhonemeRule>>,
}

impl RuleSet {
    pub fn new() -> Self {
        Self {
            groups: LetterGroups::new(),
            replacements: Vec::new(),
            rule_groups: HashMap::new(),
        }
    }
}

impl Default for RuleSet {
    fn default() -> Self {
        Self::new()
    }
}
