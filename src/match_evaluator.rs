use crate::{
    card::{Card, Hand, Rank, Suit},
    hash_tables::{CARDS, FLUSHES, HASH_ADJUST, HASH_VALUES, UNIQUE5},
};

pub struct MatchHandEvaluator;

impl MatchHandEvaluator {
    /// It evaluates the [Rank] of a [Hand]
    ///
    pub fn slow_eval(hand: &mut Hand) -> Rank {
        // first let's sort the hand, that's the reason we need a mutable reference
        hand.sort();

        println!("Sorted: {}", hand);

        // now we can safely shadow the mutable reference behind an immutable one
        let hand = hand.get_hand_slice();

        match hand {
            [Card { suit: s1, val: 14 }, Card { suit: s2, val: 13 }, Card { suit: s3, val: 12 }, Card { suit: s4, val: 11 }, Card { suit: s5, val: 10 }]
                if Self::suits(s1, s2, s3, s4, s5) =>
            {
                Rank::RoyalFlush
            }
            [Card { val : v1, .. }, Card { val: v2, .. }, Card { val: v3, .. }, Card { val: v4, .. }, Card { val: v5, .. }]
                if (v1 == v2 && v2 == v3 && v3 == v4 || v2 == v3 && v3 == v4 && v4 == v5) =>
            {
                Rank::FourOfAKind
            }
            [Card { suit: s1, .. }, Card { suit: s2, .. }, Card { suit: s3, .. }, Card { suit: s4, .. }, Card { suit: s5, .. }]
                if Self::suits(s1, s2, s3, s4, s5) =>
            {
                Rank::Flush
            }

            hand => Rank::HighCard(hand[0]),
        }
    }

    /// function that checks if 5 given suits are all the same (i.e. all [Suit::Hearts])
    ///
    fn suits(c0: &Suit, c1: &Suit, c2: &Suit, c3: &Suit, c4: &Suit) -> bool {
        c0 == c1 && c1 == c2 && c2 == c3 && c3 == c4
    }
}

#[macro_export]
macro_rules! assert_rank {
    ($hand:expr, $rank:expr) => {
        assert_eq!(MatchHandEvaluator::slow_eval(&mut $hand), $rank);
    };
}

#[cfg(test)]
mod test {
    use super::MatchHandEvaluator;
    use crate::card::{Card, Hand, Rank};
    use crate::hand;
    use crate::newcard;

    #[test]
    fn rank_royal_flush() {
        assert_rank!(hand!["Ad", "Kd", "Qd", "Jd", "10d"], Rank::RoyalFlush);
        assert_rank!(hand!["Ah", "Kh", "Qh", "Jh", "10h"], Rank::RoyalFlush);
        assert_rank!(hand!["Ac", "Kc", "Qc", "Jc", "10c"], Rank::RoyalFlush);
        assert_rank!(hand!["As", "Ks", "Qs", "Js", "10s"], Rank::RoyalFlush);
    }

    #[test]
    fn rank_four_of_a_kind() {
        assert_rank!(hand!["Kd", "Kh", "Kc", "Ks", "Qd"], Rank::FourOfAKind);
        assert_rank!(hand!["Kd", "6h", "6c", "6s", "6d"], Rank::FourOfAKind);
    }
}
