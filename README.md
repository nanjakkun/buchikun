# Buchikun

A Rust library for converting handle Japanese/Okinawan language.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
buchikun = "0.1.0"
```

## Usage

```rust
use buchikun::core::romaji_to_kana;

let result = romaji_to_kana("konnichiha");
assert_eq!(result, "こんにちは");
```

## Testing

You can run the unit tests using `cargo test`:

```bash
cargo test
```
