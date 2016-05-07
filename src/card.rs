extern crate rand;

use std::fmt;
use std::cmp::Ordering;
use std::slice::Iter;

use super::*;

/// A playing card has a suit and a rank
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Debug)]
pub struct Card {
    /// The card's rank, e.g. Jack
    pub rank: Rank,
    /// The card's suit, e.g. Hearts
    pub suit: Suit
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Ord for Card {
    /// Sorts by rank and then suit
    fn cmp(&self, other: &Card) -> Ordering
    {
        self.cmp_rank_then_suit(other)
    }
}

impl Card {
    /// Creates a card with the given suit and rank
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank: rank,
            suit: suit
        }
    }

    /// Compares by rank and then suit
    pub fn cmp_rank_then_suit(&self, other: &Card) -> Ordering
    {
        let result : Ordering = self.rank.cmp(&other.rank);
        if result == Ordering::Equal {
            return self.suit.cmp(&other.suit);
        }
        result
    }

    /// Compares by descending rank and then suit
    pub fn cmp_desc_rank_then_suit(&self, other: &Card) -> Ordering
    {
        // Reverse order of the rank
        let result : Ordering = self.rank.cmp(&other.rank).reverse();
        if result == Ordering::Equal {
            return self.suit.cmp(&other.suit);
        }
        result
    }

    /// Compares by suit and then rank
    pub fn cmp_suit_then_rank(&self, other: &Card) -> Ordering
    {
        let result : Ordering = self.suit.cmp(&other.suit);
        if result == Ordering::Equal {
            return self.rank.cmp(&other.rank);
        }
        result
    }

    /// Compares by suit and then rank
    pub fn cmp_suit_then_desc_rank(&self, other: &Card) -> Ordering
    {
        let result : Ordering = self.suit.cmp(&other.suit);
        if result == Ordering::Equal {
            return self.rank.cmp(&other.rank).reverse();
        }
        result
    }

    /// Creates a card from a string such, e.g. "AS" returns Ace of Spades
    pub fn from_str(s : &str) -> Result<Card, &'static str> {
        if s.len() != 2 {
            return Err("String is wrong length")
        }

        let s = s.to_string();
        let mut i = s.chars();
        let c1 = i.next().unwrap();
        let c2 = i.next().unwrap();

        // Test rank / suit
        if let Ok(rank) = Rank::from_char(c1) {
            if let Ok(suit) = Suit::from_char(c2) {
                return Ok(Card::new(rank, suit));
            }
        }
        // Try suit / rank
        if let Ok(suit) = Suit::from_char(c1) {
            if let Ok(rank) = Rank::from_char(c2) {
                return Ok(Card::new(rank, suit));
            }
        }

        Err("Invalid string")
    }

    /// Turns the card into a short string consisting of rank, suit, e.g. "AS"
    pub fn to_str(&self) -> String {
        let mut result = String::with_capacity(2);
        result.push(self.rank.to_char());
        result.push(self.suit.to_char());
        result
    }

    /// Returns an English formatted name of the card, e.g. "Ace of Spades"
    pub fn name(&self) -> String {
        let mut name : String = self.rank.to_str().to_string();
        name = name + " of ";
        name = name + self.suit.to_str();
        name
    }

    /// Returns an ordinal for the card which is a unique number which can be used for indexing
    pub fn ordinal(&self) -> usize {
        self.suit.ordinal() * 13 + self.rank.ordinal()
    }

    /// Tests if the card is Hearts
    pub fn is_hearts(&self) -> bool {
        self.suit == Suit::Hearts
    }

    /// Tests if the card is Clubs
    pub fn is_clubs(&self) -> bool {
        self.suit == Suit::Clubs
    }

    /// Tests if the card is Spades
    pub fn is_spades(&self) -> bool {
        self.suit == Suit::Spades
    }

    /// Tests if the card is Diamonds
    pub fn is_diamonds(&self) -> bool {
        self.suit == Suit::Diamonds
    }

    /// Returns an array slice containing all the cards in a standard 52-card deck
    pub fn all_cards() -> &'static[Card] {
        static CARDS : [Card; 52] = [
            Card { suit: Suit::Spades, rank: Rank::Two },
            Card { suit: Suit::Spades, rank: Rank::Three },
            Card { suit: Suit::Spades, rank: Rank::Four },
            Card { suit: Suit::Spades, rank: Rank::Five },
            Card { suit: Suit::Spades, rank: Rank::Six },
            Card { suit: Suit::Spades, rank: Rank::Seven },
            Card { suit: Suit::Spades, rank: Rank::Eight },
            Card { suit: Suit::Spades, rank: Rank::Nine },
            Card { suit: Suit::Spades, rank: Rank::Ten },
            Card { suit: Suit::Spades, rank: Rank::Jack },
            Card { suit: Suit::Spades, rank: Rank::Queen },
            Card { suit: Suit::Spades, rank: Rank::King },
            Card { suit: Suit::Spades, rank: Rank::Ace },

            Card { suit: Suit::Hearts, rank: Rank::Two },
            Card { suit: Suit::Hearts, rank: Rank::Three },
            Card { suit: Suit::Hearts, rank: Rank::Four },
            Card { suit: Suit::Hearts, rank: Rank::Five },
            Card { suit: Suit::Hearts, rank: Rank::Six },
            Card { suit: Suit::Hearts, rank: Rank::Seven },
            Card { suit: Suit::Hearts, rank: Rank::Eight },
            Card { suit: Suit::Hearts, rank: Rank::Nine },
            Card { suit: Suit::Hearts, rank: Rank::Ten },
            Card { suit: Suit::Hearts, rank: Rank::Jack },
            Card { suit: Suit::Hearts, rank: Rank::Queen },
            Card { suit: Suit::Hearts, rank: Rank::King },
            Card { suit: Suit::Hearts, rank: Rank::Ace },

            Card { suit: Suit::Diamonds, rank: Rank::Two },
            Card { suit: Suit::Diamonds, rank: Rank::Three },
            Card { suit: Suit::Diamonds, rank: Rank::Four },
            Card { suit: Suit::Diamonds, rank: Rank::Five },
            Card { suit: Suit::Diamonds, rank: Rank::Six },
            Card { suit: Suit::Diamonds, rank: Rank::Seven },
            Card { suit: Suit::Diamonds, rank: Rank::Eight },
            Card { suit: Suit::Diamonds, rank: Rank::Nine },
            Card { suit: Suit::Diamonds, rank: Rank::Ten },
            Card { suit: Suit::Diamonds, rank: Rank::Jack },
            Card { suit: Suit::Diamonds, rank: Rank::Queen },
            Card { suit: Suit::Diamonds, rank: Rank::King },
            Card { suit: Suit::Diamonds, rank: Rank::Ace },

            Card { suit: Suit::Clubs, rank: Rank::Two },
            Card { suit: Suit::Clubs, rank: Rank::Three },
            Card { suit: Suit::Clubs, rank: Rank::Four },
            Card { suit: Suit::Clubs, rank: Rank::Five },
            Card { suit: Suit::Clubs, rank: Rank::Six },
            Card { suit: Suit::Clubs, rank: Rank::Seven },
            Card { suit: Suit::Clubs, rank: Rank::Eight },
            Card { suit: Suit::Clubs, rank: Rank::Nine },
            Card { suit: Suit::Clubs, rank: Rank::Ten },
            Card { suit: Suit::Clubs, rank: Rank::Jack },
            Card { suit: Suit::Clubs, rank: Rank::Queen },
            Card { suit: Suit::Clubs, rank: Rank::King },
            Card { suit: Suit::Clubs, rank: Rank::Ace }
        ];
        &CARDS
    }

    pub fn iterator() -> Iter<'static, Card> {
        Card::all_cards().into_iter()
    }
}
