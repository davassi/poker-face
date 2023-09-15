use pokerust::{card::*, match_evaluator::MatchHandEvaluator};

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("Poker Face {} - 🦀 for ♠️ ♣️ ♥️ ♦️", VERSION);
    println!("Prehashing cards...");

    println!("\n1. Let's shuffle a deck...");
    let mut deck = Deck::create_shuffled_deck();
    println!("{}", deck);

    println!("2. Let's take (actually, borrow) 2 hands of 5 cards each from the deck");
    let (Some(hand_p1), Some(hand_p2)) = (deck.hand(), deck.hand()) else {
        panic!("Well, pretty unlikely to panic here as we are getting 10 cards out of a deck of 52...")
    };
    println!("Player 1 has: {}", hand_p1);
    println!("Player 2 has: {}", hand_p2);

    println!("\n3. Let's evaluate the hands...");
    let score1 = MatchHandEvaluator::slow_eval(&hand_p1);
    let score2 = MatchHandEvaluator::slow_eval(&hand_p2);
    println!("Player 1 has a {}", score1);
    println!("Player 2 has a {}", score2);

    println!("\n4. Celebrate the winner:");
    println!("The winner is Player 1\n");
}
