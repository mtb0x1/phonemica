// This binary generates test data for espeak_ng validation tests.
// It uses espeak-ng to generate expected phonemes for English text samples.
// Run with: cargo run --bin generate_test_data

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_data_path = Path::new("tests/espeak_ng_test_data.json");

    if test_data_path.exists() {
        println!(
            "Test data already exists at {}. Skipping generation.",
            test_data_path.display()
        );
        return Ok(());
    }

    println!("Generating test data...");

    // Install bundled espeak-ng data (English dictionary)
    let temp_dir = std::env::temp_dir().join("espeak-ng-test-data");
    std::fs::create_dir_all(&temp_dir)?;

    espeak_ng::install_bundled_languages(&temp_dir, &["en"])?;
    // SAFETY: This is safe because we only set this for this process
    unsafe { std::env::set_var("ESPEAK_DATA_PATH", &temp_dir) }

    println!("Installed espeak-ng data to {}", temp_dir.display());

    // Sample English text from open_subtitles dataset (common phrases)
    // These are representative of typical subtitle text
    let text_samples = vec![
        "hello world",
        "the quick brown fox jumps over the lazy dog",
        "how are you today",
        "i love learning english pronunciation",
        "what is your name",
        "nice to meet you",
        "good morning",
        "good afternoon",
        "good evening",
        "good night",
        "thank you very much",
        "you are welcome",
        "please and thank you",
        "excuse me",
        "i am sorry",
        "do you understand",
        "i understand",
        "i do not understand",
        "can you help me",
        "where is the bathroom",
        "i need some water",
        "the weather is nice today",
        "it is a beautiful day",
        "i like to read books",
        "she sells seashells by the seashore",
        "peter piper picked a peck of pickled peppers",
        "how much wood would a woodchuck chuck",
        "sheila ate thirty-six thimbles",
        "the rain in spain stays mainly in the plain",
        "red lorry yellow lorry",
        "unique new york",
        "the third thing",
        "i saw a kitten",
        "the cat is sleeping",
        "the dog barks loudly",
        "birds fly in the sky",
        "fish swim in the water",
        "children play in the park",
        "the sun rises in the east",
        "the moon shines at night",
        "stars twinkle in the darkness",
        "time flies like an arrow",
        "money does not grow on trees",
        "actions speak louder than words",
        "a penny for your thoughts",
        "break a leg",
        "bite the bullet",
        "burn the midnight oil",
        "catch my breath",
        "cut to the chase",
        "hit the nail on the head",
        "kill two birds with one stone",
        "let the cat out of the bag",
        "once in a blue moon",
        "over the moon",
        "piece of cake",
        "rain cats and dogs",
        "the ball is in your court",
        "the best things in life are free",
        "there is no such thing as a free lunch",
        "time is money",
        "when pigs fly",
        "you can not judge a book by its cover",
        "a bird in the hand is worth two in the bush",
        "an apple a day keeps the doctor away",
        "better late than never",
        "clean hands working",
        "do not count your chickens before they hatch",
        "every cloud has a silver lining",
        "fortune favors the bold",
        "good things come to those who wait",
        "home is where the heart is",
        "if at first you do not succeed try try again",
        "knowledge is power",
        "laughter is the best medicine",
        "life is what happens when you are busy making other plans",
        "never say never",
        "no pain no gain",
        "nothing is impossible",
        "practice makes perfect",
        "Rome was not built in a day",
        "seeing is believing",
        "slow and steady wins the race",
        "the early bird catches the worm",
        "the proof of the pudding is in the eating",
        "there is no place like home",
        "tomorrow is another day",
        "truth is stranger than fiction",
        "what goes up must come down",
        "you win some you lose some",
        "all that glitters is not gold",
        "beauty is in the eye of the beholder",
        "cannot see the forest for the trees",
        "do not put all your eggs in one basket",
        "easy come easy go",
        "every rose has its thorn",
        "follow your heart",
        "great minds think alike",
        "honesty is the best policy",
        "it is never too late to learn",
        "keep your chin up",
        "look on the bright side",
        "never give up",
        "one step at a time",
        "patience is a virtue",
        "where there is a will there is a way",
        "you are never too old to learn",
    ];

    let mut test_entries: Vec<serde_json::Value> = Vec::new();

    for text in text_samples {
        match espeak_ng::text_to_ipa("en", text) {
            Ok(phonemes) => {
                test_entries.push(serde_json::json!({
                    "text": text,
                    "expected": phonemes
                }));
            }
            Err(e) => {
                eprintln!("Warning: Failed to phonemize '{}': {}", text, e);
            }
        }
    }

    let json = serde_json::to_string_pretty(&test_entries)?;

    // Ensure tests directory exists
    if let Some(parent) = test_data_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(test_data_path)?;
    file.write_all(json.as_bytes())?;

    println!(
        "Generated {} test entries in {}",
        test_entries.len(),
        test_data_path.display()
    );

    Ok(())
}
