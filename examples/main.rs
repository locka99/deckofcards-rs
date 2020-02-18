use deckofcards::{Cards, Deck};

fn main() {
    let mut deck = Deck::new();

    // Shuffle the deck
    deck.shuffle();

    // Deal a card
    for _ in 0..10 {
        if let Ok(card) = deck.deal_one() {
            println!("You dealt a {}", card.name());
        } else {
            panic!("We should have enough cards for this not to happen")
        }
    }

    // Put dealt cards back onto the deck
    deck.reset();
}
