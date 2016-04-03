use std::slice::Iter;

use value::Value;
use suit::Suit;

#[derive(Copy, Clone, PartialEq, Debug)]
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

    pub fn all_cards() -> &'static[Card] {
        static CARDS : [Card; 52] = [
            Card { suit: Suit::Spades, value: Value::Ace },
            Card { suit: Suit::Spades, value: Value::Two },
            Card { suit: Suit::Spades, value: Value::Three },
            Card { suit: Suit::Spades, value: Value::Four },
            Card { suit: Suit::Spades, value: Value::Five },
            Card { suit: Suit::Spades, value: Value::Six },
            Card { suit: Suit::Spades, value: Value::Seven },
            Card { suit: Suit::Spades, value: Value::Eight },
            Card { suit: Suit::Spades, value: Value::Nine },
            Card { suit: Suit::Spades, value: Value::Ten },
            Card { suit: Suit::Spades, value: Value::Jack },
            Card { suit: Suit::Spades, value: Value::Queen },
            Card { suit: Suit::Spades, value: Value::King },

            Card { suit: Suit::Hearts, value: Value::Ace },
            Card { suit: Suit::Hearts, value: Value::Two },
            Card { suit: Suit::Hearts, value: Value::Three },
            Card { suit: Suit::Hearts, value: Value::Four },
            Card { suit: Suit::Hearts, value: Value::Five },
            Card { suit: Suit::Hearts, value: Value::Six },
            Card { suit: Suit::Hearts, value: Value::Seven },
            Card { suit: Suit::Hearts, value: Value::Eight },
            Card { suit: Suit::Hearts, value: Value::Nine },
            Card { suit: Suit::Hearts, value: Value::Ten },
            Card { suit: Suit::Hearts, value: Value::Jack },
            Card { suit: Suit::Hearts, value: Value::Queen },
            Card { suit: Suit::Hearts, value: Value::King },

            Card { suit: Suit::Diamonds, value: Value::Ace },
            Card { suit: Suit::Diamonds, value: Value::Two },
            Card { suit: Suit::Diamonds, value: Value::Three },
            Card { suit: Suit::Diamonds, value: Value::Four },
            Card { suit: Suit::Diamonds, value: Value::Five },
            Card { suit: Suit::Diamonds, value: Value::Six },
            Card { suit: Suit::Diamonds, value: Value::Seven },
            Card { suit: Suit::Diamonds, value: Value::Eight },
            Card { suit: Suit::Diamonds, value: Value::Nine },
            Card { suit: Suit::Diamonds, value: Value::Ten },
            Card { suit: Suit::Diamonds, value: Value::Jack },
            Card { suit: Suit::Diamonds, value: Value::Queen },
            Card { suit: Suit::Diamonds, value: Value::King },

            Card { suit: Suit::Clubs, value: Value::Ace },
            Card { suit: Suit::Clubs, value: Value::Two },
            Card { suit: Suit::Clubs, value: Value::Three },
            Card { suit: Suit::Clubs, value: Value::Four },
            Card { suit: Suit::Clubs, value: Value::Five },
            Card { suit: Suit::Clubs, value: Value::Six },
            Card { suit: Suit::Clubs, value: Value::Seven },
            Card { suit: Suit::Clubs, value: Value::Eight },
            Card { suit: Suit::Clubs, value: Value::Nine },
            Card { suit: Suit::Clubs, value: Value::Ten },
            Card { suit: Suit::Clubs, value: Value::Jack },
            Card { suit: Suit::Clubs, value: Value::Queen },
            Card { suit: Suit::Clubs, value: Value::King }
        ];
        &CARDS
    }

    pub fn iterator() -> Iter<'static, Card> {
        Card::all_cards().into_iter()
    }

    pub fn name(&self) -> String {
        let mut name : String = self.value.to_str().to_string();
        name = name + " of ";
        name = name + self.suit.to_str();
        name
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
