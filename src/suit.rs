use std::cmp::Ordering;
use std::slice::Iter;

use self::Suit::*;

/// Standard card suits
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

impl Ord for Suit {
    fn cmp(&self, other: &Suit) -> Ordering
    {
        let o1 = self.ordinal();
        let o2 = other.ordinal();
        if o1 < o2 {
            return Ordering::Less;
        }
        else if o1 > o2 {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl Suit {
    /// Returns an iterator through the standard suits
    pub fn iterator() -> Iter<'static, Suit> {
        Suit::suits().into_iter()
    }

    /// Returns an ordinal for the suit
    pub fn ordinal(&self) -> usize {
        let result : usize;
        match *self {
            Spades => result = 0,
            Hearts => result = 1,
            Diamonds => result = 2,
            Clubs => result = 3
        }
        result
    }

    /// Returns a Suit for the character, e.g. Hearts for 'H'
    pub fn from_char(ch: char) -> Result<Suit, &'static str> {
        let s = Suit::chars().to_string();
        for (i, c) in s.chars().enumerate() {
            if c == ch {
                return Ok(Suit::suits()[i]);
            }
        }
        Err("Invalid suit")
    }

    /// Returns a char that the represents the suit, e.g. 'H' for Hearts
    pub fn to_char(&self) -> char {
        let ord = self.ordinal();
        let b : &[u8] = Suit::chars().as_bytes();
        b[ord] as char
    }

    /// Returns a string name of the suit
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

    /// The standard list of suits
    pub fn suits() -> &'static[Suit] {
        static SUITS: [Suit; 4] =
            [Spades, Hearts, Diamonds, Clubs];
        &SUITS[..]
    }

    /// A string with chars for each suit
    fn chars() -> &'static str {
        "SHDC"
    }
}
