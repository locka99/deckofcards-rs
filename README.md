# Deck of Cards

This is a simple library implemented in Rust that models a deck of cards. You can use it to create a deck, shuffle it,
deal cards one or multiple cards at a time.

## Cargo import

You can use the module in the standard way by adding this to your Cargo.toml. Unless you have a reason not to, you should use the latest released version.

```
[dependencies]
deckofcards = "0.4"
```

## API documentation

Once you've added a dependency you can get class documentation like so:

```
cargo doc
```

## Usage

The Deck class contains zero or more Cards which are held in dealt or undealt piles. You can `shuffle()` the deck.
You can `deal_one()` card or `deal_many()` cards. You can `reset()` to return dealt cards to the undealt pile.

```
use deckofcards::*;

let mut deck = Deck::new();
```

By default if you don't shuffle, your deck will be sorted by suit then rank. You can shuffle the deck using a randomized
Knuth shuffle:

```
deck.shuffle();
```

You can deal cards out to a `Vec<Card>`:

```
let cards = deck.deal(5);
```

Or into the `Hand` object which provides additional sorting and filtering.

```
let mut hand = Hand::new();
deck.deal(&mut hand, 3);
```

Each card has a `Rank` and a `Suit`, both of which are strong enum types.

Cards can be compared, sorted and have helpers to print a long and short description using English notation, e.g. "Ace of Spades" or "AS".

### Macros

The crate provides convenience `card!` and `hand!` macros for declaring cards or hands as text:

```
#[macro_use]
extern crate deckofcards;

//... A Card that is the King of Clubs
let card = card!("KC");
//... A Hand that is Ace of Spades, 3 of Diamonds, Queen of Clubs
let hand = hand!("AS", "3D", "QC");
```

## Example

There is an example for you see how it works in `examples/main.rs`.

```
cargo run --example main
```

## Tests

There are some 30 or so unit tests.

```
cargo test
```