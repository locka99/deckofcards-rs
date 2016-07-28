#[macro_use]
extern crate deckofcards;

use deckofcards::{Deck, Cards};

fn main() {
    let mut deck = Deck::new();

    // Shuffle the deck
    deck.shuffle();

    // Deal a card
    for _ in 0..10 {
        let next = deck.deal_one();
        if next.is_ok() {
          let card = next.unwrap();
          println!("You dealt a {}", card.name());
        }
    }

    // Put dealt cards back onto the deck
    deck.reset();
}
