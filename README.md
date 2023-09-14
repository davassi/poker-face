POKER FACE - Rpn Resolver
===========================

[<img alt="github" src="https://img.shields.io/badge/github-davassi/davassi?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/poker-face/yarer)
[<img alt="build status" src="https://github.com/davassi/yarer/actions/workflows/rust.yml/badge.svg" height="20">](https://github.com/davassi/poker-face/actions?query=branch%3Amaster)
[<img alt="crates.io" src="https://img.shields.io/crates/v/yarer.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/poker-face)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/yarer?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/poker-face)
[![Downloads](https://img.shields.io/crates/d/poker-face.svg)](https://crates.io/crates/poker-face)
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)

Poker-Face is a texas hold'em poker engine implementation written in Rust. It evaluates the rank of a 5-card hand using Cactus Kev's Poker Hand Evaluator (fast, but relies heavily on hash tables) or a match control flow construct (slow, but readable).

Example of usage of the library:

```rust
Poker Face 0.1.0 - 🦀 for ♠️ ♣️ ♥️ ♦️
Prehashing cards...

1. Let's shuffle a deck...
 9♥️   7♥️   K♦️   4♦️   2♣️   10♠️   8♥️   4♥️   3♥️   2♠️   J♦️   Q♠️   5♥️  
 10♦️   K♠️   3♣️   K♥️   A♥️   8♣️   8♠️   A♠️   K♣️   5♦️   3♠️   6♠️   8♦️  
 7♦️   7♣️   5♠️   10♣️   J♣️   9♠️   6♥️   9♦️   6♦️   A♣️   9♣️   Q♥️   3♦️  
 4♠️   6♣️   10♥️   2♦️   5♣️   J♥️   4♣️   Q♣️   2♥️   J♠️   Q♦️   7♠️   A♦️  

2. Let's take (borrow) 2 hands of 5 cards each from the deck
Player 1:  9♥️   7♥️   K♦️   4♦️   2♣️  

Player 2:  10♠️   8♥️   4♥️   3♥️   2♠️  


3. Let's evaluate the hands...

4. Celebrate the winner:
The winner is Player 1

```