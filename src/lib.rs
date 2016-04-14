extern crate rand;

mod suit;
mod value;
mod card;
mod deck;
mod hand;

#[cfg(test)]
mod tests;

use rand::Rng;

pub use suit::Suit;
pub use value::Value;
pub use card::Card;
pub use deck::Deck;
pub use hand::Hand;

/// Shuffles the slice of cards
fn shuffle(cards: &mut [Card]) {
    let mut rng = rand::thread_rng();
    // Knuth shuffle
    let num_cards = cards.len();
    for i in (1 .. num_cards - 1).rev() {
        let j = rng.gen_range(0, i);
        cards.swap(i, j);
    }
}

/// Sorts the slice by suit then value (low to high)
fn sort_suit_ascending_value(cards: &mut [Card]) {
    cards.sort_by(|a, b| a.cmp_suit_then_value(b));
}

/// Sorts the slice by value(high to low) and then suit
fn sort_suit_descending_value(cards: &mut [Card]) {
    // Reverse sort (since default is low to high)
    cards.sort_by(|a, b| a.cmp_suit_then_desc_value(b));
}

/// Sorts the slice by value(high to low) and then suit
fn sort_descending_value_suit(cards: &mut [Card]) {
    // Reverse sort (since default is low to high)
    cards.sort_by(|a, b| a.cmp_desc_value_then_suit(b));
}

/// Certain actions are common to a deck and a hand of cards
pub trait Cards {
    fn cards(&self) -> &[Card];
    fn mut_cards(&mut self) -> &mut [Card];

    /// Shuffle the cards into a random order
    fn shuffle(&mut self) {
        shuffle(self.mut_cards());
    }

    /// Sort the cards by suit and then by value (low to high)
    fn sort_suit_ascending_value(&mut self) {
        sort_suit_ascending_value(self.mut_cards());
    }

    /// Sorts the cards by suit and then by value (high to low)
    fn sort_suit_descending_value(&mut self) {
        sort_suit_descending_value(self.mut_cards());
    }

    /// Sort the cards by value (high to low) and then by suit
    fn sort_descending_value_suit(&mut self) {
        sort_descending_value_suit(self.mut_cards());
    }
}
