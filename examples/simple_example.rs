use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <rules_path> <list_path> [text...]", args[0]);
        eprintln!("Example: {} en_rules en_list \"Hello world\"", args[0]);
        exit(1);
    }

    let rules_path = &args[1];
    let list_path = &args[2];

    println!("Creating phonemizer with:");
    println!("  rules_path: {}", rules_path);
    println!("  list_path: {}", list_path);

    let phonemizer = phonemica::IPAPhonemizer::new(rules_path, list_path, "en-us");

    if !phonemizer.is_loaded() {
        eprintln!("Error: {}", phonemizer.get_error());
        exit(1);
    }

    println!("Phonemizer loaded successfully!");

    if args.len() > 3 {
        for i in 3..args.len() {
            let text = &args[i];
            println!("\nInput: {}", text);
            let result = phonemizer.phonemize_text(text);
            println!("IPA: {}", result);
        }
    } else {
        let test_texts = ["hello", "world", "kitten", "phonemizer", "test"];

        for text in test_texts.iter() {
            let result = phonemizer.phonemize_text(text);
            println!("{} -> {}", text, result);
        }
    }
}
