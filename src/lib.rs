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
fn sort_descending_value_suit(cards: &mut [Card]) {
    // Reverse sort (since default is low to high)
    // TODO suit will be reversed order which is broken - should have a cmp_high_to_low_value_then_suit
    cards.sort_by(|a, b| a.cmp_value_then_suit(b).reverse());
}

pub trait Cards {
    fn cards(&self) -> &[Card];
    fn mut_cards(&mut self) -> &mut [Card];
    fn shuffle(&mut self);
    fn sort_suit_ascending_value(&mut self);
    fn sort_descending_value_suit(&mut self);
}
