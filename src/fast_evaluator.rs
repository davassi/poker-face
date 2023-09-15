use std::collections::HashSet;
use strum::IntoEnumIterator;

use crate::{
    card::{Card, Rank, Suit},
    hash_tables::{CARDS, FLUSHES, HASH_ADJUST, HASH_VALUES, UNIQUE5},
};

pub struct FastEvaluator;

impl FastEvaluator {
    pub fn rank_name(rank: u64) -> Rank {
        match rank {
            0..=1276 => Rank::HighCard,
            1277..=4136 => Rank::OnePair,
            4137..=4994 => Rank::TwoPair,
            4995..=5852 => Rank::ThreeOfAKind,
            5853..=5862 => Rank::Straight,
            5863..=7139 => Rank::Flush,
            7140..=7295 => Rank::FullHouse,
            7296..=7451 => Rank::FourOfAKind,
            7452..=7461 => Rank::StraightFlush,
            _ => panic!("Unexpected hand rank value! '{}'", rank),
        }
    }

    // magic
    fn find_fast(u: u32) -> usize {
        let u = u.wrapping_add(0xe91aaa35);
        let u = u ^ (u >> 16);
        let u = u.wrapping_add(u << 8);
        let u = u ^ (u >> 4);
        let b = (u >> 8) & 0x1ff;
        let a = u.wrapping_add(u << 2) >> 19;
        a as usize ^ HASH_ADJUST[b as usize] as usize
    }

    fn eval_5cards_fast(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32) -> u16 {
        let q = ((c1 | c2 | c3 | c4 | c5) >> 16) as usize;
        if (c1 & c2 & c3 & c4 & c5 & 0xf000) != 0 {
            return FLUSHES[q];
        }
        let s = UNIQUE5[q];
        if s != 0 {
            return s;
        }
        HASH_VALUES
            [Self::find_fast((c1 & 0xff) * (c2 & 0xff) * (c3 & 0xff) * (c4 & 0xff) * (c5 & 0xff))]
    }

    pub fn eval_5cards(c1: usize, c2: usize, c3: usize, c4: usize, c5: usize) -> u16 {
        Self::eval_5cards_fast(CARDS[c1], CARDS[c2], CARDS[c3], CARDS[c4], CARDS[c5])
    }

    /// Paul Senzee’s optimized version of Kevin Suffecool’s eval_5cards function
    fn combinations(&self) {
        let mut deck: Vec<Card> = Vec::new();

        for suit in Suit::iter() {
            for num in 1..=13 {
                deck.push(Card::new(suit, num));
            }
        }
        // Generate all combinations
        let mut combinations: Vec<[Card; 5]> = Vec::new();
        let mut hashes: HashSet<u16> = HashSet::new();
        //let deck_len = deck.len();
        for i in 0..48 {
            for j in (i + 1)..49 {
                for k in (j + 1)..50 {
                    for m in (k + 1)..51 {
                        for n in (m + 1)..52 {
                            // let rank = find_fast(i, j, k, m, n);
                            let hand = [deck[i], deck[j], deck[k], deck[m], deck[n]];
                            combinations.push(hand);
                            let h = Self::eval_5cards(i, j, k, m, n);

                            if !hashes.contains(&h) {
                                hashes.insert(h);
                            }
                        }
                    }
                }
            }
        }
        // In fact, there are only 7461 different hand strengths.
        println!(
            "Number of combinations: {} {}",
            combinations.len(),
            hashes.len()
        );
    }
}
