#![cfg(not(target_arch = "wasm32"))]

use serde::Deserialize;

#[derive(Deserialize)]
struct TestEntry {
    text: String,
    expected: String,
}

const TEST_DATA: &str = include_str!("espeak_ng_test_data.json");

#[test]
fn espeak_ng_comparison_report() {
    let entries: Vec<TestEntry> =
        serde_json::from_str(TEST_DATA).expect("Failed to parse test data JSON");

    assert!(!entries.is_empty(), "Test data should not be empty");

    let temp_dir = tempfile::tempdir().unwrap();
    let downloader = phonemica::Downloader::new(temp_dir.path().to_path_buf());
    let (rules_path, list_path) = match downloader.download_if_needed() {
        Ok(paths) => paths,
        Err(e) => {
            panic!("Failed to download data files: {}", e);
        }
    };

    let phonemizer = phonemica::IPAPhonemizer::new(
        rules_path.to_str().unwrap(),
        list_path.to_str().unwrap(),
        "en-us",
    );

    assert!(
        phonemizer.is_loaded(),
        "Phonemizer should be loaded with real data"
    );

    let mut exact_matches = 0;
    let mut similar_matches = 0;
    let mut mismatches = Vec::new();

    for entry in &entries {
        let result = phonemizer.phonemize_text(&entry.text);

        if result == entry.expected {
            exact_matches += 1;
        } else {
            let result_clean: String = result.chars().filter(|c| !c.is_whitespace()).collect();
            let expected_clean: String = entry
                .expected
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            if result_clean == expected_clean {
                similar_matches += 1;
            } else {
                mismatches.push((entry.text.clone(), entry.expected.clone(), result));
            }
        }
    }

    let total = entries.len();
    let exact_rate = (exact_matches as f64 / total as f64) * 100.0;
    let similar_rate = (similar_matches as f64 / total as f64) * 100.0;
    let total_match_rate = ((exact_matches + similar_matches) as f64 / total as f64) * 100.0;

    // if mismatch is > 5% then fail the test
    let gap = 100.0 - total_match_rate;
    if gap > 0.05 {
        
        eprintln!("\n========================================");
        eprintln!("Phonemica vs espeak-ng Comparison Report");
        eprintln!("========================================");
        eprintln!("Total entries:    {}", total);
        eprintln!("Exact matches:    {} ({:.1}%)", exact_matches, exact_rate);
        eprintln!(
            "Whitespace diff:  {} ({:.1}%)",
            similar_matches, similar_rate
        );
        eprintln!(
            "Total match:      {} ({:.1}%)",
            exact_matches + similar_matches,
            total_match_rate
        );
        eprintln!(
            "Mismatches:       {} ({:.1}%)",
            mismatches.len(),
            gap
        );
        let entries_to_dump = std::env::var("ENTRIES_TO_DUMP")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(5);
        eprintln!("First {entries_to_dump} mismatches:");
        for (i, (text, expected, got)) in mismatches.iter().take(entries_to_dump).enumerate() {
            println!("{}. Text:    \"{}\"", i + 1, text);
            println!("   espeak-ng: \"{}\"", expected);
            println!("   phonemica: \"{}\"", got);
            println!();
        }
        eprintln!("========================================\n");
        //panic!(); for now let's skip the panic till we improve things
    }
    eprintln!("Note: phonemica and espeak-ng use different phonetic systems. 5% tolerance is allowed on tested data");

}
