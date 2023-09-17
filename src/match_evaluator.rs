use crate::card::{Card, Hand, Suit};

/// The [`Rank`] enum represents the standard poker hand ranks from highest to lowest
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, strum_macros::Display)]
pub enum Rank {
    ///  1. Royal Flush
    ///
    /// The highest rank possible, consisting of the Ace, King, Queen, Jack, and Ten all of the same suit.
    RoyalFlush,
    /// 2. Straight Flush
    ///
    /// Any sequence of five consecutive cards all of the same suit. For instance, a hand with the cards 5, 6, 7, 8, and 9 of diamonds is a straight flush.
    StraightFlush,
    /// 3. Four of a Kind (Poker)
    ///
    /// A hand containing four cards of the same rank, along with one unrelated card. For example, four Kings and a 3 would constitute "four of a kind."
    FourOfAKind,
    /// 4. Full House
    ///
    /// A hand containing three cards of one rank and two cards of another rank. For example, a hand with three 8s and two Jacks would be a full house, often noted as "8s full of Jacks."
    FullHouse,
    /// 5. Flush
    ///
    /// Any five cards of the same suit, but not in sequence. For instance, if a player has five heart cards, they have a flush.
    Flush,
    /// 6. Straight
    ///
    /// Five cards in a sequence, but not all of the same suit. An example would be a hand containing a 2, 3, 4, 5, and 6, of various suits.
    Straight,
    /// 7. Three of a Kind (Trips or Set)
    ///
    /// Three cards of the same rank and two unrelated cards. An example would be three Queens and two unrelated cards.
    ThreeOfAKind,
    /// 8. Two Pair
    ///
    /// Two different pairs of cards and one unrelated card. For example, a hand with two 10s, two 9s, and an unrelated card would be "two pair."
    TwoPair,
    /// 9. One Pair
    ///
    /// Two cards of the same rank and three unrelated cards. An example would be two 7s and three unrelated cards.
    OnePair,
    /// 10. High Card
    ///
    /// When a hand does not fall into any of the above categories, it is judged based on the highest individual card. So if no player has even one pair, the player with the highest card in their hand wins.
    HighCard(Card),
    None,
}

pub struct MatchHandEvaluator;

/// The core [MatchHandEvaluator] implementation.
///
/// It examines the properties of a hand using array matching and struct matching to determine which rank the hand belongs to.
/// 
impl MatchHandEvaluator {
    /// It evaluates the [Rank] of a [Hand]
    ///
    pub fn match_eval(hand: &mut Hand) -> Rank {
        // first let's sort the hand, that's the reason we need here a mutable reference
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
            [Card { suit: s1, val: v1 }, Card { suit: s2, val: v2 }, Card { suit: s3, val: v3 }, Card { suit: s4, val: v4 }, Card { suit: s5, val: v5 }]
                if Self::seq(*v1, *v2, *v3, *v4, *v5) && Self::suits(s1, s2, s3, s4, s5) =>
            {
                Rank::StraightFlush
            }
            [Card { suit: s1, val: 14 }, Card { suit: s2, val: 5 }, Card { suit: s3, val: 4 }, Card { suit: s4, val: 3 }, Card { suit: s5, val: 2 }]
                if Self::suits(s1, s2, s3, s4, s5) =>
            {
                Rank::StraightFlush // special case with Ace as 1
            }
            [Card { val: v1, .. }, Card { val: v2, .. }, Card { val: v3, .. }, Card { val: v4, .. }, Card { val: v5, .. }]
                if (v1 == v2 && v2 == v3 && v3 == v4 || v2 == v3 && v3 == v4 && v4 == v5) =>
            {
                Rank::FourOfAKind
            }
            [Card { val: v1, .. }, Card { val: v2, .. }, Card { val: v3, .. }, Card { val: v4, .. }, Card { val: v5, .. }]
                if (v1 == v2 && v2 == v3 && v4 == v5 || v3 == v4 && v4 == v5 && v1 == v2) =>
            {
                Rank::FullHouse
            }
            [Card { suit: s1, .. }, Card { suit: s2, .. }, Card { suit: s3, .. }, Card { suit: s4, .. }, Card { suit: s5, .. }]
                if Self::suits(s1, s2, s3, s4, s5) =>
            {
                Rank::Flush
            }
            //
            [Card { val: v1, .. }, Card { val: v2, .. }, Card { val: v3, .. }, Card { val: v4, .. }, Card { val: v5, .. }]
                if Self::seq(*v1, *v2, *v3, *v4, *v5) =>
            {
                Rank::Straight
            }
            [Card { val: 14, .. }, Card { val: 5, .. }, Card { val: 4, .. }, Card { val: 3, .. }, Card { val: 2, .. }] =>
            {
                Rank::Straight // special case with Ace as 1
            }
            [Card { val: v1, .. }, Card { val: v2, .. }, Card { val: v3, .. }, Card { val: v4, .. }, Card { val: v5, .. }]
                if (v1 == v2 && v2 == v3 || v3 == v4 && v4 == v5) =>
            {
                Rank::ThreeOfAKind
            }
            [Card { val: v1, .. }, Card { val: v2, .. }, Card { val: v3, .. }, Card { val: v4, .. }, Card { val: v5, .. }]
                if (v1 == v2 && v3 == v4 || v2 == v3 && v4 == v5) =>
            {
                Rank::TwoPair
            }
            [Card { val: v1, .. }, Card { val: v2, .. }, Card { val: v3, .. }, Card { val: v4, .. }, Card { val: v5, .. }]
                if (v1 == v2 || v2 == v3 || v3 == v4 || v4 == v5) =>
            {
                Rank::OnePair
            }

            hand => Rank::HighCard(hand[0]),
        }
    }

    /// function that checks if 5 given suits are all the same (i.e. all [Suit::Hearts])
    ///
    fn suits(c0: &Suit, c1: &Suit, c2: &Suit, c3: &Suit, c4: &Suit) -> bool {
        c0 == c1 && c1 == c2 && c2 == c3 && c3 == c4
    }

    /// function that checks if 5 given card ranks are a numeric sequence
    ///
    fn seq(v0: u8, v1: u8, v2: u8, v3: u8, v4: u8) -> bool {
        v0 == (v1 + 1) && v1 == (v2 + 1) && v2 == (v3 + 1) && v3 == (v4 + 1)
    }
}

#[macro_export]
macro_rules! assert_rank {
    ($hand:expr, $rank:expr) => {
        assert_eq!(MatchHandEvaluator::match_eval(&mut $hand), $rank);
    };
}

#[cfg(test)]
mod test {
    use super::MatchHandEvaluator;
    use super::Rank;
    use crate::card::{Card, Hand};
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
    fn rank_straight_flush() {
        assert_rank!(hand!["5d", "4d", "3d", "2d", "Ad"], Rank::StraightFlush);
        assert_rank!(hand!["10h", "9h", "8h", "7h", "6h"], Rank::StraightFlush);
        assert_rank!(hand!["Ks", "Qs", "Js", "10s", "9s"], Rank::StraightFlush);
    }

    #[test]
    fn rank_four_of_a_kind() {
        assert_rank!(hand!["Kd", "Kh", "Kc", "Ks", "Qd"], Rank::FourOfAKind);
        assert_rank!(hand!["Kd", "6h", "6c", "6s", "6d"], Rank::FourOfAKind);
    }

    #[test]
    fn rank_full_house() {
        assert_rank!(hand!["Kd", "Kh", "Kc", "8s", "8d"], Rank::FullHouse);
        assert_rank!(hand!["2d", "2h", "Qc", "Qs", "Qd"], Rank::FullHouse);
    }

    #[test]
    fn rank_3_of_a_kind() {
        assert_rank!(hand!["Kd", "Kh", "Kc", "10s", "8d"], Rank::ThreeOfAKind);
        assert_rank!(hand!["2d", "Jh", "Qc", "Qs", "Qd"], Rank::ThreeOfAKind);
    }

    #[test]
    fn rank_two_pairs() {
        assert_rank!(hand!["Kd", "Kh", "Jc", "Js", "10d"], Rank::TwoPair);
        assert_rank!(hand!["9d", "5h", "5c", "6s", "6d"], Rank::TwoPair);
    }

    #[test]
    fn rank_one_pairs() {
        assert_rank!(hand!["Kd", "Kh", "2c", "Js", "10d"], Rank::OnePair);
        assert_rank!(hand!["9d", "5h", "5c", "3s", "6d"], Rank::OnePair);
    }

    #[test]
    fn rank_seq() {
        assert_eq!(MatchHandEvaluator::seq(14, 13, 12, 11, 10), true);
        assert_eq!(MatchHandEvaluator::seq(13, 14, 12, 11, 5), false);
    }
}
