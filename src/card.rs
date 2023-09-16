//use log::debug;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::hash_tables::CARDS;

use thiserror::Error;

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
    pub val: u8,
    pub suit: Suit,
}

impl Card {
    pub fn new(val: u8, suit: Suit) -> Self {
        Card { suit, val }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

#[derive(Error, Debug)]
pub enum CardError {
    #[error("the card string is not the correct length")]
    InvalidLength,
    
    #[error("the card value is invalid")]
    InvalidValue,
    
    #[error("the card suit is invalid")]
    InvalidSuit,
}

impl TryFrom<&str> for Card {
    type Error = CardError;

    fn try_from(card: &str) -> Result<Self, Self::Error> {
    
        if card.len() != 2 && card.len() != 3 {
            return Err(CardError::InvalidLength);
        }

        let chars: Vec<char> = card.chars().collect();
        let val = chars[0];

        let val = match val {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            '1' => 10,
            '2'..='9' => val.to_digit(10).unwrap() as u8,
            _ => return Err(CardError::InvalidValue),
        };

        let suit = chars[if val == 10 {2} else {1}];

        let suit = match suit.to_ascii_lowercase() {
            'h' => Suit::Hearts,
            's' => Suit::Spades,
            'd' => Suit::Diamonds,
            'c' => Suit::Clubs,
            _ => return Err(CardError::InvalidSuit),
        };

        Ok(Card::new(val, suit))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;
        match self.val {
            n @ 2..=10 => write!(f, "{n}"),
            14 => write!(f, "A"),
            13 => write!(f, "K"),
            12 => write!(f, "Q"),
            11 => write!(f, "J"),
            _ => panic!("Indeed, that's a good reason to panic."),
        }?;
        match self.suit {
            //
            Suit::Hearts => write!(f, "♥️"),   //H
            Suit::Spades => write!(f, "♠️"),   //S
            Suit::Diamonds => write!(f, "♦️"), //D
            Suit::Clubs => write!(f, "♣️"),    //C
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
    /// sorting the hand in a descenting order
    ///
    pub fn sort(&mut self) -> () {
        self.hand.sort();
        self.hand.reverse();
    }

    /// Borrowing the cards of this [`Hand`].
    ///
    pub const fn get_hand_slice(&self) -> &[Card; 5] {
        &self.hand
    }

    /// An handy constructor for tests
    ///
    pub fn new(hand: [Card; 5]) -> Hand {
        Hand { hand }
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
            for val in 2..=14 {
                let card = Card::new(val, suit);
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

#[macro_export]
macro_rules! newcard {
    ($val:expr, $suit:tt) => {
        Card::new($val, $suit)
    };
    ($val:expr) => {
        Card::try_from($val).unwrap()
    };
}

macro_rules! newhand {
  
    ($c:expr,$c1:expr,$c2:expr,$c3:expr,$c4:expr) => {
        [newcard![$c],newcard![$c1],newcard![$c2],newcard![$c3],newcard![$c4]]
    };
}

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_valid_card() {
    
        let card = newcard!["Ad"];
        assert_eq!(card.val, 14);
        assert_eq!(card.suit, Suit::Diamonds);

        assert_eq!(newcard!["Ah"], Card::new(14, Suit::Hearts));
        assert_eq!(newcard!["Kh"], Card::new(13, Suit::Hearts));
        assert_eq!(newcard!["Qh"], Card::new(12, Suit::Hearts));
        assert_eq!(newcard!["Jh"], Card::new(11, Suit::Hearts));
        assert_eq!(newcard!["10h"], Card::new(10, Suit::Hearts));
        assert_eq!(newcard!["9h"], Card::new(9, Suit::Hearts));
    }

    #[test]
    fn test_try_from_valid_hand() {
    
        let hand = newhand!["Ad","Kd","Qd","Jd","10d"];
        assert_eq!(hand[0], Card::new(14, Suit::Diamonds));
    }
}