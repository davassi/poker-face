use std::collections::HashMap;

use pokerust::card::*;

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("Poker Face {} - 🦀 for ♠️ ♣️ ♥️ ♦️", VERSION);
    println!("Prehashing cards...");

    println!("\n1. Let's shuffle a deck...");
    let mut deck = Deck::create_shuffled_deck();
    println!("{}", deck);

    println!("2. Let's take (borrow) 2 hands of 5 cards each from the deck");
    let hand_p1: Hand = deck.hand();
    let hand_p2: Hand = deck.hand();
    println!("Player 1: {}", hand_p1);
    println!("Player 2: {}", hand_p2);

    println!("\n3. Let's evaluate the hands...");
    //let score1 = deck.eval5(hand_p1);
    // let score2 = deck.eval5(hand_p1);
    println!("Player 1 has a {}", score1);
    println!("Player 2 has a {}", score2);

    println!("\n4. Celebrate the winner:");
    println!("The winner is Player 1\n");
}
