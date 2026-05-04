use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

use super::error::{PhonemizeError, Result};

unsafe extern "C" {
    fn phonemizer_create(
        rules: *const c_char,
        list: *const c_char,
        dialect: *const c_char,
    ) -> *mut c_void;
    fn phonemizer_create_from_buff(
        rules_data: *const u8,
        rules_len: usize,
        list_data: *const u8,
        list_len: usize,
        dialect: *const c_char,
    ) -> *mut c_void;
    fn phonemizer_destroy(h: *mut c_void);
    fn phonemizer_phonemize(h: *mut c_void, text: *const c_char) -> *mut c_char;
    fn phonemizer_free_string(s: *mut c_char);
    fn phonemizer_get_error(h: *mut c_void) -> *const c_char;
}

/// Thin Rust wrapper around the C++ `IPAPhonemizer` bridge.
///
/// The underlying C++ object is heap-allocated via `phonemizer_create` /
/// `phonemizer_create_from_buff` and freed by `phonemizer_destroy` on drop.
/// A null handle means the phonemizer failed to load; use `is_loaded()` before
/// calling `phonemize_text`.
pub struct IPAPhonemizer(*mut c_void);

unsafe impl Send for IPAPhonemizer {}
unsafe impl Sync for IPAPhonemizer {}

impl IPAPhonemizer {
    /// Create a phonemizer by loading rule and list files from the filesystem.
    pub fn new(rules_path: &str, list_path: &str, dialect: &str) -> Self {
        let rules = CString::new(rules_path).expect("rules_path has no interior NUL");
        let list = CString::new(list_path).expect("list_path has no interior NUL");
        let dialect = CString::new(dialect).expect("dialect has no interior NUL");
        // SAFETY: all three pointers are valid, NUL-terminated C strings.
        let handle =
            unsafe { phonemizer_create(rules.as_ptr(), list.as_ptr(), dialect.as_ptr()) };
        Self(handle)
    }

    /// Create a phonemizer from in-memory byte slices (required for WASM where
    /// there is no filesystem).
    pub fn from_buff(rules_buff: &[u8], list_buff: &[u8], dialect: &str) -> Self {
        let dialect = CString::new(dialect).expect("dialect has no interior NUL");
        // SAFETY: slice pointers and lengths are valid; dialect is NUL-terminated.
        let handle = unsafe {
            phonemizer_create_from_buff(
                rules_buff.as_ptr(),
                rules_buff.len(),
                list_buff.as_ptr(),
                list_buff.len(),
                dialect.as_ptr(),
            )
        };
        Self(handle)
    }

    pub fn is_loaded(&self) -> bool {
        !self.0.is_null()
    }

    pub fn get_error(&self) -> &str {
        if self.0.is_null() {
            return "phonemizer failed to load";
        }
        // SAFETY: handle is non-null; returned pointer is valid for the lifetime of self.
        let ptr = unsafe { phonemizer_get_error(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap_or("(invalid UTF-8 in error)")
    }

    pub fn phonemize_text(&self, text: &str) -> String {
        if self.0.is_null() {
            return String::new();
        }
        let input = match CString::new(text) {
            Ok(s) => s,
            Err(_) => return String::new(),
        };
        // SAFETY: handle is non-null; input is a valid NUL-terminated string.
        let raw = unsafe { phonemizer_phonemize(self.0, input.as_ptr()) };
        if raw.is_null() {
            return String::new();
        }
        let result = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();
        // SAFETY: raw was allocated by the C++ bridge; free it here.
        unsafe { phonemizer_free_string(raw) };
        result
    }

    /// Convenience constructor returning `Result` for callers that prefer `?`.
    pub fn try_new(rules_path: &str, list_path: &str, dialect: &str) -> Result<Self> {
        let p = Self::new(rules_path, list_path, dialect);
        if p.is_loaded() {
            Ok(p)
        } else {
            Err(PhonemizeError::DictionaryLoad(p.get_error().to_owned()))
        }
    }
}

impl Drop for IPAPhonemizer {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: handle is non-null and was allocated by the C++ bridge.
            unsafe { phonemizer_destroy(self.0) };
        }
    }
}
