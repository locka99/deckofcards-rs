use std::cmp::Ordering;
use std::slice::Iter;

use self::Value::*;

/// Standard card values
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Debug)]
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

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering
    {
        let ord1 = self.ordinal();
        let ord2 = other.ordinal();
        if ord1 < ord2 {
            return Ordering::Less;
        }
        else if ord1 > ord2 {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}


impl Value {
    /// Returns an iterator through the standard values
    pub fn iterator() -> Iter<'static, Value> {
        Value::values().into_iter()
    }

    /// Returns an ordinal for the card
    pub fn ordinal(&self) -> usize {
        let result : usize;
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
            Ace => result = 12
        }
        result
    }

    /// Returns a Value represented by a char
    pub fn from_char(ch: char) -> Result<Value, &'static str> {
        let s = Value::chars().to_string();
        for (i, c) in s.chars().enumerate() {
            // Special case for '1'->'A'
            if c == ch || (ch == '1' && c == 'A') {
                return Ok(Value::values()[i]);
            }
        }
        Err("Invalid value")
    }

    /// Turns a Value into a char
    pub fn to_char(&self) -> char {
        let ord = self.ordinal();
        let b : &[u8] = Value::chars().as_bytes();
        b[ord] as char
    }

    /// Turns a Value into a string
    pub fn to_str(&self) -> &str {
        let value_str;
        match *self {
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
            Ace => value_str = "Ace"
        }
        value_str
    }

    /// Gets the standard list of chars
    fn values() -> &'static[Value] {
        static VALUES: [Value; 13] =
            [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace];
        &VALUES[..]
    }

    /// A string containing chars for each standard Value
    fn chars() -> &'static str {
        "23456789TJQKA"
    }
}
