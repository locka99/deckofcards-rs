extern crate rand;

/// Makes a card from its short string description, e.g. card!("AS") makes the Ace of Spades.
#[macro_export]
macro_rules! card {
    ($s:expr) => {
        {
            let cr = $crate::Card::from_str($s);
            if cr.is_err() {
                panic!("Not a known card {}", $s);
            }
            cr.unwrap()
        }
    };
}

/// Makes a hand of cards from the list of cards specified by rank/suit, e.g. hand!("QH", "2D")
#[macro_export]
macro_rules! hand {
    () => {
        Hand::new()
    };
    ( $( $s:expr ),* ) => {
        {
            let mut hand = $crate::Hand::new();
            $(
                hand += card!($s);
            )*
            hand
        }
    };
}

/// Combines two hands into one hand. e.g. let hand_combined = combine_hands!(hand1, hand2);
#[macro_export]
macro_rules! combine_hands {
    ( $( $h: expr),* ) => {
        {
            let mut result = Hand::new();
            $(
                result += $h;
            )*
            result
        }
    }
}

#[macro_export]
macro_rules! deck {
    () => {
        Deck::new()
    }
}

mod suit;
mod rank;
mod card;
mod cards;
mod deck;
mod hand;

pub use suit::Suit;
pub use rank::Rank;
pub use card::Card;
pub use cards::{Cards, cards_of_suit, cards_of_rank};
pub use deck::Deck;
pub use hand::Hand;

#[cfg(test)]
mod tests;
