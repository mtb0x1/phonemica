// Copyright 2024 - Apache 2.0 License
// C API for the cephonemizer engine.
// Include this header in any consumer that calls the bridge functions.

#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef void* PhonemizerHandle;

// Create a phonemizer that reads rules and list from the filesystem.
// Returns NULL on failure (query phonemizer_get_error on a prior handle, or
// check your path arguments directly — the handle is freed on failure).
PhonemizerHandle phonemizer_create(const char* rules_path,
                                   const char* list_path,
                                   const char* dialect);

// Create a phonemizer from in-memory buffers.
// rules_data/list_data must remain valid until phonemizer_destroy().
// This is the only constructor available in WASM (no filesystem).
PhonemizerHandle phonemizer_create_from_buff(const char* rules_data, size_t rules_len,
                                              const char* list_data,  size_t list_len,
                                              const char* dialect);

void phonemizer_destroy(PhonemizerHandle handle);

// Phonemize text. Caller must free the returned string with phonemizer_free_string().
// Returns NULL on failure.
char* phonemizer_phonemize(PhonemizerHandle handle, const char* text);

void phonemizer_free_string(char* str);

// Returns a static error string from the last failed create, or "null handle".
const char* phonemizer_get_error(PhonemizerHandle handle);

#ifdef __cplusplus
}  // extern "C"
#endif
