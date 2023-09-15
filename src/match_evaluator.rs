use crate::{
    card::{Card, Hand, Rank, Suit},
    hash_tables::{CARDS, FLUSHES, HASH_ADJUST, HASH_VALUES, UNIQUE5},
};

pub struct MatchHandEvaluator;

impl MatchHandEvaluator {

    pub fn slow_eval(hand: &mut Hand) -> Rank {

        // first let's sort the hand, that's why the reference is mutable
        hand.sort();

        let hand = hand.get_hand_slice();

        match hand {
        
            [ Card { suit: s1, num: 13 }, Card { suit: s2,num:  12 }, Card { suit: s3,num:  11 }, Card { suit: s4,num:  10 },Card { suit: s5,num:  9 }] if Self::suits(s1, s2, s3, s4, s5) => Rank::RoyalFlush,
            [ Card { suit: s1, .. }, Card { suit: s2, .. }, Card { suit: s3, .. }, Card { suit: s4, .. },Card { suit: s5, .. }] if Self::suits(s1, s2, s3, s4, s5) => Rank::Flush,

            _ => Rank::HighCard, 
        }
    }
            ///  1. Royal Flush
            /// 
            /// The highest rank possible, consisting of the Ace, King, Queen, Jack, and Ten all of the same suit.
          
    
    

    fn suits(c0: &Suit, c1: &Suit, c2: &Suit, c3: &Suit, c4: &Suit) -> bool {
        c0 == c1 && 
        c1 == c2 &&
        c2 == c3 &&
        c3 == c4
    }
}

    
