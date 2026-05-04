// Copyright 2024 - Apache 2.0 License
// Native test driver for cephonemizer (used by `just test`).

#include "bridge.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>

static int failures = 0;

static void check(bool cond, const char* msg) {
    if (!cond) { std::fprintf(stderr, "FAIL: %s\n", msg); ++failures; }
}

static void test_basic(const char* rules_path, const char* list_path) {
    PhonemizerHandle h = phonemizer_create(rules_path, list_path, "en-us");
    check(h != nullptr, "phonemizer_create returned null");
    if (!h) return;

    const char* words[] = {"hello", "world", "the", "quick", "brown", "fox"};
    for (const char* w : words) {
        char* out = phonemizer_phonemize(h, w);
        check(out && out[0] != '\0', w);
        if (out) {
            std::printf("  %-20s -> %s\n", w, out);
            phonemizer_free_string(out);
        }
    }
    phonemizer_destroy(h);
}

static void test_bad_paths() {
    PhonemizerHandle h = phonemizer_create("no_such_rules", "no_such_list", "en-us");
    check(h == nullptr, "bad paths should return null handle");
    if (h) phonemizer_destroy(h);
}

static void test_from_buff(const char* rules_path, const char* list_path) {
    // Read both files into memory, then exercise the buffer constructor.
    auto read_file = [](const char* path, size_t* out_len) -> char* {
        FILE* f = std::fopen(path, "rb");
        if (!f) return nullptr;
        std::fseek(f, 0, SEEK_END);
        long sz = std::ftell(f);
        std::rewind(f);
        char* buf = static_cast<char*>(std::malloc(sz));
        if (buf) std::fread(buf, 1, sz, f);
        std::fclose(f);
        *out_len = static_cast<size_t>(sz);
        return buf;
    };

    size_t rlen = 0, llen = 0;
    char* rdata = read_file(rules_path, &rlen);
    char* ldata = read_file(list_path,  &llen);
    check(rdata && ldata, "could not read resource files for from_buff test");
    if (!rdata || !ldata) { free(rdata); free(ldata); return; }

    PhonemizerHandle h = phonemizer_create_from_buff(rdata, rlen, ldata, llen, "en-us");
    check(h != nullptr, "phonemizer_create_from_buff returned null");
    if (h) {
        char* out = phonemizer_phonemize(h, "hello");
        check(out && out[0] != '\0', "from_buff phonemize hello");
        if (out) {
            std::printf("  from_buff: hello -> %s\n", out);
            phonemizer_free_string(out);
        }
        phonemizer_destroy(h);
    }

    std::free(rdata);
    std::free(ldata);
}

int main(int argc, char* argv[]) {
    if (argc < 3) {
        std::fprintf(stderr, "usage: test_native <rules_path> <list_path>\n");
        return 1;
    }
    const char* rules = argv[1];
    const char* list  = argv[2];

    std::printf("--- bad paths ---\n");
    test_bad_paths();

    std::printf("--- basic phonemization ---\n");
    test_basic(rules, list);

    std::printf("--- from_buff ---\n");
    test_from_buff(rules, list);

    if (failures == 0) {
        std::printf("ALL TESTS PASSED\n");
        return 0;
    }
    std::fprintf(stderr, "%d TEST(S) FAILED\n", failures);
    return 1;
}
