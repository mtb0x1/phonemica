// Resources embedded at compile time for WASM targets
// These should be downloaded and placed in a resources/ directory during build
// See build.rs for download logic

pub const RULES_DATA: &[u8] = include_bytes!("../resources/en_rules");
pub const LIST_DATA: &[u8] = include_bytes!("../resources/en_list");
