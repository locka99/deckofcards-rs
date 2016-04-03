use std::slice::Iter;

use self::Value::*;

// Standard card values
#[derive(Copy, Clone, PartialEq, Debug)]
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
        static VALUES: [Value; 13] =
            [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace];
        VALUES.into_iter()
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
            King => value_str = "King",
        }
        value_str
    }
}
