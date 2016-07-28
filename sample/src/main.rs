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
        if let Ok(card) = next {
            println!("You dealt a {}", card.name());
        }
        else {
            panic!("We should have enough cards for this not to happen")
        }
    }

    // Put dealt cards back onto the deck
    deck.reset();
}
