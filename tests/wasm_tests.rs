#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

// Set test runner for WASM
wasm_bindgen_test_configure!(run_in_browser);

// ============================================================================
// WASM Phonemizer Initialization Tests
// ============================================================================

mod wasm_initialization {
    use super::*;
    use phonemica::wasm::Phonemizer;

    #[wasm_bindgen_test]
    fn test_wasm_phonemizer_initialization() {
        let result = Phonemizer::new();
        assert!(
            result.is_ok(),
            "Phonemizer should initialize successfully with embedded data"
        );
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemizer_custom_initialization() {
        // Note: These paths won't exist in WASM environment, so we expect failure
        let result = Phonemizer::new_with_custom(
            "invalid_rules".to_string(),
            "invalid_list".to_string(),
            "en-us".to_string(),
        );

        assert!(
            result.is_err(),
            "Should fail with invalid file paths"
        );
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemizer_is_loaded() {
        let phonemizer = Phonemizer::new().unwrap();
        assert!(
            phonemizer.is_loaded(),
            "Phonemizer should be loaded after initialization"
        );
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemizer_no_error_when_loaded() {
        let phonemizer = Phonemizer::new().unwrap();
        let error = phonemizer.get_error();
        assert!(
            error.is_empty(),
            "Should have no error message when loaded successfully"
        );
    }
}

// ============================================================================
// WASM Phonemization Core Functionality Tests
// ============================================================================

mod wasm_phonemization {
    use super::*;
    use phonemica::wasm::Phonemizer;

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_single_word() {
        let phonemizer = Phonemizer::new().unwrap();
        let result = phonemizer.phonemize("hello");

        assert!(
            !result.is_empty(),
            "Should produce phonemization for 'hello'"
        );
        web_sys::console::log_1(&format!("hello -> {}", result).into());
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_empty_string() {
        let phonemizer = Phonemizer::new().unwrap();
        let result = phonemizer.phonemize("");

        assert_eq!(result, "", "Empty input should produce empty output");
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_multiple_words() {
        let phonemizer = Phonemizer::new().unwrap();
        let result = phonemizer.phonemize("hello world");

        assert!(
            !result.is_empty(),
            "Should produce phonemization for multiple words"
        );
        web_sys::console::log_1(&format!("hello world -> {}", result).into());
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_with_punctuation() {
        let phonemizer = Phonemizer::new().unwrap();

        let test_cases = vec![
            "hello!",
            "world?",
            "phoneme...",
            "hello, world",
            "test-word",
        ];

        for test_case in test_cases {
            let result = phonemizer.phonemize(test_case);
            assert!(
                !result.is_empty(),
                "Should handle: '{}'",
                test_case
            );
        }
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_common_words() {
        let phonemizer = Phonemizer::new().unwrap();

        let common_words = vec![
            "hello",
            "world",
            "test",
            "word",
            "phoneme",
            "english",
            "language",
            "speech",
        ];

        for word in common_words {
            let result = phonemizer.phonemize(word);
            assert!(
                !result.is_empty(),
                "Should phonemize: '{}'",
                word
            );
            web_sys::console::log_1(&format!("{} -> {}", word, result).into());
        }
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_case_sensitivity() {
        let phonemizer = Phonemizer::new().unwrap();

        let lowercase = phonemizer.phonemize("hello");
        let uppercase = phonemizer.phonemize("HELLO");
        let mixed = phonemizer.phonemize("HeLLo");

        assert!(
            !lowercase.is_empty() && !uppercase.is_empty() && !mixed.is_empty(),
            "Should handle different cases"
        );
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_long_text() {
        let phonemizer = Phonemizer::new().unwrap();

        let long_text = 
            "The quick brown fox jumps over the lazy dog. \
             This is a longer test sentence with multiple words. \
             Let's see how the phonemizer handles extended text input.";

        let result = phonemizer.phonemize(long_text);

        assert!(
            !result.is_empty(),
            "Should phonemize longer text"
        );
        web_sys::console::log_1(&format!("Long text phonemization completed: {} chars -> {} chars", 
            long_text.len(), result.len()).into());
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_special_characters() {
        let phonemizer = Phonemizer::new().unwrap();

        let special_text = "café naïve résumé";
        let result = phonemizer.phonemize(special_text);

        assert!(
            !result.is_empty(),
            "Should handle special characters"
        );
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_consistency() {
        let phonemizer = Phonemizer::new().unwrap();

        let word = "philosophy";
        let result1 = phonemizer.phonemize(word);
        let result2 = phonemizer.phonemize(word);

        assert_eq!(
            result1, result2,
            "Same input should produce same output"
        );
    }
}

// ============================================================================
// WASM Resource Tests
// ============================================================================

mod wasm_resources {
    use super::*;
    use phonemica::wasm::{get_rules_size, get_list_size};

    #[wasm_bindgen_test]
    fn test_wasm_rules_size() {
        let rules_size = get_rules_size();
        assert!(rules_size > 0, "Rules data should be embedded and have non-zero size");
        web_sys::console::log_1(&format!("Rules data size: {} bytes", rules_size).into());
    }

    #[wasm_bindgen_test]
    fn test_wasm_list_size() {
        let list_size = get_list_size();
        assert!(list_size > 0, "List data should be embedded and have non-zero size");
        web_sys::console::log_1(&format!("List data size: {} bytes", list_size).into());
    }

    #[wasm_bindgen_test]
    fn test_wasm_resources_exist() {
        let rules_size = get_rules_size();
        let list_size = get_list_size();

        assert!(
            rules_size > 0 && list_size > 0,
            "Both rules and list resources should exist"
        );
        assert!(
            rules_size + list_size < 50_000_000,
            "Resources should be reasonably sized (< 50MB)"
        );
    }
}

// ============================================================================
// WASM Error Handling Tests
// ============================================================================

mod wasm_error_handling {
    use super::*;
    use phonemica::wasm::Phonemizer;

    #[wasm_bindgen_test]
    fn test_wasm_invalid_custom_paths_returns_error() {
        let result = Phonemizer::new_with_custom(
            "invalid_rules_path".to_string(),
            "invalid_list_path".to_string(),
            "en-us".to_string(),
        );

        assert!(result.is_err(), "Should return error for invalid paths");

        if let Err(js_err) = result {
            let error_msg = format!("{:?}", js_err);
            assert!(
                !error_msg.is_empty(),
                "Error should contain a message"
            );
        }
    }

    #[wasm_bindgen_test]
    fn test_wasm_default_phonemizer_always_works() {
        // The default phonemizer should always work since it uses embedded data
        for _ in 0..3 {
            let result = Phonemizer::new();
            assert!(result.is_ok(), "Default phonemizer should always initialize");
        }
    }
}

// ============================================================================
// WASM Integration Tests
// ============================================================================

mod wasm_integration {
    use super::*;
    use phonemica::wasm::Phonemizer;

    #[wasm_bindgen_test]
    fn test_wasm_multiple_phonemizer_instances() {
        let phonemizer1 = Phonemizer::new().unwrap();
        let phonemizer2 = Phonemizer::new().unwrap();

        let result1 = phonemizer1.phonemize("hello");
        let result2 = phonemizer2.phonemize("hello");

        assert_eq!(
            result1, result2,
            "Multiple instances should produce consistent results"
        );
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemizer_state_persistence() {
        let phonemizer = Phonemizer::new().unwrap();

        let word1 = "first";
        let word2 = "second";

        let result1 = phonemizer.phonemize(word1);
        let result2 = phonemizer.phonemize(word2);

        assert!(!result1.is_empty() && !result2.is_empty(),
            "Phonemizer state should persist across multiple calls");
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_batch_processing() {
        let phonemizer = Phonemizer::new().unwrap();

        let words = vec![
            "apple", "banana", "cherry", "date", "elderberry",
            "fig", "grape", "honeydew", "iguana", "jackfruit"
        ];

        let mut results = Vec::new();
        for word in &words {
            results.push(phonemizer.phonemize(word));
        }

        assert_eq!(
            results.len(),
            words.len(),
            "Should process all words"
        );

        for (i, result) in results.iter().enumerate() {
            assert!(!result.is_empty(), "Word {} should produce output", i);
        }

        web_sys::console::log_1(&format!("Batch processing complete: {} words", results.len()).into());
    }
}

// ============================================================================
// WASM Performance Tests
// ============================================================================

mod wasm_performance {
    use super::*;
    use phonemica::wasm::Phonemizer;

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_repeated_calls() {
        let phonemizer = Phonemizer::new().unwrap();
        let word = "telecommunications";

        // Perform multiple phonemizations
        for _ in 0..10 {
            let result = phonemizer.phonemize(word);
            assert!(!result.is_empty(), "Should phonemize consistently");
        }

        web_sys::console::log_1(&"Repeated calls test completed".into());
    }

    #[wasm_bindgen_test]
    fn test_wasm_phonemize_various_word_lengths() {
        let phonemizer = Phonemizer::new().unwrap();

        let words = vec![
            "a",
            "no",
            "yes",
            "apple",
            "hello",
            "beautiful",
            "pronunciation",
            "onomatopoeia",
            "pseudopseudohypoparathyroidism",
        ];

        for word in words {
            let result = phonemizer.phonemize(word);
            assert!(
                !result.is_empty(),
                "Should phonemize word of length {}",
                word.len()
            );
        }

        web_sys::console::log_1(&"Word length variation test completed".into());
    }
}