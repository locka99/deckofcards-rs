# deckofcards for Rust

This is a simple library implemented in Rust that models a deck of cards.

## Cargo import

You can use the module in the standard way by adding this to your Cargo.toml. Unless you have a reason not to, you should use the latest released version.

```
[dependencies]
deckofcards = "0.3.1"
```

## API documentation

Once you've added a dependency you can get class documentation like so:

```
cargo doc
```

## Usage

The Deck class contains zero or more Cards which are held in dealt or undealt piles. You can shuffle() the deck. You can deal_one() card or deal_many(). You can reset() to return dealt cards to the undealt pile.

By default if you don't shuffle, your deck will be sorted.

You can deal cards out to a Vec<Card>, or also into the Hand object which provides additional sorting and filtering.

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

    // Sort by suit and then value
    deck.sort_suit_ascending_value();
}
```
