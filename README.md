# Buchikun

A Rust library for converting handle Japanese language/Okinawan language.

[日本語 (Japanese)](README.ja.md)

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
