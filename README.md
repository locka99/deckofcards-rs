# deckofcards for Rust

This is a simple library implemented in Rust that models a deck of cards.

## Cargo import

```
[dependencies]
deckofcards = "0.1.1"
```

## Usage

The Deck class contains zero or more Cards which are held in dealt or undealt piles. You can shuffle() the deck. You can deal_one() card or deal_many(). You can reset() to return dealt cards to the undealt pile.

By default if you don't shuffle, your deck will be sorted.

Typical usage:

```
extern crate deckofcards;

use deckofcards::Deck;

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
```
