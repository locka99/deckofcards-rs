extern crate rand;

/// Creates a `Card` and sets its rank / suit from its abbreviated string description. The description
/// is of the form "RS", Rank followed by Suit, e.g. "2D" for Two of Diamonds.
///
/// # Examples
///
/// Creates the Ace of Spades
///
/// ```
/// let card = card!("AS");
/// ```
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

/// Creates a `Hand` of cards from the list of abbreviated cards string specified by rank / suit,
///
/// # Examples
///
/// Creates a hand containing the Queen of Hearts and Two of Diamonds.
///
/// ```
/// let hand = hand!("QH", "2D");
/// ```
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

/// Creates a new `Hand` that is the combination two hands into one hand. This does not consume
/// the original hands.
///
/// # Examples
///
/// Combine hand1 and hand2 into a new hand_combined.
///
/// ```
/// let hand_combined = combine_hands!(hand1, hand2);
/// ```
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

/// Creates a standard deck of 52 playing cards
#[macro_export]
macro_rules! deck {
    () => {
        Deck::new()
    }
}

mod suit;
pub use suit::{Suit};

mod rank;
pub use rank::{Rank};

mod card;
pub use card::{Card};

mod cards;
pub use cards::{Cards, cards_of_suit, cards_of_rank};

mod deck;
pub use deck::{Deck};

mod hand;
pub use hand::{Hand};

#[cfg(test)]
mod tests;
