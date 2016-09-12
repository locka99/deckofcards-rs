use std::cmp::Ordering;
use std::slice::Iter;

use self::Rank::*;

/// This enumeration holds the ranks in a standard deck of cards.
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Debug)]
pub enum Rank {
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
    Ace,
}

impl Ord for Rank {
    fn cmp(&self, other: &Rank) -> Ordering {
        let ord1 = self.ordinal();
        let ord2 = other.ordinal();
        if ord1 < ord2 {
            return Ordering::Less;
        } else if ord1 > ord2 {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}


impl Rank {
    /// Returns an iterator through the standard ranks
    pub fn iterator() -> Iter<'static, Rank> {
        Rank::ranks().into_iter()
    }

    /// Returns an ordinal for the rank.
    pub fn ordinal(&self) -> usize {
        match *self {
            Two => 0,
            Three => 1,
            Four => 2,
            Five => 3,
            Six => 4,
            Seven => 5,
            Eight => 6,
            Nine => 7,
            Ten => 8,
            Jack => 9,
            Queen => 10,
            King => 11,
            Ace => 12,
        }
    }

    /// A comparator that treats an Ace as a 1
    pub fn cmp_ace_low(&self, other: &Rank) -> Ordering {
        // Fudge the ordinal so that Ace can be treated as a 0
        let mut ord1 = self.ordinal() + 1;
        let mut ord2 = other.ordinal() + 1;
        if *self == Ace {
            ord1 = 0;
        }
        if *other == Ace {
            ord2 = 0;
        }
        if ord1 < ord2 {
            return Ordering::Less;
        } else if ord1 > ord2 {
            return Ordering::Greater;
        }
        Ordering::Equal
    }

    /// Returns a Rank represented by a char
    pub fn from_char(ch: char) -> Result<Rank, &'static str> {
        let s = Rank::chars().to_string();
        for (i, c) in s.chars().enumerate() {
            // Special case for '1'->'A'
            if c == ch || (ch == '1' && c == 'A') {
                return Ok(Rank::ranks()[i]);
            }
        }
        Err("Invalid rank")
    }

    /// Turns a Rank into a char
    pub fn to_char(&self) -> char {
        let ord = self.ordinal();
        let b: &[u8] = Rank::chars().as_bytes();
        b[ord] as char
    }

    /// Turns a Rank into a string
    pub fn to_str(&self) -> &'static str {
        match *self {
            Two => "Two",
            Three => "Three",
            Four => "Four",
            Five => "Five",
            Six => "Six",
            Seven => "Seven",
            Eight => "Eight",
            Nine => "Nine",
            Ten => "Ten",
            Jack => "Jack",
            Queen => "Queen",
            King => "King",
            Ace => "Ace",
        }
    }

    /// Gets the standard card ranks
    pub fn ranks() -> &'static [Rank] {
        static RANKS: [Rank; 13] = [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack,
                                    Queen, King, Ace];
        &RANKS[..]
    }

    /// A string containing chars for each standard Rank
    fn chars() -> &'static str {
        "23456789TJQKA"
    }
}
