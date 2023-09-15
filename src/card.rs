//use log::debug;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::hash_tables::CARDS;

/// The [`Suit`] enum. It represents the categories into which the cards of a deck are divided: [`Suit::Hearts`], [`Suit::Diamonds`], [`Suit::Spades`], [`Suit::Clubs`]
///
#[derive(Debug, PartialEq, Clone, Copy, EnumIter, Eq, Hash)]
pub enum Suit {
    /// ♥️
    Hearts,
    /// ♦️
    Diamonds,
    /// ♦️
    Spades,
    /// ♣️
    Clubs,
}

/// The [`Card`] struct. It represents a card, composed by a [`Suit`] enum and a rank value.
///
/// The range of rank values is described as follows:
/// K,   Q,  J, 10, 9, 8, 7, 6, 5, 4, 3, 2, A
/// 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1
///
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

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num.partial_cmp(&other.num)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num.cmp(&other.num)
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
            _ => panic!("Indeed, that's a good reason to panic."),
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

/// An array of 5 cards compose a [`Hand`].
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Hand {
    hand: [Card; 5],
}

impl Hand {
    pub fn sort(&mut self) -> () {
        self.hand.sort();
    } 

    /// Returns a reference to the Card slice of this [`Hand`].
    pub const fn get_hand_slice(&self) -> &[Card; 5] {
        &self.hand
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Deck {
    deck: Vec<Card>,
    it: usize,
}

impl Deck {
    /// It creates a shuffled deck, ready to play.
    ///
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

    // gets a hand from the deck. TODO: return None if the deck is finisced.
    pub fn hand(&mut self) -> Option<Hand> {
        let hand: Hand = Hand {
            hand: self.deck[self.it..=(self.it + 4)].try_into().ok()?,
        };
        self.it += 5; // got 5 cards from the deck
        Some(hand)
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
