#[cfg(not(target_arch = "wasm32"))]
pub mod download;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
mod wasm_resources;

pub mod phonemizer;

#[cfg(not(target_arch = "wasm32"))]
pub use download::Downloader;
pub use phonemizer::IPAPhonemizer;
pub use phonemizer::error::{PhonemizeError, Result as PhonemizeResult};
