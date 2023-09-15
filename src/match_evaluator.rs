use crate::{
    card::{Card, Hand, Rank, Suit},
    hash_tables::{CARDS, FLUSHES, HASH_ADJUST, HASH_VALUES, UNIQUE5},
};

pub struct MatchHandEvaluator;

impl MatchHandEvaluator {
    pub fn slow_eval(hand: &mut Hand) -> Rank {

        // first let's sort the hand, that's why the reference is mutable
        hand.sort();

        match hand.get_hand_slice() {
            [_,_,_,_,_] => Rank::StraightFlush,
            //[_,_,_,_,_] => Rank::StraightFlush,
            _ => todo!(), 
        };

        Rank::StraightFlush
    }
}
