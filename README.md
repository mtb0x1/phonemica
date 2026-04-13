# Phonemica

Phonemica is an experimental phonemizer designed for English (US/GB) text-to-phoneme conversion. 

> [!WARNING]
> This project is currently under active testing and is considered **experimental**. It has been mostly "vibe coded" and may exhibit unpredictable behavior.

## Features

- **Multi-target Support**: Compiles to both **Native** and **WASM** targets.
- **eSpeak-ng Integration**: Leverages linguistic data from the [eSpeak-ng](https://github.com/espeak-ng/espeak-ng) project (specifically `en_rules` and `en_list`).
- **English Support**: Currently under testing for English (US/GB).

## Architecture

Phonemica is built in Rust and designed to be lightweight and portable. It includes modules for:
- Resource downloading (for native targets).
- WASM-specific resource management.
- Prefix and suffix handling for complex linguistic structures.

## Status

Testing and refined development for en (US/GB) is ongoing. Use with caution in production environments or don't :).
