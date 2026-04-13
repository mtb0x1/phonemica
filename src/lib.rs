#[cfg(not(target_arch = "wasm32"))]
pub mod download;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
mod wasm_resources;

pub mod phonemizer;
pub mod suffix_prefix;
pub mod types;

#[cfg(not(target_arch = "wasm32"))]
pub use download::Downloader;
pub use phonemizer::IPAPhonemizer;
pub use phonemizer::error::{PhonemizeError, Result as PhonemizeResult};
pub use suffix_prefix::{PrefixHandler, SuffixFlags, SuffixHandler};
pub use types::*;
