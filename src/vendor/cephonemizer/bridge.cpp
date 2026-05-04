// Copyright 2024 - Apache 2.0 License
// C bridge for the phonemizer engine.
// Compiled with -fno-exceptions: no try/catch anywhere in this file.

#include "phonemizer.h"
#include "wasm32.h"
#include "bridge.h"
#include <cstring>
#include <cstdlib>
#include <new>

extern "C" {

PhonemizerHandle phonemizer_create(const char* rules_path,
                                    const char* list_path,
                                    const char* dialect) {
    std::string d = dialect ? std::string(dialect) : "en-us";
    auto* p = new(std::nothrow) IPAPhonemizer(
        rules_path ? std::string(rules_path) : std::string(),
        list_path  ? std::string(list_path)  : std::string(),
        d);
    if (!p || !p->isLoaded()) {
        delete p;
        return nullptr;
    }
    return static_cast<PhonemizerHandle>(p);
}

// Fixed registry keys used by phonemizer_create_from_buff.
static constexpr const char* kRulesKey = "__rules__";
static constexpr const char* kListKey  = "__list__";

PhonemizerHandle phonemizer_create_from_buff(const char* rules_data, size_t rules_len,
                                              const char* list_data,  size_t list_len,
                                              const char* dialect) {
    wasm32::register_file(kRulesKey, rules_data, rules_len);
    wasm32::register_file(kListKey,  list_data,  list_len);
    return phonemizer_create(kRulesKey, kListKey, dialect);
}

void phonemizer_destroy(PhonemizerHandle handle) {
    delete static_cast<IPAPhonemizer*>(handle);
}

char* phonemizer_phonemize(PhonemizerHandle handle, const char* text) {
    if (!handle || !text) return nullptr;
    auto* p = static_cast<IPAPhonemizer*>(handle);
    std::string result = p->phonemizeText(text);
    char* out = static_cast<char*>(malloc(result.size() + 1));
    if (!out) return nullptr;
    memcpy(out, result.c_str(), result.size() + 1);
    return out;
}

void phonemizer_free_string(char* str) {
    free(str);
}

const char* phonemizer_get_error(PhonemizerHandle handle) {
    if (!handle) return "null handle";
    return static_cast<IPAPhonemizer*>(handle)->getError().c_str();
}

// WASM host entry point: inject a named file into the registry before
// calling phonemizer_create or phonemizer_create_from_buff.
#ifdef __wasm__
void wasm32_register_file(const char* name, const char* data, size_t size) {
    wasm32::register_file(name, data, size);
}
#endif

}  // extern "C"
