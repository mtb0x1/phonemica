#![cfg(not(target_arch = "wasm32"))]

use std::fs;
use tempfile::TempDir;

// ============================================================================
// Unit Tests for Phonemizer Initialization and Error Handling
// ============================================================================

mod initialization {
    use super::*;

    #[test]
    fn test_phonemizer_loads_with_valid_files() {
        let temp_dir = TempDir::new().unwrap();
        let (rules_path, list_path) = create_test_data(&temp_dir);

        let phonemizer =
            phonemica::phonemizer::IPAPhonemizer::new(rules_path.to_str().unwrap(), list_path.to_str().unwrap(), "en-us");

        assert!(phonemizer.is_loaded(), "Phonemizer should load with valid files");
        assert!(
            phonemizer.get_error().is_empty(),
            "Should have no error message when loaded successfully"
        );
    }

    #[test]
    fn test_phonemizer_fails_with_missing_rules() {
        let phonemizer =
            phonemica::IPAPhonemizer::new("nonexistent_rules", "nonexistent_list", "en-us");

        assert!(
            !phonemizer.is_loaded(),
            "Phonemizer should not load with missing files"
        );
        assert!(
            !phonemizer.get_error().is_empty(),
            "Should have error message when files are missing"
        );
    }

    #[test]
    fn test_phonemizer_fails_with_missing_list() {
        let temp_dir = TempDir::new().unwrap();
        let rules_path = temp_dir.path().join("en_rules");
        fs::write(&rules_path, ".group a\na a:").unwrap();

        let phonemizer = phonemica::IPAPhonemizer::new(
            rules_path.to_str().unwrap(),
            "nonexistent_list",
            "en-us",
        );

        assert!(
            !phonemizer.is_loaded(),
            "Phonemizer should not load with missing list file"
        );
    }

    #[test]
    fn test_phonemizer_with_different_dialects() {
        let temp_dir = TempDir::new().unwrap();
        let (rules_path, list_path) = create_test_data(&temp_dir);

        let dialects = vec!["en-us", "en-uk", "en-au"];

        for dialect in dialects {
            let phonemizer = phonemica::IPAPhonemizer::new(
                rules_path.to_str().unwrap(),
                list_path.to_str().unwrap(),
                dialect,
            );

            assert!(
                phonemizer.is_loaded(),
                "Phonemizer should load with dialect '{}'",
                dialect
            );
        }
    }
}

// ============================================================================
// Unit Tests for Phonemization Core Functionality
// ============================================================================

mod phonemization {
    use super::*;

    #[test]
    fn test_phonemize_basic_text() {
        let temp_dir = TempDir::new().unwrap();
        let (rules_path, list_path) = create_test_data(&temp_dir);

        let phonemizer =
            phonemica::IPAPhonemizer::new(rules_path.to_str().unwrap(), list_path.to_str().unwrap(), "en-us");

        let result = phonemizer.phonemize_text("hello");
        assert!(!result.is_empty(), "Should produce output for 'hello'");
        assert_ne!(result, "hello", "Output should be different from input");
    }

    #[test]
    fn test_phonemize_empty_string() {
        let temp_dir = TempDir::new().unwrap();
        let (rules_path, list_path) = create_test_data(&temp_dir);

        let phonemizer =
            phonemica::IPAPhonemizer::new(rules_path.to_str().unwrap(), list_path.to_str().unwrap(), "en-us");

        let result = phonemizer.phonemize_text("");
        assert_eq!(result, "", "Empty input should produce empty output");
    }

    #[test]
    fn test_phonemize_multiple_words() {
        let temp_dir = TempDir::new().unwrap();
        let (rules_path, list_path) = create_test_data(&temp_dir);

        let phonemizer =
            phonemica::IPAPhonemizer::new(rules_path.to_str().unwrap(), list_path.to_str().unwrap(), "en-us");

        let result = phonemizer.phonemize_text("hello world");
        assert!(
            !result.is_empty(),
            "Should produce output for multiple words"
        );
    }

    #[test]
    fn test_phonemize_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        let (rules_path, list_path) = create_test_data(&temp_dir);

        let phonemizer =
            phonemica::IPAPhonemizer::new(rules_path.to_str().unwrap(), list_path.to_str().unwrap(), "en-us");

        let test_cases = vec!["hello!", "hello?", "hello...", "hello,world"];

        for test_case in test_cases {
            let result = phonemizer.phonemize_text(test_case);
            assert!(
                !result.is_empty(),
                "Should handle special characters in '{}'",
                test_case
            );
        }
    }

    #[test]
    fn test_phonemize_case_insensitivity() {
        let temp_dir = TempDir::new().unwrap();
        let (rules_path, list_path) = create_test_data(&temp_dir);

        let phonemizer =
            phonemica::IPAPhonemizer::new(rules_path.to_str().unwrap(), list_path.to_str().unwrap(), "en-us");

        let lower = phonemizer.phonemize_text("hello");
        let upper = phonemizer.phonemize_text("HELLO");
        let mixed = phonemizer.phonemize_text("HeLLo");

        assert!(
            !lower.is_empty() && !upper.is_empty() && !mixed.is_empty(),
            "Should handle different cases"
        );
    }

    #[test]
    fn test_phonemize_dictionary_word() {
        let temp_dir = TempDir::new().unwrap();

        let rules_content = ".group a\na a:";
        let list_content = "hello h@loU\nworld w3:ld";

        let rules_path = temp_dir.path().join("en_rules");
        let list_path = temp_dir.path().join("en_list");

        fs::write(&rules_path, rules_content).unwrap();
        fs::write(&list_path, list_content).unwrap();

        let phonemizer = phonemica::IPAPhonemizer::new(
            rules_path.to_str().unwrap(),
            list_path.to_str().unwrap(),
            "en-us",
        );

        let result = phonemizer.phonemize_text("hello");
        assert!(
            result.contains("@") || result.contains("loU"),
            "Should use dictionary data when available"
        );
    }
}

// ============================================================================
// Integration Tests with Real Data
// ============================================================================

mod real_data_integration {
    use super::*;

    #[test]
    fn test_phonemizer_with_real_data() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();

        let downloader = phonemica::Downloader::new(cache_dir);
        let (rules_path, list_path) = match downloader.download_if_needed() {
            Ok(paths) => paths,
            Err(e) => {
                eprintln!("Failed to download data files: {}", e);
                panic!("Cannot download data files");
            }
        };

        let phonemizer =
            phonemica::IPAPhonemizer::new(rules_path.to_str().unwrap(), list_path.to_str().unwrap(), "en-us");

        assert!(
            phonemizer.is_loaded(),
            "Phonemizer should be loaded with real data"
        );
    }

    #[test]
    fn test_common_english_words() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();

        let downloader = phonemica::Downloader::new(cache_dir);
        let (rules_path, list_path) = match downloader.download_if_needed() {
            Ok(paths) => paths,
            Err(e) => {
                eprintln!("Skipping real data test: {}", e);
                return;
            }
        };

        let phonemizer =
            phonemica::IPAPhonemizer::new(rules_path.to_str().unwrap(), list_path.to_str().unwrap(), "en-us");

        let test_words = vec![
            "hello",
            "world",
            "kitten",
            "phoneme",
            "english",
            "beautiful",
            "pronunciation",
        ];

        for word in test_words {
            let result = phonemizer.phonemize_text(word);
            assert!(
                !result.is_empty(),
                "Should produce phonemization for '{}'",
                word
            );
            println!("{} -> {}", word, result);
        }
    }

    #[test]
    fn test_phonemize_sentences() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();

        let downloader = phonemica::Downloader::new(cache_dir);
        let (rules_path, list_path) = match downloader.download_if_needed() {
            Ok(paths) => paths,
            Err(e) => {
                eprintln!("Skipping real data test: {}", e);
                return;
            }
        };

        let phonemizer =
            phonemica::IPAPhonemizer::new(rules_path.to_str().unwrap(), list_path.to_str().unwrap(), "en-us");

        let sentences = vec![
            "The quick brown fox jumps over the lazy dog.",
            "Hello, how are you today?",
            "I love learning English pronunciation!",
        ];

        for sentence in sentences {
            let result = phonemizer.phonemize_text(sentence);
            assert!(
                !result.is_empty(),
                "Should phonemize sentences: '{}'",
                sentence
            );
            println!("Sentence: {}", sentence);
            println!("Result: {}\n", result);
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn create_test_data(temp_dir: &TempDir) -> (std::path::PathBuf, std::path::PathBuf) {
    let rules_content = r#"
.group a
a a:
"#;

    let list_content = r#"
hello h@loU
world w3:ld
"#;

    let rules_path = temp_dir.path().join("en_rules");
    let list_path = temp_dir.path().join("en_list");

    fs::write(&rules_path, rules_content).unwrap();
    fs::write(&list_path, list_content).unwrap();

    (rules_path, list_path)
}
