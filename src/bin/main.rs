use pokerust::{card::*, match_evaluator::MatchHandEvaluator};

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("Poker Face {} - 🦀 for ♠️ ♣️ ♥️ ♦️", VERSION);
    println!("Prehashing cards...");

    println!("\n1. Let's shuffle a deck...");
    let mut deck: Deck = Deck::create_shuffled_deck();
    println!("{}", deck);

    println!("2. Let's take (actually, borrow) 2 hands of 5 cards each from the deck");
    let (Some(mut hand_p1), Some(mut hand_p2)) = (deck.hand(), deck.hand()) else {
        panic!(
            "Well, pretty unlikely to panic here as we are getting just 10 cards out of a deck of 52..."
        )
    };
    println!("Player 1 has: {}", hand_p1);
    println!("Player 2 has: {}", hand_p2);

    println!("\n3. Let's evaluate the hands...");
    let score1: Rank = MatchHandEvaluator::slow_eval(&mut hand_p1);
    let score2: Rank = MatchHandEvaluator::slow_eval(&mut hand_p2);

    println!("Player 1 has a {}", score1);
    match score1 {
        Rank::HighCard(c) => println!("With highcard of value {c}"),
        _ => (),
    }

    println!("Player 2 has a {}", score2);
    match score2 {
        Rank::HighCard(c) => println!("With highcard of value {c}"),
        _ => (),
    }

    let winner = if score1 > score2 { 1 } else { 2 };
    println!("\n4. Celebrate the winner:");
    println!("The winner is Player {winner}\n");
}
