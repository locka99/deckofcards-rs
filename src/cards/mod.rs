use std::slice::Iter;

use self::Suit::*;
use self::Value::*;

// Standard card suits
#[derive(Copy,Clone,PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] =
            [Spades, Hearts, Diamonds, Clubs];
        SUITS.into_iter()
    }

    pub fn to_str(&self) -> &str {
        let suit_str;
        match *self {
            Spades => suit_str = "Spades",
            Hearts => suit_str = "Hearts",
            Diamonds => suit_str = "Diamonds",
            Clubs => suit_str = "Clubs"
        }
        suit_str
    }
}

// Standard card values
#[derive(Copy,Clone,PartialEq)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Value {
    pub fn iterator() -> Iter<'static, Value> {
        static VALUES: [Value; 13] =
            [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace];
        VALUES.into_iter()
    }

    pub fn to_str(&self) -> &str {
        let value_str;
        match *self {
            Ace => value_str = "Ace",
            Two => value_str = "Two",
            Three => value_str = "Three",
            Four => value_str = "Four",
            Five => value_str = "Five",
            Six => value_str = "Six",
            Seven => value_str = "Seven",
            Eight => value_str = "Eight",
            Nine => value_str = "Nine",
            Ten => value_str = "Ten",
            Jack => value_str = "Jack",
            Queen => value_str = "Queen",
            King => value_str = "King",
        }
        value_str
    }
}

pub struct Card {
    suit: Suit,
    value: Value
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Card {
        Card {
            suit: suit,
            value: value
        }
    }

    pub fn name(&self) -> String {
        let mut name : String = self.value.to_str().to_string();
        name = name + " of ";
        name = name + self.suit.to_str();
        name
    }

    pub fn is_hearts(&self) -> bool {
        self.suit == Suit::Hearts
    }

    pub fn is_clubs(&self) -> bool {
        self.suit == Suit::Clubs
    }

    pub fn is_spades(&self) -> bool {
        self.suit == Suit::Spades
    }

    pub fn is_diamonds(&self) -> bool {
        self.suit == Suit::Diamonds
    }
}
