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
        let rank = match ch {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' | '1' => Ace,
            _ => return Err("Invalid rank")
        };
        Ok(rank)
    }

    /// Turns a Rank into a char
    pub fn to_char(&self) -> char {
        match *self {
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            Ten => 'T',
            Jack => 'J',
            Queen => 'Q',
            King => 'K',
            Ace => 'A',
        }
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
        static RANKS: [Rank; 13] = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
        ];
        &RANKS[..]
    }
}
