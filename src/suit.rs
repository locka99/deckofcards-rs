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
        Suit::suits().into_iter()
    }

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

    pub fn from_char(ch: char) -> Result<Suit, &'static str> {
        let s = Suit::chars().to_string();
        for (i, c) in s.chars().enumerate() {
            if c == ch {
                return Ok(Suit::suits()[i]);
            }
        }
        Err("Invalid suit")
    }

    pub fn to_char(&self) -> char {
        let ord = self.ordinal();
        let b : &[u8] = Suit::chars().as_bytes();
        b[ord] as char
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

    fn suits() -> &'static[Suit] {
        static SUITS: [Suit; 4] =
            [Spades, Hearts, Diamonds, Clubs];
        &SUITS[0..]
    }

    fn chars() -> &'static str {
        "SHDC"
    }
}
