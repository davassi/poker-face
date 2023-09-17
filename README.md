POKER FACE - ♥️♦️♣️♠️
===========================

[<img alt="github" src="https://img.shields.io/badge/github-davassi/davassi?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/davassi/poker-face)
[<img alt="build status" src="https://github.com/davassi/poker-face/actions/workflows/rust.yml/badge.svg" height="20">](https://github.com/davassi/poker-face/actions?query=branch%3Amaster)
[<img alt="crates.io" src="https://img.shields.io/crates/v/poker-face.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/poker-face)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/poker-face?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/poker-face)
[![Downloads](https://img.shields.io/crates/d/poker-face.svg)](https://crates.io/crates/poker-face)
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
 
Poker-Face is a rust virtusism exercise implementation of a Texas Hold'em poker engine. It evaluates the rank of a 5-card hand using the typical Rust match control flow construct. 
It determines the Rank of a 5 card hand examining the properties using Array Matching and Struct Matching [MatchHandEvaluator](https://github.com/davassi/poker-face/blob/master/src/match_evaluator.rs).

Example of usage of the library, as a binary:

```rust
Poker Face 0.1.0 - 🦀 for ♠️ ♣️ ♥️ ♦️
Prehashing cards...

1. Let's shuffle a deck...
 J♥️   7♠️   9♠️   10♥️  5♥️   K♦️   K♣️   K♥️   Q♦️   6♥️   3♥️   J♦️   4♥️  
 A♦️   J♣️   9♣️   5♠️   2♠️   4♣️   5♦️   3♦️   5♣️   6♦️   J♠️   Q♠️   4♠️  
 10♠️  7♣️   9♥️   7♥️   9♦️   3♠️   A♣️   A♥️   7♦️   2♦️   10♣️  6♣️   4♦️  
 K♠️   A♠️   8♣️   8♥️   Q♥️   8♦️   2♣️   2♥️   3♣️   Q♣️   6♠️   8♠️   10♦️ 

2. Let's take (actually, borrow) 2 hands of 5 cards each from the deck
Player 1 has:  J♥️   7♠️   9♠️   10♥️  5♥️  

Player 2 has:  K♦️   K♣️   K♥️   Q♦️   6♥️  


3. Let's evaluate the hands...
Sorted:  J♥️   10♥️  9♠️   7♠️   5♥️  

Sorted:  K♥️   K♣️   K♦️   Q♦️   6♥️  

Player 1 has a [HighCard with a highcard of value  J♥️] 
Player 2 has a [ThreeOfAKind]

4. Celebrate the winner:
The winner is Player 2!


```

As a library, there are some handy macros (newcard!, hand!, assert_rank!) implemented to deal with cards, hands and ranks. Example:

```rust
    assert_rank!(hand!["Ad","Kd","Qd","Jd","10d"], Rank::RoyalFlush);

    assert_rank!(hand!["Kd", "Kh", "Kc", "Ks", "Qd"], Rank::FourOfAKind);

    assert_rank!(hand!["2d", "2h", "Qc", "Qs", "Qd"], Rank::FullHouse);
```

or 

```rust
    assert_eq!(newcard!["Ah"], Card::new(14, Suit::Hearts));
```

## Execute

To run it from cargo, just type:

```console
cargo run -q -- 
```

or to build and install a release from the code:

```console
cargo build --release
cargo install --path .
./target/release/poker-face
```

## Contribution

If you have any suggestions for features (i.e. more functionality to implement), or if you find any problems in the code, design, interface, etc., please feel free to share them on our [GitHub](https://github.com/davassi/poker-face/issues).

I really appreciate your feedback!