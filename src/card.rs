extern crate rand;

use std::cmp::Ordering;
use std::slice::Iter;

use super::*;

/// A playing card has a suit and a value
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Debug)]
pub struct Card {
    /// The card's suit, e.g. Hearts
    pub suit: Suit,
    /// The card's value, e.g. Jack
    pub value: Value
}

impl Ord for Card {
    /// Sorts by value and then suit
    fn cmp(&self, other: &Card) -> Ordering
    {
        self.cmp_value_then_suit(other)
    }
}

impl Card {
    /// Creates a card with the given suit and value
    pub fn new(suit: Suit, value: Value) -> Card {
        Card {
            suit: suit,
            value: value
        }
    }

    /// Compares by value and then suit
    pub fn cmp_value_then_suit(&self, other: &Card) -> Ordering
    {
        let result : Ordering = self.value.cmp(&other.value);
        if result == Ordering::Equal {
            return self.suit.cmp(&other.suit);
        }
        result
    }

    /// Compares by suit and then value
    pub fn cmp_suit_then_value(&self, other: &Card) -> Ordering
    {
        let result : Ordering = self.suit.cmp(&other.suit);
        if result == Ordering::Equal {
            return self.value.cmp(&other.value);
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

        // Test value / suit
        if let Ok(value) = Value::from_char(c1) {
            if let Ok(suit) = Suit::from_char(c2) {
                return Ok(Card::new(suit, value));
            }
        }
        // Try suit / value
        if let Ok(suit) = Suit::from_char(c1) {
            if let Ok(value) = Value::from_char(c2) {
                return Ok(Card::new(suit, value));
            }
        }

        Err("Invalid string")
    }

    /// Turns the card into a short string consisting of value, suit, e.g. "AS"
    pub fn to_str(&self) -> String {
        let mut result = String::with_capacity(2);
        result.push(self.value.to_char());
        result.push(self.suit.to_char());
        result
    }

    /// Returns an English formatted name of the card, e.g. "Ace of Spades"
    pub fn name(&self) -> String {
        let mut name : String = self.value.to_str().to_string();
        name = name + " of ";
        name = name + self.suit.to_str();
        name
    }

    /// Returns an ordinal for the card which is a unique number which can be used for indexing
    pub fn ordinal(&self) -> usize {
        self.suit.ordinal() * 13 + self.value.ordinal()
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
            Card { suit: Suit::Spades, value: Value::Two },
            Card { suit: Suit::Spades, value: Value::Three },
            Card { suit: Suit::Spades, value: Value::Four },
            Card { suit: Suit::Spades, value: Value::Five },
            Card { suit: Suit::Spades, value: Value::Six },
            Card { suit: Suit::Spades, value: Value::Seven },
            Card { suit: Suit::Spades, value: Value::Eight },
            Card { suit: Suit::Spades, value: Value::Nine },
            Card { suit: Suit::Spades, value: Value::Ten },
            Card { suit: Suit::Spades, value: Value::Jack },
            Card { suit: Suit::Spades, value: Value::Queen },
            Card { suit: Suit::Spades, value: Value::King },
            Card { suit: Suit::Spades, value: Value::Ace },

            Card { suit: Suit::Hearts, value: Value::Two },
            Card { suit: Suit::Hearts, value: Value::Three },
            Card { suit: Suit::Hearts, value: Value::Four },
            Card { suit: Suit::Hearts, value: Value::Five },
            Card { suit: Suit::Hearts, value: Value::Six },
            Card { suit: Suit::Hearts, value: Value::Seven },
            Card { suit: Suit::Hearts, value: Value::Eight },
            Card { suit: Suit::Hearts, value: Value::Nine },
            Card { suit: Suit::Hearts, value: Value::Ten },
            Card { suit: Suit::Hearts, value: Value::Jack },
            Card { suit: Suit::Hearts, value: Value::Queen },
            Card { suit: Suit::Hearts, value: Value::King },
            Card { suit: Suit::Hearts, value: Value::Ace },

            Card { suit: Suit::Diamonds, value: Value::Two },
            Card { suit: Suit::Diamonds, value: Value::Three },
            Card { suit: Suit::Diamonds, value: Value::Four },
            Card { suit: Suit::Diamonds, value: Value::Five },
            Card { suit: Suit::Diamonds, value: Value::Six },
            Card { suit: Suit::Diamonds, value: Value::Seven },
            Card { suit: Suit::Diamonds, value: Value::Eight },
            Card { suit: Suit::Diamonds, value: Value::Nine },
            Card { suit: Suit::Diamonds, value: Value::Ten },
            Card { suit: Suit::Diamonds, value: Value::Jack },
            Card { suit: Suit::Diamonds, value: Value::Queen },
            Card { suit: Suit::Diamonds, value: Value::King },
            Card { suit: Suit::Diamonds, value: Value::Ace },

            Card { suit: Suit::Clubs, value: Value::Two },
            Card { suit: Suit::Clubs, value: Value::Three },
            Card { suit: Suit::Clubs, value: Value::Four },
            Card { suit: Suit::Clubs, value: Value::Five },
            Card { suit: Suit::Clubs, value: Value::Six },
            Card { suit: Suit::Clubs, value: Value::Seven },
            Card { suit: Suit::Clubs, value: Value::Eight },
            Card { suit: Suit::Clubs, value: Value::Nine },
            Card { suit: Suit::Clubs, value: Value::Ten },
            Card { suit: Suit::Clubs, value: Value::Jack },
            Card { suit: Suit::Clubs, value: Value::Queen },
            Card { suit: Suit::Clubs, value: Value::King },
            Card { suit: Suit::Clubs, value: Value::Ace }
        ];
        &CARDS
    }

    pub fn iterator() -> Iter<'static, Card> {
        Card::all_cards().into_iter()
    }
}
