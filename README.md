# POKER FACE - ♥️♦️♣️♠️

[<img alt="github" src="https://img.shields.io/badge/github-davassi/davassi?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/davassi/poker-face)
[<img alt="build status" src="https://github.com/davassi/poker-face/actions/workflows/rust.yml/badge.svg" height="20">](https://github.com/davassi/poker-face/actions?query=branch%3Amaster)
[<img alt="crates.io" src="https://img.shields.io/crates/v/poker-face.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/poker-face)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/poker-face?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/poker-face)
[![Downloads](https://img.shields.io/crates/d/poker-face.svg)](https://crates.io/crates/poker-face)
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)

A fast and idiomatic Rust implementation of a Texas Hold'em poker hand evaluator. This library showcases Rust's powerful pattern matching capabilities by evaluating 5-card poker hands using native `match` control flow with array and struct patterns.

## Features

- **Pattern-Driven Evaluation**: Uses Rust's `match` expressions with array and struct patterns for clean, readable hand evaluation
- **Comprehensive Hand Rankings**: Supports all standard poker hands from High Card to Royal Flush
- **Convenient Macros**: Ergonomic macros (`hand!`, `newcard!`, `assert_rank!`) for working with cards and hands
- **CLI Tool Included**: Interactive command-line interface for testing and demonstrations
- **Well-Tested**: 28+ unit tests covering edge cases and hand ranking scenarios
- **Zero-Cost Abstractions**: Fast evaluation using Rust's compile-time optimizations

See the [MatchHandEvaluator](https://github.com/davassi/poker-face/blob/master/src/match_evaluator.rs) implementation for details on the pattern-matching approach.

## Installation

Add poker-face to your `Cargo.toml`:

```toml
[dependencies]
poker-face = "0.2.0"
```

Or use cargo:

```bash
cargo add poker-face
```

## Quick Start

### Using as a Library

The library provides convenient macros for working with poker hands:

```rust
use pokerface::{hand, newcard, assert_rank, Rank, Card, Suit};

fn main() {
    // Create and evaluate hands using the hand! macro
    let royal = hand!["Ad","Kd","Qd","Jd","10d"];
    assert_eq!(Rank::evaluate(&royal), Rank::RoyalFlush);

    // Create individual cards
    let ace_hearts = newcard!["Ah"];
    assert_eq!(ace_hearts, Card::new(14, Suit::Hearts));

    // Test hand rankings
    assert_rank!(hand!["Kd", "Kh", "Kc", "Ks", "Qd"], Rank::FourOfAKind);
    assert_rank!(hand!["2d", "2h", "Qc", "Qs", "Qd"], Rank::FullHouse);
}
```

### Using the CLI Tool

The package includes an interactive command-line tool for demonstrations:

```bash
# Run directly with cargo
cargo run -q

# Or build and install
cargo build --release
cargo install --path .
pokerface
```

Example output:

```
Poker Face - 🦀 for ♠️ ♣️ ♥️ ♦️

1. Let's shuffle a deck...
2. Let's take 2 hands of 5 cards each from the deck
   Player 1 has:  J♥️   10♥️  9♠️   7♠️   5♥️
   Player 2 has:  K♥️   K♣️   K♦️   Q♦️   6♥️

3. Let's evaluate the hands...
   Player 1 has a [HighCard with a highcard of value J♥️]
   Player 2 has a [ThreeOfAKind]

4. Celebrate the winner:
   The winner is Player 2!
```

## Supported Hand Rankings

The evaluator recognizes all standard Texas Hold'em poker hands:

- **Royal Flush**: A♠ K♠ Q♠ J♠ 10♠
- **Straight Flush**: 9♥ 8♥ 7♥ 6♥ 5♥
- **Four of a Kind**: K♦ K♣ K♥ K♠ Q♦
- **Full House**: Q♠ Q♥ Q♦ 3♣ 3♦
- **Flush**: A♣ J♣ 8♣ 4♣ 3♣
- **Straight**: 10♦ 9♠ 8♥ 7♣ 6♦
- **Three of a Kind**: 7♥ 7♦ 7♠ K♣ 2♠
- **Two Pair**: J♠ J♦ 4♥ 4♣ 9♠
- **One Pair**: 10♥ 10♣ A♠ 5♦ 3♥
- **High Card**: A♦ K♣ 8♥ 5♠ 2♦

## How It Works

Poker-Face uses Rust's pattern matching to evaluate hands in a readable and efficient way. Instead of using lookup tables or bitwise operations, it leverages Rust's structural pattern matching on arrays and structs. This approach makes the code:

- **Readable**: Each hand type is clearly expressed as a pattern
- **Maintainable**: Easy to understand and modify
- **Type-Safe**: Compile-time guarantees prevent errors
- **Performant**: Zero-cost abstractions mean no runtime overhead

Check out the [MatchHandEvaluator](https://github.com/davassi/poker-face/blob/master/src/match_evaluator.rs) source to see pattern matching in action.

## API Documentation

Full API documentation is available on [docs.rs](https://docs.rs/poker-face).

Key types and functions:
- `Card`: Represents a playing card with rank and suit
- `Rank`: Enum representing all poker hand rankings
- `hand!`: Macro for creating hands from string notation
- `newcard!`: Macro for creating individual cards
- `assert_rank!`: Test macro for validating hand rankings

## Contributing

Contributions are welcome! If you have suggestions for new features or find any issues, please:

1. Check existing [issues](https://github.com/davassi/poker-face/issues) first
2. Open a new issue to discuss your idea or bug report
3. Submit a pull request with your changes

All feedback is appreciated!

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

Built with Rust 🦀 for poker enthusiasts ♠️ ♣️ ♥️ ♦️