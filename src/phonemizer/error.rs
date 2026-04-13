use std::fmt;
use std::io;

/// Error type for phonemizer operations
#[derive(Debug)]
pub enum PhonemizeError {
    /// Error loading dictionary files
    DictionaryLoad(String),
    /// Error loading rules files
    RulesLoad(String),
    /// Invalid dialect specification
    InvalidDialect(String),
    /// Invalid input provided
    InvalidInput(String),
    /// IO errors
    IoError(io::Error),
}

impl fmt::Display for PhonemizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhonemizeError::DictionaryLoad(msg) => write!(f, "Dictionary load error: {}", msg),
            PhonemizeError::RulesLoad(msg) => write!(f, "Rules load error: {}", msg),
            PhonemizeError::InvalidDialect(msg) => write!(f, "Invalid dialect: {}", msg),
            PhonemizeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            PhonemizeError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for PhonemizeError {}

impl From<io::Error> for PhonemizeError {
    fn from(err: io::Error) -> Self {
        PhonemizeError::IoError(err)
    }
}

/// Result type for phonemizer operations
pub type Result<T> = std::result::Result<T, PhonemizeError>;
