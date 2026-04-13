use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Cursor;
use std::io::{BufRead, BufReader};

use crate::types::{PhonemeRule, ReplaceRule, RuleSet, Token};

pub struct IPAPhonemizer {
    dialect: String,
    loaded: bool,
    error: String,
    dict: HashMap<String, String>,
    verb_dict: HashMap<String, String>,
    #[allow(dead_code)]
    past_dict: HashMap<String, String>,
    noun_dict: HashMap<String, String>,
    pastf_words: HashSet<String>,
    nounf_words: HashSet<String>,
    verbf_words: HashSet<String>,
    ruleset: RuleSet,
    ipa_overrides: HashMap<String, String>,
    unstressed_words: HashSet<String>,
    unstressend_words: HashSet<String>,
    abbrev_words: HashSet<String>,
    stress_pos: HashMap<String, i32>,
    word_alt_flags: HashMap<String, i32>,
    atstart_dict: HashMap<String, String>,
    atend_dict: HashMap<String, String>,
    capital_dict: HashMap<String, String>,
    onlys_words: HashSet<String>,
    onlys_bare_dict: HashMap<String, String>,
    only_words: HashSet<String>,
    noun_form_stress: HashSet<String>,
    verb_flag_words: HashSet<String>,
    compound_prefixes: Vec<(String, String)>,
    strend_words: HashSet<String>,
    u2_strend2_words: HashSet<String>,
    comma_strend2_words: HashSet<String>,
    u_plus_secondary_words: HashSet<String>,
    phrase_dict: HashMap<String, String>,
    phrase_split_dict: HashMap<String, (String, String)>,
    keep_sec_phrase_keys: HashSet<String>,
}

impl IPAPhonemizer {
    pub fn new(rules_path: &str, list_path: &str, dialect: &str) -> Self {
        let mut phonemizer = Self {
            dialect: dialect.to_string(),
            loaded: false,
            error: String::new(),
            dict: HashMap::new(),
            verb_dict: HashMap::new(),
            past_dict: HashMap::new(),
            noun_dict: HashMap::new(),
            pastf_words: HashSet::new(),
            nounf_words: HashSet::new(),
            verbf_words: HashSet::new(),
            ruleset: RuleSet::new(),
            ipa_overrides: HashMap::new(),
            unstressed_words: HashSet::new(),
            unstressend_words: HashSet::new(),
            abbrev_words: HashSet::new(),
            stress_pos: HashMap::new(),
            word_alt_flags: HashMap::new(),
            atstart_dict: HashMap::new(),
            atend_dict: HashMap::new(),
            capital_dict: HashMap::new(),
            onlys_words: HashSet::new(),
            onlys_bare_dict: HashMap::new(),
            only_words: HashSet::new(),
            noun_form_stress: HashSet::new(),
            verb_flag_words: HashSet::new(),
            compound_prefixes: Vec::new(),
            strend_words: HashSet::new(),
            u2_strend2_words: HashSet::new(),
            comma_strend2_words: HashSet::new(),
            u_plus_secondary_words: HashSet::new(),
            phrase_dict: HashMap::new(),
            phrase_split_dict: HashMap::new(),
            keep_sec_phrase_keys: HashSet::new(),
        };

        phonemizer.ipa_overrides = phonemizer.build_ipa_overrides(dialect);

        if !phonemizer.load_dictionary(list_path) {
            return phonemizer;
        }
        if !phonemizer.load_rules(rules_path) {
            return phonemizer;
        }

        phonemizer.loaded = true;
        phonemizer
    }

    pub fn from_buff(rules_buff: &[u8], list_buff: &[u8], dialect: &str) -> Self {
        let mut phonemizer = Self {
            dialect: dialect.to_string(),
            loaded: false,
            error: String::new(),
            dict: HashMap::new(),
            verb_dict: HashMap::new(),
            past_dict: HashMap::new(),
            noun_dict: HashMap::new(),
            pastf_words: HashSet::new(),
            nounf_words: HashSet::new(),
            verbf_words: HashSet::new(),
            ruleset: RuleSet::new(),
            ipa_overrides: HashMap::new(),
            unstressed_words: HashSet::new(),
            unstressend_words: HashSet::new(),
            abbrev_words: HashSet::new(),
            stress_pos: HashMap::new(),
            word_alt_flags: HashMap::new(),
            atstart_dict: HashMap::new(),
            atend_dict: HashMap::new(),
            capital_dict: HashMap::new(),
            onlys_words: HashSet::new(),
            onlys_bare_dict: HashMap::new(),
            only_words: HashSet::new(),
            noun_form_stress: HashSet::new(),
            verb_flag_words: HashSet::new(),
            compound_prefixes: Vec::new(),
            strend_words: HashSet::new(),
            u2_strend2_words: HashSet::new(),
            comma_strend2_words: HashSet::new(),
            u_plus_secondary_words: HashSet::new(),
            phrase_dict: HashMap::new(),
            phrase_split_dict: HashMap::new(),
            keep_sec_phrase_keys: HashSet::new(),
        };

        phonemizer.ipa_overrides = phonemizer.build_ipa_overrides(dialect);

        if !phonemizer.load_dictionary_from_buff(list_buff) {
            return phonemizer;
        }
        if !phonemizer.load_rules_from_buff(rules_buff) {
            return phonemizer;
        }

        phonemizer.loaded = true;
        phonemizer
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn build_ipa_overrides(&self, dialect: &str) -> HashMap<String, String> {
        let mut overrides = HashMap::new();

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

        if dialect == "en-us" || dialect == "en_us" {
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
        } else {
            overrides.insert("3".to_string(), "ɜ".to_string());
            overrides.insert("a".to_string(), "a".to_string());
            overrides.insert("aa".to_string(), "a".to_string());
            overrides.insert("0".to_string(), "ɒ".to_string());
            overrides.insert("oU".to_string(), "əʊ".to_string());
            overrides.insert("A@".to_string(), "ɑː".to_string());
            overrides.insert("IR".to_string(), "əɹ".to_string());
        }

        overrides
    }

    fn to_lower_ascii(s: &str) -> String {
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

    fn trim(s: &str) -> String {
        s.trim().to_string()
    }

    fn split_ws(s: &str) -> Vec<String> {
        s.split_whitespace().map(|s| s.to_string()).collect()
    }

    #[allow(dead_code)]
    fn is_vowel_letter(c: char) -> bool {
        let c = c.to_ascii_lowercase();
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
    }

    fn load_dictionary(&mut self, path: &str) -> bool {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                self.error = format!("Cannot open dictionary file: {}: {}", path, e);
                return false;
            }
        };

        let reader = BufReader::new(file);
        self.load_dictionary_from_reader(reader)
    }

    fn load_dictionary_from_buff(&mut self, buff: &[u8]) -> bool {
        let reader = BufReader::new(Cursor::new(buff));
        self.load_dictionary_from_reader(reader)
    }

    fn load_dictionary_from_reader<R: BufRead>(&mut self, reader: R) -> bool {
        let is_en_us = self.dialect == "en-us" || self.dialect == "en_us";

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };

            let comment_pos = line.find("//");
            let line = if let Some(pos) = comment_pos {
                &line[..pos]
            } else {
                &line
            };
            let line = Self::trim(line);
            if line.is_empty() {
                continue;
            }

            if self.parse_dictionary_line(&line, is_en_us).is_err() {
                continue;
            }
        }

        self.compound_prefixes
            .sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        self.unstressed_words.remove("made");

        true
    }

    fn parse_dictionary_line(&mut self, line: &str, is_en_us: bool) -> Result<(), ()> {
        if line.starts_with('(') {
            return self.parse_phrase_entry(line, is_en_us);
        }

        let mut dialect_cond = 0i32;
        let mut cond_negated = false;
        let mut remaining: String = line.to_string();

        if let Some(stripped_wut) = line.strip_prefix('?')
            && let Some(space_pos) = stripped_wut.find(|c: char| c.is_whitespace()) {
                let (neg, cond_str) = if let Some(stripped_not) = stripped_wut.strip_prefix('!') {
                    (true, stripped_not)
                } else {
                    (false, stripped_wut)
                };
                if let Ok(cond) = cond_str.parse::<i32>() {
                    dialect_cond = cond;
                    cond_negated = neg;
                }
                remaining = Self::trim(&line[space_pos + 2..]);
            }

        if dialect_cond != 0 {
            let matches = dialect_cond == 3 || dialect_cond == 6;
            let applies = if cond_negated { !matches } else { matches };
            if !applies {
                return Ok(());
            }
        }

        let parts = Self::split_ws(&remaining);
        if parts.len() < 2 {
            return Err(());
        }

        let word = &parts[0];
        let phonemes_str = parts[1].clone();

        let norm_word = Self::to_lower_ascii(word);

        let mut has_noun_flag = false;
        let mut has_verb_flag = false;
        let mut has_pastf_flag = false;
        let mut has_nounf_flag = false;
        let mut has_verbf_flag = false;
        let mut has_atend_flag = false;
        let mut has_capital_flag = false;
        let mut has_atstart_flag = false;
        let mut has_onlys_flag = false;
        let mut has_only_flag = false;
        let mut has_grammar_flag = false;
        let mut stress_n = 0i32;
        let mut has_strend2_flag = false;
        let mut has_u2_flag = false;

        for f in parts.iter().skip(2) {
            match f.as_str() {
                "$noun" => {
                    has_noun_flag = true;
                }
                "$verb" => {
                    has_verb_flag = true;
                    has_grammar_flag = true;
                }
                "$past" => {}
                "$pastf" => {
                    has_pastf_flag = true;
                }
                "$nounf" => {
                    has_nounf_flag = true;
                    has_grammar_flag = true;
                }
                "$verbf" => {
                    has_verbf_flag = true;
                    has_grammar_flag = true;
                }
                "$atend" | "$allcaps" | "$sentence" => {
                    has_atend_flag = true;
                }
                "$capital" => {
                    has_capital_flag = true;
                }
                "$atstart" => {
                    has_atstart_flag = true;
                }
                "$strend2" => {
                    has_strend2_flag = true;
                    has_grammar_flag = true;
                }
                "$u2" => {
                    has_u2_flag = true;
                }
                "$u+" => {
                    self.unstressed_words.insert(norm_word.clone());
                    if phonemes_str.contains(',') && !phonemes_str.contains('\'') {
                        self.u_plus_secondary_words.insert(norm_word.clone());
                    }
                }
                "$u" => {
                    self.unstressed_words.insert(norm_word.clone());
                }
                "$unstressend" => {
                    self.unstressend_words.insert(norm_word.clone());
                }
                "$abbrev" => {
                    self.abbrev_words.insert(norm_word.clone());
                }
                "$only" => {
                    has_only_flag = true;
                }
                "$onlys" => {
                    has_onlys_flag = true;
                }
                _ => {}
            }

            if f.starts_with("$alt") && f.len() == 5
                && let Some(n) = f[4..].chars().next()
                    && ('1'..='6').contains(&n) {
                        let mask = 1 << (n as i32 - '1' as i32);
                        self.word_alt_flags
                            .entry(norm_word.clone())
                            .and_modify(|v| *v |= mask)
                            .or_insert(mask);
                    }

            if f.len() == 2
                && f.starts_with('$')
                && f.chars().nth(1).is_some_and(|c| ('1'..='6').contains(&c))
            {
                stress_n = f.chars().nth(1).unwrap() as i32 - '0' as i32;
            }
        }

        if phonemes_str.starts_with('$')
            && stress_n == 0
            && phonemes_str.len() == 2
            && phonemes_str
                .chars()
                .nth(1)
                .is_some_and(|c| ('1'..='6').contains(&c))
        {
            stress_n = phonemes_str.chars().nth(1).unwrap() as i32 - '0' as i32;
        }

        if phonemes_str == "$abbrev" {
            self.abbrev_words.insert(norm_word.clone());
        }

        if phonemes_str.starts_with('$')
            && phonemes_str.len() == 5
            && phonemes_str.starts_with("$alt")
            && let Some(n) = phonemes_str.chars().nth(4)
                && ('1'..='6').contains(&n) {
                    let mask = 1 << (n as i32 - '1' as i32);
                    self.word_alt_flags
                        .entry(norm_word.clone())
                        .and_modify(|v| *v |= mask)
                        .or_insert(mask);
                }

        if phonemes_str == "$verb"
            || phonemes_str == "$verbf"
            || phonemes_str == "$nounf"
            || phonemes_str == "$pastf"
            || phonemes_str == "$only"
        {
            has_grammar_flag = true;
        }

        if has_pastf_flag {
            self.pastf_words.insert(norm_word.clone());
        }
        if has_nounf_flag {
            self.nounf_words.insert(norm_word.clone());
        }
        if has_verbf_flag {
            self.verbf_words.insert(norm_word.clone());
        }

        if phonemes_str == "$u" || phonemes_str == "$u+" {
            self.unstressed_words.insert(norm_word.clone());
        }
        if phonemes_str == "$u" {
            has_grammar_flag = true;
        }
        if phonemes_str == "$verb" {
            has_verb_flag = true;
        }

        let is_flag_only = phonemes_str.starts_with('$');
        if stress_n > 0 && !has_noun_flag && !has_verb_flag && (!has_grammar_flag || is_flag_only) {
            self.stress_pos.entry(norm_word.clone()).or_insert(stress_n);
            if is_flag_only && has_onlys_flag {
                self.noun_form_stress.insert(norm_word.clone());
            }
        }

        if is_flag_only {
            if phonemes_str.len() == 5 && phonemes_str.starts_with("$alt")
                && let Some(n) = phonemes_str.chars().nth(4)
                    && ('1'..='6').contains(&n) {
                        self.dict.remove(&norm_word);
                    }
            if has_verb_flag {
                self.verb_flag_words.insert(norm_word.clone());
            }
            return Ok(());
        }

        if has_noun_flag {
            if !is_flag_only {
                self.noun_dict
                    .insert(norm_word.clone(), phonemes_str.clone());
            }
            return Ok(());
        }

        if has_verb_flag {
            if !is_flag_only {
                self.verb_dict
                    .insert(norm_word.clone(), phonemes_str.clone());
            }
            return Ok(());
        }

        if has_atend_flag {
            if !has_atstart_flag && !phonemes_str.is_empty() && !phonemes_str.starts_with('$') {
                self.atend_dict
                    .insert(norm_word.clone(), phonemes_str.clone());
            }
            return Ok(());
        }

        if has_capital_flag {
            if !phonemes_str.is_empty() && !phonemes_str.starts_with('$') {
                self.capital_dict
                    .insert(norm_word.clone(), phonemes_str.clone());
            }
            return Ok(());
        }

        if has_atstart_flag {
            self.atstart_dict
                .insert(norm_word.clone(), phonemes_str.clone());
            return Ok(());
        }

        if has_onlys_flag {
            if dialect_cond != 0 {
                self.dict.insert(norm_word.clone(), phonemes_str.clone());
                self.onlys_words.insert(norm_word.clone());
            } else if self
                .dict
                .insert(norm_word.clone(), phonemes_str.clone())
                .is_none()
            {
                self.onlys_words.insert(norm_word.clone());
            } else if !phonemes_str.is_empty() && !phonemes_str.starts_with('$') {
                self.onlys_bare_dict
                    .insert(norm_word.clone(), phonemes_str.clone());
            }
            return Ok(());
        }

        self.dict.insert(norm_word.clone(), phonemes_str.clone());

        if has_only_flag {
            self.only_words.insert(norm_word.clone());
        }

        if has_strend2_flag
            && norm_word.len() >= 2
            && !phonemes_str.is_empty()
            && !phonemes_str.starts_with(',')
            && !phonemes_str.starts_with('\'')
            && !phonemes_str.starts_with('%')
        {
            self.compound_prefixes
                .push((norm_word.clone(), phonemes_str.clone()));
            self.strend_words.insert(norm_word.clone());
        }

        if has_strend2_flag && !phonemes_str.is_empty() && phonemes_str.starts_with(',') {
            self.comma_strend2_words.insert(norm_word.clone());
        }

        if has_u2_flag && has_strend2_flag {
            self.u2_strend2_words.insert(norm_word.clone());
        }

        Ok(())
    }

    fn parse_phrase_entry(&mut self, line: &str, _is_en_us: bool) -> Result<(), ()> {
        if !line.starts_with('(') {
            return Err(());
        }

        if let Some(close_pos) = line.find(')')
            && close_pos > 1 {
                let phrase_words = Self::trim(&line[1..close_pos]);
                let rest = Self::trim(&line[close_pos + 1..]);

                if !rest.is_empty() && !rest.starts_with('$') {
                    let rp: Vec<&str> = rest.split_whitespace().collect();
                    if !rp.is_empty() && !rp[0].starts_with('$') {
                        let mut has_atend = false;
                        let mut has_pause = false;
                        let mut has_u2_plus = false;

                        for item in rp.iter().skip(1) {
                            if *item == "$atend" {
                                has_atend = true;
                            }
                            if *item == "$pause" {
                                has_pause = true;
                            }
                            if *item == "$u2+" {
                                has_u2_plus = true;
                            }
                        }

                        let words: Vec<&str> = phrase_words.split_whitespace().collect();
                        if words.len() == 2
                            && !has_atend
                            && !has_pause
                            && !words[0].contains('.')
                            && !words[1].contains('.')
                        {
                            let key = format!(
                                "{} {}",
                                Self::to_lower_ascii(words[0]),
                                Self::to_lower_ascii(words[1])
                            );

                            if rp[0].contains("||") {
                                if let Some(pipe_pos) = rp[0].find("||") {
                                    let p1 = rp[0][..pipe_pos].to_string();
                                    let p2 = rp[0][pipe_pos + 2..].to_string();
                                    self.phrase_split_dict.insert(key, (p1, p2));
                                }
                            } else {
                                let mut phoneme = rp[0].to_string();
                                if !phoneme.contains('\'') && !phoneme.starts_with('%') {
                                    phoneme = format!("%{}", phoneme);
                                }
                                self.phrase_dict.entry(key.clone()).or_insert(phoneme);
                                if has_u2_plus {
                                    self.keep_sec_phrase_keys.insert(key);
                                }
                            }
                        }
                    }
                }
            }

        Ok(())
    }

    fn load_rules(&mut self, path: &str) -> bool {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                self.error = format!("Cannot open rules file: {}: {}", path, e);
                return false;
            }
        };

        let reader = BufReader::new(file);
        self.load_rules_from_reader(reader)
    }

    fn load_rules_from_buff(&mut self, buff: &[u8]) -> bool {
        let reader = BufReader::new(Cursor::new(buff));
        self.load_rules_from_reader(reader)
    }

    fn load_rules_from_reader<R: BufRead>(&mut self, reader: R) -> bool {
        self.ruleset = RuleSet::new();

        let _is_en_us = self.dialect == "en-us" || self.dialect == "en_us";

        let mut current_group = String::new();
        let mut in_replace_section = false;

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };

            let comment_pos = line.find("//");
            let line = if let Some(pos) = comment_pos {
                &line[..pos]
            } else {
                &line
            };
            let line = Self::trim(line);
            if line.is_empty() {
                continue;
            }

            if line.starts_with('.') {
                if line.len() >= 2 && line.chars().nth(1) == Some('L') {
                    self.parse_lgroup_def(&line);
                    continue;
                } else if line == ".replace" {
                    in_replace_section = true;
                    current_group.clear();
                    continue;
                } else if let Some(stripped) = line.strip_prefix(".group") {
                    in_replace_section = false;
                    current_group = Self::trim(stripped);
                    continue;
                }
            }

            if in_replace_section {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    self.ruleset.replacements.push(ReplaceRule {
                        from: parts[0].to_string(),
                        to: parts[1].to_string(),
                    });
                }
                continue;
            }

            if current_group.is_empty() {
                continue;
            }

            let (dialect_cond, cond_negated, rule_line) = if let Some(stripped) = line.strip_prefix('?') {
                if let Some(space_pos) = stripped.find(|c: char| c.is_whitespace()) {
                    let cond_str = &stripped[..space_pos];
                    let (neg, cond_str) = if let Some(neg_stripped) = cond_str.strip_prefix('!') {
                        (true, neg_stripped)
                    } else {
                        (false, cond_str)
                    };
                    let dialect_cond: i32 = cond_str.parse().unwrap_or(0);
                    let rule_line = Self::trim(&line[space_pos + 2..]);
                    (dialect_cond, neg, rule_line)
                } else {
                    (0, false, line.clone())
                }
            } else {
                (0, false, line.clone())
            };

            if dialect_cond != 0 {
                let matches = dialect_cond == 3;
                let applies = if cond_negated { !matches } else { matches };
                if !applies {
                    continue;
                }
            }

            if let Some(rule) = self.parse_rule_line(&rule_line, &current_group) {
                if !rule.phonemes.is_empty() && rule.phonemes.starts_with('$') {
                    continue;
                }
                if rule.match_str.is_empty() {
                    continue;
                }

                self.ruleset
                    .rule_groups
                    .entry(current_group.clone())
                    .or_default()
                    .push(rule);
            }
        }

        true
    }

    fn parse_lgroup_def(&mut self, line: &str) {
        if line.len() < 3 || line.chars().nth(1) != Some('L') {
            return;
        }

        let mut id = 0i32;
        let mut i = 2usize;
        while i < line.len() {
            if let Some(c) = line.chars().nth(i) {
                if c.is_ascii_digit() {
                    id = id * 10 + c as i32 - '0' as i32;
                    i += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if id <= 0 || id >= 100 {
            return;
        }

        let rest = Self::trim(&line[i..]);
        let items: Vec<&str> = rest.split_whitespace().collect();

        let lgroup_idx = (id - 1) as usize;
        if lgroup_idx < self.ruleset.groups.lgroups.len() {
            for item in items {
                if item.starts_with("//") {
                    break;
                }
                self.ruleset.groups.lgroups[lgroup_idx].push(item.to_string());
            }
        }
    }

    fn parse_rule_line(&self, line: &str, default_group: &str) -> Option<PhonemeRule> {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            return None;
        }

        let mut rule = PhonemeRule::default();

        let mut ti = 0;

        if let Some(first) = tokens.first()
            && let Some(stripped) = first.strip_suffix(')') {
                rule.left_ctx = stripped.to_string();
                ti += 1;
            }

        if ti < tokens.len() && !tokens[ti].starts_with('(') {
            rule.match_str = tokens[ti].to_string();
            ti += 1;
        } else {
            rule.match_str = default_group.to_string();
        }

        if ti < tokens.len() && tokens[ti].starts_with('(') {
            rule.right_ctx = tokens[ti][1..].to_string();
            ti += 1;
        }

        for item in tokens.iter().skip(ti) {
            rule.phonemes.push_str(item);
        }

        for (k, rc) in rule.right_ctx.char_indices() {
            if rc == 'P' {
                let mut is_prefix_marker = false;
                if k + 1 >= rule.right_ctx.len() {
                    is_prefix_marker = true;
                } else {
                    let nc = rule.right_ctx.chars().nth(k + 1).unwrap();
                    if nc.is_ascii_digit() || nc == '_' || nc == '+' || nc == '<' {
                        is_prefix_marker = true;
                    }
                }
                if k > 0 && rule.right_ctx.chars().nth(k - 1) == Some('L') {
                    is_prefix_marker = false;
                }
                if is_prefix_marker {
                    rule.is_prefix = true;
                    break;
                }
            }
        }

        for (k, rc) in rule.right_ctx.char_indices() {
            if rc == 'S' && (k == 0 || rule.right_ctx.chars().nth(k - 1) != Some('L')) {
                let mut k2 = k + 1;
                let mut n = 0i32;
                while k2 < rule.right_ctx.len() {
                    if let Some(c) = rule.right_ctx.chars().nth(k2) {
                        if c.is_ascii_digit() {
                            n = n * 10 + c as i32 - '0' as i32;
                            k2 += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                let mut sflags = 0i32;
                while k2 < rule.right_ctx.len() {
                    if let Some(c) = rule.right_ctx.chars().nth(k2) {
                        if c.is_alphabetic() {
                            match c {
                                'i' => sflags |= 0x200,
                                'm' => sflags |= 0x80000,
                                'v' => sflags |= 0x800,
                                'e' => sflags |= 0x100,
                                'd' => sflags |= 0x1000,
                                'q' => sflags |= 0x4000,
                                'p' => sflags |= 0x400,
                                _ => {}
                            }
                            k2 += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if n > 0 {
                    rule.is_suffix = true;
                    rule.suffix_strip_len = n;
                    rule.suffix_flags = sflags;
                    break;
                }
            }
        }

        Some(rule)
    }

    pub fn phonemize_text(&self, text: &str) -> String {
        if !self.loaded {
            return String::new();
        }

        let normalized = self.normalize_text(text);
        let tokens = self.tokenize_text(&normalized);

        let mut result = String::new();
        let mut prev_word: Option<String> = None;
        let mut expect_past = false;
        let mut expect_noun = false;
        let mut i = 0;

        while i < tokens.len() {
            let token = &tokens[i];

            if token.needs_space_before && !result.is_empty() {
                result.push(' ');
            }

            if token.is_word {
                let word_lower = Self::to_lower_ascii(&token.text);
                let is_first = prev_word.is_none();
                let is_last = i + 1 >= tokens.len();

                // Check for phrase matches (bigram + next word)
                let mut phrase_phonemes: Option<String> = None;
                if let Some(ref prev) = prev_word {
                    let phrase_key = format!("{} {}", prev, word_lower);
                    
                    // Check phrase_split_dict first for split phoneme pairs
                    if let Some((_p1, p2)) = self.phrase_split_dict.get(&phrase_key) {
                        // Already processed first word, now add second part
                        phrase_phonemes = Some(p2.clone());
                    } else if let Some(p) = self.phrase_dict.get(&phrase_key) {
                        // Use phrase dictionary pronunciation
                        phrase_phonemes = Some(p.clone());
                    }
                }

                let phonemes = if let Some(ph) = phrase_phonemes {
                    ph
                } else {
                    // Use contextual dictionary lookups based on flags
                    if is_first {
                        // Check atstart_dict
                        if let Some(phonemes) = self.atstart_dict.get(&word_lower) {
                            phonemes.clone()
                        } else {
                            self.word_to_phonemes(&token.text)
                        }
                    } else if is_last {
                        // Check atend_dict
                        if let Some(phonemes) = self.atend_dict.get(&word_lower) {
                            phonemes.clone()
                        } else {
                            self.word_to_phonemes(&token.text)
                        }
                    } else if expect_past {
                        // Check verb_dict for past tense pronunciation
                        if let Some(phonemes) = self.verb_dict.get(&word_lower) {
                            phonemes.clone()
                        } else {
                            self.word_to_phonemes(&token.text)
                        }
                    } else if expect_noun {
                        // Check noun_dict for noun-specific pronunciation
                        if let Some(phonemes) = self.noun_dict.get(&word_lower) {
                            phonemes.clone()
                        } else {
                            self.word_to_phonemes(&token.text)
                        }
                    } else {
                        self.word_to_phonemes(&token.text)
                    }
                };

                let ipa = self.phonemes_to_ipa(&phonemes);
                let post_processed = self.post_process_phonemes(&token.text, &ipa);
                result.push_str(&post_processed);

                // Update context flags for next word
                expect_past = self.pastf_words.contains(&word_lower);
                expect_noun = self.nounf_words.contains(&word_lower);

                // Update prev_word for phrase matching
                prev_word = Some(word_lower);
            } else {
                result.push_str(&token.text);
                // Reset context flags for non-word tokens (punctuation, etc.)
                expect_past = false;
                expect_noun = false;
                prev_word = None;
            }

            i += 1;
        }

        result
    }

    fn normalize_text(&self, text: &str) -> String {
        let mut result = text.to_string();

        result = result
            .chars()
            .map(|c| if c.is_whitespace() { ' ' } else { c })
            .collect();

        result
    }

    fn tokenize_text(&self, text: &str) -> Vec<Token> {
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

    fn word_to_phonemes(&self, word: &str) -> String {
        let lower_word = Self::to_lower_ascii(word);

        if let Some(phonemes) = self.dict.get(&lower_word) {
            return phonemes.clone();
        }

        if let Some(phonemes) = self.atstart_dict.get(&lower_word) {
            return phonemes.clone();
        }

        if let Some(phonemes) = self.atend_dict.get(&lower_word) {
            return phonemes.clone();
        }

        if word.chars().next().is_some_and(|c| c.is_uppercase())
            && let Some(phonemes) = self.capital_dict.get(&lower_word) {
                return phonemes.clone();
            }

        let processed = self.apply_replacements(&lower_word);
        let phonemes = self.apply_rules(&processed);

        self.phonemes_to_ipa(&phonemes)
    }

    /// Like word_to_phonemes but returns raw phonemes without IPA conversion
    /// Used for suffix/prefix handling where we need to work with phoneme codes
    fn word_to_phonemes_internal(&self, word: &str) -> String {
        let lower_word = Self::to_lower_ascii(word);

        let raw_phonemes = if let Some(phonemes) = self.dict.get(&lower_word) {
            phonemes.clone()
        } else if let Some(phonemes) = self.atstart_dict.get(&lower_word) {
            phonemes.clone()
        } else if let Some(phonemes) = self.atend_dict.get(&lower_word) {
            phonemes.clone()
        } else if word.chars().next().is_some_and(|c| c.is_uppercase()) {
            if let Some(phonemes) = self.capital_dict.get(&lower_word) {
                phonemes.clone()
            } else {
                let processed = self.apply_replacements(&lower_word);
                self.apply_rules(&processed)
            }
        } else {
            let processed = self.apply_replacements(&lower_word);
            self.apply_rules(&processed)
        };

        // Apply stress processing to raw phonemes
        let processed_phonemes = self.process_phoneme_string(&lower_word, &raw_phonemes, false);

        self.phonemes_to_ipa(&processed_phonemes)
    }

    fn apply_replacements(&self, word: &str) -> String {
        let mut result = word.to_string();
        for rr in &self.ruleset.replacements {
            let mut pos = 0;
            while let Some(found) = result[pos..].find(&rr.from) {
                let actual_pos = pos + found;
                result = format!(
                    "{}{}{}",
                    &result[..actual_pos],
                    &rr.to,
                    &result[actual_pos + rr.from.len()..]
                );
                pos = actual_pos + rr.to.len();
            }
        }
        result
    }

    fn apply_rules(&self, word: &str) -> String {
        self.apply_rules_internal(word, false)
    }

    /// Internal rule application with suffix stripping control
    fn apply_rules_internal(&self, word: &str, allow_suffix_strip: bool) -> String {
        use crate::suffix_prefix::{PrefixHandler, SuffixFlags, SuffixHandler};

        let word_lower = Self::to_lower_ascii(word);
        let chars: Vec<char> = word_lower.chars().collect();
        let mut result = String::new();
        let mut pos = 0;

        while pos < chars.len() {
            let mut best_phoneme = String::new();
            let mut best_advance = 1;
            let mut best_score = -1i32;
            let mut best_rule: Option<PhonemeRule> = None;

            // Try 2-character match first
            if pos + 1 < chars.len() {
                let key: String = chars[pos..pos + 2].iter().collect();
                if let Some(rules) = self.ruleset.rule_groups.get(&key) {
                    for rule in rules {
                        if let Some((phonemes, advance, score)) =
                            self.match_rule_at_pos(rule, &chars, pos, &word_lower)
                            && score > best_score {
                                best_score = score;
                                best_phoneme = phonemes;
                                best_advance = advance;
                                best_rule = Some(rule.clone());
                            }
                    }
                }
            }

            // Try 1-character match
            if pos < chars.len() {
                let key: String = chars[pos].to_string();
                if let Some(rules) = self.ruleset.rule_groups.get(&key) {
                    for rule in rules {
                        if let Some((phonemes, advance, score)) =
                            self.match_rule_at_pos(rule, &chars, pos, &word_lower)
                        {
                            // For ties, prefer 2-char matches (already have higher score)
                            if score > best_score {
                                best_score = score;
                                best_phoneme = phonemes;
                                best_advance = advance;
                                best_rule = Some(rule.clone());
                            }
                        }
                    }
                }
            }

            // Handle suffix rules (e.g., -ing, -ed, -ly)
            if let Some(ref rule) = best_rule {
                if allow_suffix_strip
                    && rule.is_suffix
                    && SuffixHandler::is_suffix_match(rule, &word_lower, pos)
                {
                    // Extract stem by stripping suffix_strip_len chars from end
                    let stem = SuffixHandler::extract_stem(&word_lower, rule.suffix_strip_len);

                    // Check if stem is in only_words_ set - if so, don't strip suffix
                    if !self.only_words.contains(&stem) {
                        // Restore morphological changes to stem
                        let flags = SuffixFlags::from_bits(rule.suffix_flags);
                        let restored_stem = SuffixHandler::restore_stem(&stem, flags);

                        // Re-phonemize stem
                        let stem_phonemes = self.apply_rules_internal(&restored_stem, true);

                        // Apply -ed devoicing if needed
                        let suffix_phonemes = if flags.verb_form {
                            SuffixHandler::apply_ed_devoicing(&restored_stem, &best_phoneme)
                        } else {
                            best_phoneme.clone()
                        };

                        result.push_str(&stem_phonemes);
                        result.push_str(&suffix_phonemes);
                        return result;
                    }
                    // If stem is in only_words, fall through to normal processing
                }

                // Handle prefix rules (e.g., un-, re-, pre-)
                if PrefixHandler::is_prefix_match(rule, pos)
                    && !word_lower.is_empty()
                    && best_advance > 0
                {
                    let suffix = PrefixHandler::extract_suffix(&word_lower, best_advance);

                    // Re-phonemize suffix if it's not empty
                    if !suffix.is_empty() {
                        let suffix_phonemes = self.word_to_phonemes_internal(&suffix);

                        // Apply compound stress rules
                        let mut new_prefix = best_phoneme.clone();
                        let mut new_suffix = suffix_phonemes;

                        // Check for special prefix handling
                        // If word is in strend_words_, place primary stress on last vowel
                        if self.strend_words.contains(&word_lower) {
                            new_suffix = self.apply_final_stress(&new_suffix);
                        }
                        // Otherwise apply standard compound stress rules
                        else if self.comma_strend2_words.contains(&word_lower) {
                            // For comma_strend2_words_, keep secondary stress
                            // Don't modify the suffix stress
                        } else {
                            // Standard compound stress demotion
                            let (p, s) = PrefixHandler::apply_compound_stress(&new_prefix, &new_suffix);
                            new_prefix = p;
                            new_suffix = s;
                        }

                        result.push_str(&new_prefix);
                        result.push_str(&new_suffix);
                        return result;
                    }
                }
            }

            // If no rule matched, just copy the character
            if best_score < 0 {
                result.push(chars[pos]);
                pos += 1;
            } else {
                result.push_str(&best_phoneme);
                pos += best_advance;
            }
        }

        result
    }

    fn match_rule_at_pos(
        &self,
        rule: &PhonemeRule,
        chars: &[char],
        pos: usize,
        word: &str,
    ) -> Option<(String, usize, i32)> {
        // Check if rule matches at position
        let match_len = rule.match_str.len();
        if pos + match_len > chars.len() {
            return None;
        }

        // Match the main string (case-insensitive)
        for i in 0..match_len {
            if !chars[pos + i].eq_ignore_ascii_case(&rule.match_str.chars().nth(i)?) {
                return None;
            }
        }

        // Check left context
        if !self.check_left_context(rule, chars, pos) {
            return None;
        }

        // Check right context
        let (right_valid, del_fwd) = self.check_right_context(rule, chars, pos + match_len, word);
        if !right_valid {
            return None;
        }

        // If phoneme starts with $, it might be a flag-only entry
        if rule.phonemes.starts_with('$') && !rule.phonemes.contains('(') {
            return None;
        }

        let phoneme = rule.phonemes.clone();
        let advance = if del_fwd > 0 {
            match_len + del_fwd as usize
        } else {
            match_len
        };

        // Enhanced scoring formula:
        // base (1) + extra chars * 21 + context bonuses + rule-specific bonuses
        let extra_chars = (match_len as i32 - 1).max(0) * 21;
        let mut score = 1 + extra_chars;

        // Priority: 2-character matches get precedence over 1-character
        if match_len >= 2 {
            score += 100; // Strong priority for multi-char matches
        }

        // Bonus for matching at word start
        if pos == 0 {
            score += 5;
        }

        // Bonus for matching at word end
        if pos + match_len >= chars.len() {
            score += 3;
        }

        // Bonus for left context (presence of context increases confidence)
        if !rule.left_ctx.is_empty() {
            if rule.left_ctx.contains('_') {
                score += 20; // Strong signal for word boundary
            } else {
                score += 10; // Good signal for specific context
            }
        }

        // Bonus for right context
        if !rule.right_ctx.is_empty() {
            if rule.right_ctx.contains('_') {
                score += 20; // Strong signal for word boundary
            } else if !rule.right_ctx.contains('S') {
                score += 8; // Moderate signal for specific context
            }
        }

        // Bonus for vowel-sequence matching
        if match_len > 0 && self.ruleset.groups.group_a.contains(&chars[pos]) {
            // Check if this is a vowel sequence starter
            if rule.match_str.len() > 1 
                || (pos + 1 < chars.len() && self.ruleset.groups.group_a.contains(&chars[pos + 1])) {
                score += 5; // Vowel sequences are contextually important
            }
        }

        Some((phoneme, advance, score))
    }

    fn apply_stress_position(&self, phoneme_str: &str, pos: i32) -> String {
        // Place primary stress on the Nth vowel (1-based index)
        if pos <= 0 || phoneme_str.is_empty() {
            return phoneme_str.to_string();
        }

        let mut vowel_count = 0;
        let mut result = String::new();
        let mut target_vowel_pos = None;
        let chars: Vec<char> = phoneme_str.chars().collect();

        // First pass: find the target vowel position
        for (i, &ch) in chars.iter().enumerate() {
            if self.is_vowel_phoneme_char(ch) {
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

        // Second pass: reconstruct string with stress placement
        let mut prev_was_stress = false;
        for (i, &ch) in chars.iter().enumerate() {
            if i == target_pos {
                // Skip existing stress markers at this position
                if prev_was_stress && (ch == '\'' || ch == ',' || ch == '%') {
                    continue;
                }
                // Insert primary stress before the vowel
                if ch != '\'' && ch != ',' && ch != '%' {
                    result.push('\'');
                }
            }

            // Demote existing primary stress to secondary elsewhere
            if ch == '\'' && i != target_pos {
                result.push(',');
                prev_was_stress = true;
            } else {
                // Skip stress markers at target position
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

    fn is_vowel_phoneme_char(&self, ch: char) -> bool {
        // Check if a character represents a vowel in phoneme notation
        matches!(
            ch,
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'A' | 'E' | 'I' | 'O' | 'U' 
            | '@' | '3' | '0'
        )
    }

    fn apply_final_stress(&self, phoneme_str: &str) -> String {
        // Place primary stress on the last vowel in a phoneme string
        let mut result = String::new();
        let chars: Vec<char> = phoneme_str.chars().collect();
        
        // Find the position of the last vowel
        let mut last_vowel_pos = None;
        for (i, &ch) in chars.iter().enumerate() {
            if self.ruleset.groups.group_a.contains(&ch.to_ascii_lowercase()) 
                || matches!(ch, '@' | '3' | 'I' | 'U' | 'A' | 'O') {
                last_vowel_pos = Some(i);
            }
        }
        
        if let Some(pos) = last_vowel_pos {
            // Insert primary stress before the last vowel
            for (i, &ch) in chars.iter().enumerate() {
                if i == pos && ch != '\'' && ch != ',' {
                    result.push('\'');
                }
                // Skip any existing stress markers at this position
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

    fn process_phoneme_string(&self, word: &str, phoneme_str: &str, force_final_stress: bool) -> String {
        // Apply stress logic to phoneme string based on word properties
        // Handle: stress position dict, unstressed words, last-resort stress, final stress
        
        let mut result = phoneme_str.to_string();
        
        // Check if word has stress position specified in stress_pos dict
        if let Some(stress_pos) = self.stress_pos.get(word)
            && *stress_pos > 0 {
                result = self.apply_stress_position(&result, *stress_pos);
                return result;
            }
        
        // Check if word is in unstressed_words set
        if self.unstressed_words.contains(word) {
            // Remove primary stress and replace with unstressed marker
            result = result
                .replace('\'', "%")
                .replace(',', "");
            return result;
        }
        
        // Check for force final stress
        if force_final_stress && self.strend_words.contains(word) {
            result = self.apply_final_stress(&result);
            return result;
        }
        
        // Apply last-resort stress insertion
        // If no stress markers present, place primary stress on first vowel
        if !result.contains('\'') && !result.contains(',') && !result.contains('%') {
            // Check if word is abbreviation (should read as letters, no stress)
            if !self.abbrev_words.contains(word) {
                result = self.insert_last_resort_stress(&result);
            }
        }
        
        // Handle secondary stress maintenance for specific words
        if self.unstressend_words.contains(word) {
            // Keep secondary stress even at sentence end
            // (already preserved in the phoneme string)
        }
        
        if self.u_plus_secondary_words.contains(word) {
            // Maintain secondary stress (don't promote to primary)
            // (already preserved in the phoneme string)
        }
        
        result
    }

    fn insert_last_resort_stress(&self, phoneme_str: &str) -> String {
        // Insert primary stress on first vowel if no stress markers present
        let chars: Vec<char> = phoneme_str.chars().collect();
        let mut result = String::new();
        let mut inserted = false;

        for &ch in chars.iter() {
            if !inserted && self.is_vowel_phoneme_char(ch) {
                result.push('\'');
                inserted = true;
            }
            result.push(ch);
        }

        result
    }

    fn post_process_phonemes(&self, word: &str, ipa_str: &str) -> String {
        // Apply dialect-specific transformations and special handling
        let mut result = ipa_str.to_string();

        // Handle abbreviations - should be read as letter sequence
        if self.abbrev_words.contains(word) {
            // For abbreviations, output individual letters with no stress
            // This is typically handled by dictionary entries, but can be reinforced here
            result = result
                .replace("ˈ", "")
                .replace("ˌ", "")
                .replace("ˏ", "");
        }

        // Dialect-specific transformations
        if self.dialect == "en-us" || self.dialect == "en_us" {
            result = self.apply_american_english_rules(&result, word);
        } else if self.dialect == "en-gb" || self.dialect == "en_gb" {
            result = self.apply_british_english_rules(&result, word);
        }

        result
    }

    fn apply_american_english_rules(&self, ipa_str: &str, _word: &str) -> String {
        // American English: R is always pronounced (rhotic)
        // The IPA overrides already handle this with the rhotic r (ɹ)
        // Further adjustments can be made here if needed

        // American English vowel quality adjustments
        // These are mostly handled by IPA overrides already

        ipa_str.to_string()
    }

    fn apply_british_english_rules(&self, ipa_str: &str, _word: &str) -> String {
        // British English: R is only pronounced before vowels (non-rhotic)
        // This requires more complex logic that depends on word position and following segments
        // For now, this is handled primarily through dictionary entries and IPA overrides

        ipa_str.to_string()
    }

    #[allow(dead_code)]
    fn single_code_to_ipa(&self, code: &str) -> String {
        // Convert a single phoneme code to its IPA representation
        // Look up in ipa_overrides table and return mapped value
        // Return original code if not found in overrides
        
        if code.is_empty() {
            return String::new();
        }
        
        // Try exact match first in ipa_overrides
        if let Some(ipa) = self.ipa_overrides.get(code) {
            return ipa.clone();
        }
        
        // Fallback: return the code as-is (might be a valid IPA character already)
        code.to_string()
    }

    #[allow(dead_code)]
    fn is_vowel_code(&self, code: &str) -> bool {
        // Check if a phoneme code (e.g., "aI", "eI", "oU") represents a vowel
        // by looking in the IPA overrides table for vowel IPA characters
        
        if code.is_empty() {
            return false;
        }
        
        // Check if the code exists in ipa_overrides
        if let Some(ipa) = self.ipa_overrides.get(code) {
            // Check if the IPA character represents a vowel
            // Vowels end with vowel IPA characters: a, e, i, o, u, ə, æ, ɪ, etc.
            let last_char = ipa.chars().last().unwrap_or('\0');
            return matches!(
                last_char,
                'a' | 'e' | 'i' | 'o' | 'u' | 'y'
                    | 'ə' | 'æ' | 'ɪ' | 'ʊ' | 'ɐ' | 'ɑ' | 'ɔ' | 'ʌ' | 'ᴜ' | 'ᵻ'
            );
        }
        
        // Fallback: check if code itself contains vowel letters
        code.chars().any(|c| {
            matches!(
                c.to_ascii_lowercase(),
                'a' | 'e' | 'i' | 'o' | 'u' | 'y'
            )
        })
    }

    fn check_left_context(&self, rule: &PhonemeRule, chars: &[char], match_pos: usize) -> bool {
        // If no left context, it always matches
        if rule.left_ctx.is_empty() {
            return true;
        }

        // Check left context (scan backwards from match_pos - 1)
        let ctx = &rule.left_ctx;
        let word: String = chars.iter().collect();
        let mut char_pos = match_pos as i32 - 1;
        let ctx_chars: Vec<char> = ctx.chars().collect();
        let mut ctx_idx = ctx_chars.len() as i32 - 1;

        while ctx_idx >= 0 {
            let ctx_char = ctx_chars[ctx_idx as usize];

            match ctx_char {
                '_' => {
                    // Word boundary - must be at start of word
                    if char_pos >= 0 {
                        return false;
                    }
                    // Consume the underscore in context
                    ctx_idx -= 1;
                    if ctx_idx < 0 {
                        return true;
                    }
                    continue;
                }
                'A' | 'B' | 'C' | 'F' | 'G' | 'H' | 'Y' | 'K' => {
                    // Letter group
                    if char_pos < 0 {
                        return false;
                    }
                    if !self
                        .ruleset
                        .groups
                        .match_group(ctx_char, &word, char_pos as usize)
                    {
                        return false;
                    }
                    char_pos -= 1;
                }
                '\'' | '-' => {
                    // Special characters - match literally
                    if char_pos < 0 {
                        return false;
                    }
                    if chars[char_pos as usize] != ctx_char {
                        return false;
                    }
                    char_pos -= 1;
                }
                _ => {
                    // Literal character (case-insensitive)
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

    fn check_right_context(
        &self,
        rule: &PhonemeRule,
        chars: &[char],
        match_end: usize,
        _word: &str,
    ) -> (bool, i32) {
        // Check right context and calculate forward deletion count
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
                    // Suffix indicator - this is a suffix rule
                    // Signal for special handling (return false to skip generic suffix matching here)
                    return (true, 0);
                }
                '_' => {
                    // Word boundary - must be at end of word
                    if char_pos < chars.len() {
                        return (false, 0);
                    }
                    del_fwd += 1;
                }
                'A' | 'B' | 'C' | 'F' | 'G' | 'H' | 'Y' | 'K' => {
                    // Letter group
                    if char_pos >= chars.len() {
                        return (false, 0);
                    }
                    if !self
                        .ruleset
                        .groups
                        .match_group(ctx_char, &word, char_pos)
                    {
                        return (false, 0);
                    }
                    char_pos += 1;
                }
                '&' | '@' => {
                    // Compound operators for repeated groups
                    // & = zero or more consonants
                    // @ = one or more consonants
                    // For now, skip these (can be enhanced later)
                    let require_one = ctx_char == '@';
                    let mut count = 0;
                    
                    while char_pos < chars.len() 
                        && self.ruleset.groups.match_group('B', &word, char_pos) {
                        count += 1;
                        char_pos += 1;
                    }
                    
                    if require_one && count == 0 {
                        return (false, 0);
                    }
                }
                '\'' | '-' => {
                    // Special characters - match literally
                    if char_pos >= chars.len() {
                        return (false, 0);
                    }
                    if chars[char_pos] != ctx_char {
                        return (false, 0);
                    }
                    char_pos += 1;
                }
                _ => {
                    // Literal character (case-insensitive)
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

            // Try to match multi-character phoneme codes (up to 4 chars)
            let mut found_override = false;
            for len in (1..=4).rev() {
                if i + len <= chars.len() {
                    // Collect code characters into a string
                    let code: String = chars[i..i + len].iter().collect();
                    if let Some(ipa) = self.ipa_overrides.get(&code) {
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
}
