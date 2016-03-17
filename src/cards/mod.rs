// Standard card suits
#[derive(PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

// Standard card values
#[derive(PartialEq)]
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

pub struct Card {
    suit: Suit,
    value: Value
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Card {
        Card {
            suit: suit,
            value: value
        }
    }

    pub fn is_hearts(&self) -> bool {
        self.suit == Suit::Hearts
    }

    pub fn is_clubs(&self) -> bool {
        self.suit == Suit::Clubs
    }

    pub fn is_spades(&self) -> bool {
        self.suit == Suit::Spades
    }

    pub fn is_diamonds(&self) -> bool {
        self.suit == Suit::Diamonds
    }
}
