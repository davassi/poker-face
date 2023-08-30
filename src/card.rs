//use log::debug;
use crate::hash_tables::{FLUSHES, HASH_ADJUST, HASH_VALUES, UNIQUE5};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Clone, Copy, EnumIter, Eq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

/// K,   Q,  J, 10, 9, 8, 7, 6, 5, 4, 3, 2, A
/// 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Card {
    suit: Suit,
    num: u8,
}

impl Card {
    pub fn new(suit: Suit, num: u8) -> Self {
        Card { suit, num }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;
        match self.num {
            n @ 2..=10 => write!(f, "{n}"),
            1 => write!(f, "A"),
            11 => write!(f, "J"),
            12 => write!(f, "Q"),
            13 => write!(f, "K"),
            _ => panic!("not goot at all"),
        }?;
        match self.suit {
            //
            Suit::Hearts => write!(f, "♥️"),
            Suit::Spades => write!(f, "♠️"),
            Suit::Diamonds => write!(f, "♦️"),
            Suit::Clubs => write!(f, "♣️"),
        }?;
        write!(f, " ")
    }
}

#[derive(Debug, PartialEq, Clone, Copy, strum_macros::Display)]
pub enum Rank {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    None,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Hand {
    hand: [Card; 5],
}

const CARDS: [u32; 52] = [
    0x18002, 0x14002, 0x12002, 0x11002, 0x28103, 0x24103, 0x22103, 0x21103, 0x48205, 0x44205,
    0x42205, 0x41205, 0x88307, 0x84307, 0x82307, 0x81307, 0x10840b, 0x10440b, 0x10240b, 0x10140b,
    0x20850d, 0x20450d, 0x20250d, 0x20150d, 0x408611, 0x404611, 0x402611, 0x401611, 0x808713,
    0x804713, 0x802713, 0x801713, 0x1008817, 0x1004817, 0x1002817, 0x1001817, 0x200891d, 0x200491d,
    0x200291d, 0x200191d, 0x4008a1f, 0x4004a1f, 0x4002a1f, 0x4001a1f, 0x8008b25, 0x8004b25,
    0x8002b25, 0x8001b25, 0x10008c29, 0x10004c29, 0x10002c29, 0x10001c29,
];

//const CARD_HASHES: HashMap<Card, u32> = HashMap::from(Card::new(suit, num), CARDS[0]);

#[derive(Debug, PartialEq, Clone)]
pub struct Deck {
    deck: Vec<Card>,
    it: usize,
}

impl Deck {
    pub fn create_shuffled_deck() -> Deck {
        let mut card_hash: HashMap<Card, u32> = HashMap::new();
        let mut counter = 0;
        let mut deck: Vec<Card> = Vec::new();

        for suit in Suit::iter() {
            for num in 1..=13 {
                let card = Card::new(suit, num);
                deck.push(card);
                card_hash.insert(card, CARDS[counter]);
                counter += 1;
            }
        }
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        Deck { deck, it: 0 }
    }

    pub fn hand(&mut self) -> Hand {
        let hand: Hand = Hand {
            hand: self.deck[self.it..=(self.it + 4)].try_into().unwrap(),
        };
        self.it += 5;
        hand
    }

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
    pub fn combinations(&self) {
        let mut deck: Vec<Card> = Vec::new();

        for suit in Suit::iter() {
            for num in 1..=13 {
                deck.push(Card { suit, num });
            }
        }
        // Generate all combinations
        let mut combinations: Vec<[Card; 5]> = Vec::new();
        let mut hashes: HashSet<u16> = HashSet::new();
        let deck_len = deck.len();
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

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..=4 {
            write!(f, "{} ", self.hand[i])?;
        }
        writeln!(f, "")
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..52 {
            write!(f, "{} ", self.deck[i])?;
            if (i + 1) % 13 == 0 {
                writeln!(f, "")?;
            }
        }
        write!(f, "")
    }
}
