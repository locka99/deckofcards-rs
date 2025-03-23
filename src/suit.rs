use std::cmp::Ordering;
use std::slice::Iter;

use self::Suit::*;

/// This enumeration holds the suits in a standard deck of cards.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Ord for Suit {
    fn cmp(&self, other: &Suit) -> Ordering {
        let o1 = self.ordinal();
        let o2 = other.ordinal();
        if o1 < o2 {
            return Ordering::Less;
        } else if o1 > o2 {
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
        match *self {
            Spades => 0,
            Hearts => 1,
            Diamonds => 2,
            Clubs => 3,
        }
    }

    /// Returns a Suit for the character, e.g. Hearts for 'H'
    pub fn from_char(ch: char) -> Result<Suit, &'static str> {
        match ch {
            'S' => Ok(Spades),
            'H' => Ok(Hearts),
            'D' => Ok(Diamonds),
            'C' => Ok(Clubs),
            _ => Err("Invalid suit")
        }
    }

    /// Returns a char that the represents the suit, e.g. 'H' for Hearts
    pub fn to_char(&self) -> char {
        match self {
            Spades => 'S',
            Hearts => 'H',
            Diamonds => 'D',
            Clubs => 'C',
        }
    }

    /// Returns a Suit for the unicode character
    pub fn from_unicode(ch: char) -> Result<Suit, &'static str> {
        match ch {
            '♠' => Ok(Spades),
            '♥' => Ok(Hearts),
            '♦' => Ok(Diamonds),
            '♣' => Ok(Clubs),
            _ => Err("Invalid suit")
        }
    }

    /// Returns a char that represents the suit in unicode
    pub fn to_unicode(&self) -> char {
        match self {
            Spades => '♠',
            Hearts => '♥',
            Diamonds => '♦',
            Clubs => '♣',
        }
    }

    /// Returns a string name of the suit
    pub fn to_str(&self) -> &'static str {
        match *self {
            Spades => "Spades",
            Hearts => "Hearts",
            Diamonds => "Diamonds",
            Clubs => "Clubs",
        }
    }

    /// The standard list of suits
    pub fn suits() -> &'static [Suit] {
        static SUITS: [Suit; 4] = [Spades, Hearts, Diamonds, Clubs];
        &SUITS[..]
    }
}
