use std::cmp::Ordering;
use std::slice::Iter;

use self::Rank::*;

/// Standard card ranks
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
        let result: usize;
        match *self {
            Two => result = 0,
            Three => result = 1,
            Four => result = 2,
            Five => result = 3,
            Six => result = 4,
            Seven => result = 5,
            Eight => result = 6,
            Nine => result = 7,
            Ten => result = 8,
            Jack => result = 9,
            Queen => result = 10,
            King => result = 11,
            Ace => result = 12,
        }
        result
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
    pub fn to_str(&self) -> &str {
        let rank_str;
        match *self {
            Two => rank_str = "Two",
            Three => rank_str = "Three",
            Four => rank_str = "Four",
            Five => rank_str = "Five",
            Six => rank_str = "Six",
            Seven => rank_str = "Seven",
            Eight => rank_str = "Eight",
            Nine => rank_str = "Nine",
            Ten => rank_str = "Ten",
            Jack => rank_str = "Jack",
            Queen => rank_str = "Queen",
            King => rank_str = "King",
            Ace => rank_str = "Ace",
        }
        rank_str
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
