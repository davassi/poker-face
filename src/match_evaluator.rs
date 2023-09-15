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
            [c0,c1,c2,c3,c4] if Self::royal(c0,c1,c2,c3,c4) => Rank::RoyalFlush,
            [c0,c1,c2,c3,c4] if Self::suits(c0,c1,c2,c3,c4) => Rank::Flush,
            
            [_,_,_,_,_] => todo!(),
            _ => todo!(), 
        };

        Rank::StraightFlush
    }

    ///  1. Royal Flush
    /// 
    /// The highest rank possible, consisting of the Ace, King, Queen, Jack, and Ten all of the same suit.
    fn royal(c0: &Card, c1: &Card, c2: &Card, c3: &Card, c4: &Card) -> bool {
       
        Self::suits(c0,c1,c2,c3,c4) && (c0.num == 13)
            && c1.num == 12
            && c2.num == 11
            && c3.num == 10
            && c4.num == 9 
    }

    fn suits(c0: &Card, c1: &Card, c2: &Card, c3: &Card, c4: &Card) -> bool {
        c0.suit == c1.suit && 
        c1.suit == c2.suit &&
        c2.suit == c3.suit &&
        c3.suit == c4.suit
    }
}

    
