use crate::phonemizer::IPAPhonemizer;
use crate::wasm_resources;
use wasm_bindgen::prelude::*;

/// Returns the embedded rules data  
#[inline(always)]
const fn get_rules_data() -> &'static [u8] {
    wasm_resources::RULES_DATA
}

/// Returns the embedded list data
#[inline(always)]
const fn get_list_data() -> &'static [u8] {
    wasm_resources::LIST_DATA
}

/// Gets the size of embedded rules in bytes
#[wasm_bindgen]
pub fn get_rules_size() -> usize {
    wasm_resources::RULES_DATA.len()
}

/// Gets the size of embedded list in bytes
#[wasm_bindgen]
pub fn get_list_size() -> usize {
    wasm_resources::LIST_DATA.len()
}

/// WASM wrapper for the IPAPhonemizer
#[wasm_bindgen]
pub struct Phonemizer {
    inner: IPAPhonemizer,
}

#[wasm_bindgen]
impl Phonemizer {

    /// Creates a new phonemizer instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Phonemizer, JsValue> {
        let phonemizer = IPAPhonemizer::from_buff(get_rules_data(), get_list_data(), "us-en");

        if !phonemizer.is_loaded() {
            let error_msg = phonemizer.get_error();
            return Err(JsValue::from_str(&format!(
                "Phonemizer error: {}",
                error_msg
            )));
        }

        Ok(Phonemizer { inner: phonemizer })
    }

    /// Creates a new phonemizer instance
    ///
    /// # Arguments
    /// * `rules_path` - Path or identifier for rules data
    /// * `list_path` - Path or identifier for list data
    /// * `dialect` - dialect
    pub fn new_with_custom(
        rules_path: String,
        list_path: String,
        dialect: String,
    ) -> Result<Phonemizer, JsValue> {
        let phonemizer = IPAPhonemizer::new(&rules_path, &list_path, &dialect);

        if !phonemizer.is_loaded() {
            let error_msg = phonemizer.get_error();
            return Err(JsValue::from_str(&format!(
                "Phonemizer error: {}",
                error_msg
            )));
        }

        Ok(Phonemizer { inner: phonemizer })
    }

    /// Phonemizes the given text
    ///
    /// # Arguments
    /// * `text` - The text to phonemize
    ///
    /// # Returns
    /// The phonemized text
    pub fn phonemize_text(&self, text: &str) -> String {
        self.inner.phonemize_text(text)
    }

    /// Gets the last error message
    pub fn get_error(&self) -> String {
        self.inner.get_error().into()
    }

    /// Checks if the phonemizer is properly loaded
    pub fn is_loaded(&self) -> bool {
        self.inner.is_loaded()
    }
}
