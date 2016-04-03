use std::slice::Iter;

use self::Suit::*;

// Standard card suits
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
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

    pub fn ordinal(&self) -> u8 {
        let result : u8;
        match *self {
            Spades => result = 0,
            Hearts => result = 1,
            Diamonds => result = 2,
            Clubs => result = 3
        }
        result
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
