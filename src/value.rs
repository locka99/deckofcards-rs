use std::slice::Iter;

use self::Value::*;

// Standard card values
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
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
        Value::values().into_iter()
    }

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

    pub fn from_char(ch: char) -> Result<Value, &'static str> {
        let s = Value::chars().to_string();
        for (i, c) in s.chars().enumerate() {
            if c == ch {
                return Ok(Value::values()[i]);
            }
        }
        Err("Invalid value")
    }

    pub fn to_char(&self) -> char {
        let ord = self.ordinal();
        let b : &[u8] = Value::chars().as_bytes();
        b[ord] as char
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
            King => value_str = "King"
        }
        value_str
    }

    fn values() -> &'static[Value] {
        static VALUES: [Value; 13] =
            [Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King];
        &VALUES[0..]
    }

    fn chars() -> &'static str {
        "A23456789TJQK"
    }
}
