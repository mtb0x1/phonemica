use core::panic;
use std::env;
use std::path::Path;
mod download {
    include!("src/download.rs");
}
use download::*;

fn main() {
    // Download resource files used by the phonemizer (rules + word list).
    let out_dir = "resources";
    let out_path = Path::new(out_dir);
    match Downloader::new(out_path.into()).download_if_needed() {
        Ok(_) => (),
        Err(e) => panic!("couldn't dl resources: {e}"),
    };

    // Compile the C++ cephonemizer library.
    //
    // For wasm32-wasip1 / wasm32-unknown-unknown the cc crate picks up the
    // target triple from $TARGET and uses the appropriate clang.  We add the
    // wasi-sdk sysroot explicitly when cross-compiling to wasm32 so that the
    // C++ standard library headers are found.
    let target = env::var("TARGET").unwrap_or_default();
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag("-std=c++17")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .file("src/vendor/cephonemizer/phonemizer.cpp")
        .file("src/vendor/cephonemizer/bridge.cpp");

    if target.starts_with("wasm32") {
        let wasi_sdk = std::env::var("WASI_SDK")
            .expect("WASI_SDK must be set when building for wasm32");
        let sysroot = std::env::var("WASI_SYS")
            .unwrap_or_else(|_| format!("{wasi_sdk}/share/wasi-sysroot"));
        //panic!(" {wasi_sdk} and {sysroot}");

        // Compile C++ as wasm32-wasip1 even though the Rust crate targets
        // wasm32-unknown-unknown.  The WASI sysroot is designed for wasip1
        // and resolves its own C/C++ include paths automatically for that
        // triple (include/wasm32-wasip1/noeh/c++/v1 etc.).  Using
        // wasm32-unknown-unknown here causes the locale machinery in libc++
        // to fail because it can't find the WASI rune-table definitions.
        //
        // WASM object files are ISA-compatible across WASI sub-targets; the
        // difference is only in the runtime system-call ABI, which our C++
        // code never exercises (all I/O goes through the wasm32.h shim).
        // Suppress cc-rs's default `-lstdc++`; we link libc++ from the WASI
        // SDK instead.  Use the noeh (no-exception-handling) variant to match
        // our -fno-exceptions flag.  libc++abi provides operator new/delete;
        // malloc/free come from Rust's dlmalloc in the wasm32-unknown-unknown
        // target so we do not need to link the WASI libc separately.
        let noeh_lib = format!("{sysroot}/lib/wasm32-wasip1/noeh");
        build
            .compiler(format!("{wasi_sdk}/bin/clang++"))
            .flag("--target=wasm32-wasip1")
            .flag(&format!("--sysroot={sysroot}"))
            .cpp_link_stdlib(None);

        println!("cargo:rustc-link-search=native={noeh_lib}");
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-lib=static=c++abi");
    }

    build.compile("cephonemizer");

    // Tell cargo to re-run this script if the C++ sources change.
    println!("cargo:rerun-if-changed=src/vendor/cephonemizer/phonemizer.cpp");
    println!("cargo:rerun-if-changed=src/vendor/cephonemizer/bridge.cpp");
    println!("cargo:rerun-if-changed=src/vendor/cephonemizer/phonemizer.h");
    println!("cargo:rerun-if-changed=src/vendor/cephonemizer/bridge.h");
    println!("cargo:rerun-if-changed=src/vendor/cephonemizer/wasm32.h");
}