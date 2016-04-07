use std::slice::Iter;

use self::Value::*;

/// Standard card values
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Value {
    Ace,
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
    King
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
            Ace => result = 0,
            Two => result = 1,
            Three => result = 2,
            Four => result = 3,
            Five => result = 4,
            Six => result = 5,
            Seven => result = 6,
            Eight => result = 7,
            Nine => result = 8,
            Ten => result = 9,
            Jack => result = 10,
            Queen => result = 11,
            King => result = 12
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
            King => value_str = "King"
        }
        value_str
    }

    /// Gets the standard list of chars
    fn values() -> &'static[Value] {
        static VALUES: [Value; 13] =
            [Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King];
        &VALUES[..]
    }

    /// A string containing chars for each standard Value
    fn chars() -> &'static str {
        "A23456789TJQK"
    }
}
