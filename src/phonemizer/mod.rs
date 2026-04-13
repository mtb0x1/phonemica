pub mod error;
pub mod r#impl;
pub mod dictionaries;
pub mod rules;
pub mod context;
pub mod stress;
pub mod ipa;
pub mod tokenizer;

pub use error::{PhonemizeError, Result};
pub use r#impl::IPAPhonemizer;
pub use dictionaries::{DictionarySet, DictionaryContext, WordFlags, StressFlags};
pub use rules::{RuleMatcher, ContextMatcher, RuleMatch, MatchScorer, RuleApplier};
pub use context::{ContextMatchingEngine, ContextGroups};
pub use stress::{StressConfig, StressPlace, StressProcessor, DefaultStressProcessor};
pub use ipa::{IpaConverter, AmericanEnglishConverter, BritishEnglishConverter};
pub use tokenizer::{Tokenizer, TextTokenizer};

// Module declarations (to be implemented in phases)
// pub mod dictionaries;
// pub mod rules;
// pub mod stress;
// pub mod ipa;
// pub mod context;
// pub mod tokenizer;
// pub mod core;

// Re-exports for public API (to be replaced during refactoring)
// pub use core::Phonemizer;
