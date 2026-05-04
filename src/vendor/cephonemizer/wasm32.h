// Copyright 2024 - Apache 2.0 License
// WASM STL shim for wasm32-unknown-unknown.
//
// Provides wasm32:: aliases for portable STL types and WASM-safe
// replacements for OS-dependent I/O (file streams, stderr, getenv).
//
// On native builds types alias to std:: — zero overhead.
// On WASM, file I/O is backed by an in-process registry populated
// via wasm32::register_file() before the phonemizer is constructed.

#pragma once

#include <string>
#include <sstream>
#include <cstring>

#ifndef __wasm__
#  include <fstream>
#  include <iostream>
#  include <cstdlib>
#endif

// ===========================================================================
// File registry — present on both platforms.
//
// phonemizer_create_from_buff() uses the fixed names "__rules__" and
// "__list__" so callers only need to know those two keys.
// ===========================================================================
namespace wasm32 {

namespace detail {
    struct FileEntry { const char* name; const char* data; size_t size; };
    inline FileEntry registry[8]{};
    inline int registry_count = 0;

    inline const FileEntry* find(const char* name) noexcept {
        for (int i = 0; i < registry_count; ++i)
            if (registry[i].name && std::strcmp(registry[i].name, name) == 0)
                return &registry[i];
        return nullptr;
    }
}  // namespace detail

// Register a named in-memory file. Must be called before phonemizer_create.
// The data pointer must remain valid for the lifetime of the phonemizer.
inline void register_file(const char* name, const char* data, size_t size) noexcept {
    if (detail::registry_count < 8)
        detail::registry[detail::registry_count++] = {name, data, size};
}

// ===========================================================================
// wasm32::ifstream
//
// Looks up the path in the registry first.
// On native, falls back to a real std::ifstream when not found.
// On WASM, reports !is_open() when the name is unregistered.
// ===========================================================================
class ifstream {
    const char* buf_  = nullptr;
    size_t      len_  = 0;
    size_t      pos_  = 0;
    bool        open_ = false;
#ifndef __wasm__
    std::ifstream native_;
    bool          use_native_ = false;
#endif

public:
    explicit ifstream(const std::string& path) {
        const auto* e = detail::find(path.c_str());
        if (e) {
            buf_  = e->data;
            len_  = e->size;
            open_ = true;
        }
#ifndef __wasm__
        else {
            native_.open(path);
            use_native_ = native_.is_open();
            open_       = use_native_;
        }
#endif
    }

    bool is_open() const noexcept { return open_; }

    explicit operator bool() const noexcept {
#ifndef __wasm__
        if (use_native_) return static_cast<bool>(native_);
#endif
        return open_ && pos_ <= len_;
    }

    friend bool getline(ifstream& s, std::string& line);
};

// Reads one line from a wasm32::ifstream (strips \r, returns false at EOF).
inline bool getline(ifstream& s, std::string& line) {
    line.clear();
#ifndef __wasm__
    if (s.use_native_) return static_cast<bool>(std::getline(s.native_, line));
#endif
    if (!s.open_ || s.pos_ >= s.len_) return false;
    while (s.pos_ < s.len_) {
        char c = s.buf_[s.pos_++];
        if (c == '\n') return true;
        if (c != '\r') line += c;
    }
    return true;  // last line without a trailing newline is still valid
}

// ===========================================================================
// In-memory types — identical on both platforms
// ===========================================================================
using std::istringstream;

// ===========================================================================
// Debug I/O
// ===========================================================================
#ifdef __wasm__

// Null-sink stream: swallows all operator<< calls including manipulators.
struct null_ostream {
    template<typename T>
    constexpr null_ostream& operator<<(T const&) noexcept { return *this; }
};
inline null_ostream cerr;

// getenv has no meaning in bare WASM; debug paths are compiled out.
inline const char* getenv(const char*) noexcept { return nullptr; }

#else  // native

using std::cerr;
using std::getenv;

#endif  // __wasm__

}  // namespace wasm32

// ===========================================================================
// WASM host entry point — declared here; defined in bridge.cpp.
// The host (Rust / JS) calls this to inject file data before creating
// a phonemizer.
// ===========================================================================
#ifdef __wasm__
extern "C" void wasm32_register_file(const char* name, const char* data, size_t size);
#endif
