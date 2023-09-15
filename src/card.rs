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
///  A,  K,  Q,  J, 10, 9, 8, 7, 6, 5, 4, 3, 2
/// 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2
///
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub num: u8,
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
            14 => write!(f, "A"),
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

/// The [`Rank`] enum represents the standard poker hand ranks from highest to lowest
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, strum_macros::Display)]
pub enum Rank {
    ///  1. Royal Flush
    ///
    /// The highest rank possible, consisting of the Ace, King, Queen, Jack, and Ten all of the same suit.
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    /// 5. Flush
    ///
    /// Any five cards of the same suit, but not in sequence. For instance, if a player has five heart cards, they have a flush.
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    /// 10. High Card
    ///
    /// When a hand does not fall into any of the above categories, it is judged based on the highest individual card. So if no player has even one pair, the player with the highest card in their hand wins.
    HighCard(Card),
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
            for num in 2..=14 {
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


/*
2. Straight Flush

Any sequence of five consecutive cards all of the same suit. For instance, a hand with the cards 5, 6, 7, 8, and 9 of diamonds is a straight flush.
3. Four of a Kind (Quads)

A hand containing four cards of the same rank, along with one unrelated card. For example, four Kings and a 3 would constitute "four of a kind."
4. Full House

A hand containing three cards of one rank and two cards of another rank. For example, a hand with three 8s and two Jacks would be a full house, often noted as "8s full of Jacks."

6. Straight

Five cards in a sequence, but not all of the same suit. An example would be a hand containing a 2, 3, 4, 5, and 6, of various suits.
7. Three of a Kind (Trips or Set)

Three cards of the same rank and two unrelated cards. An example would be three Queens and two unrelated cards.
8. Two Pair

Two different pairs of cards and one unrelated card. For example, a hand with two 10s, two 9s, and an unrelated card would be "two pair."
9. One Pair

Two cards of the same rank and three unrelated cards. An example would be two 7s and three unrelated cards.
 */