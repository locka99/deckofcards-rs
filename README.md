# deckofcards for Rust

This is a simple library implemented in Rust that models a deck of cards.

## Cargo import

You can use the module in the standard way by adding this to your Cargo.toml. Unless you have a reason not to, you should use the latest released version.

```
[dependencies]
deckofcards = "*"
```

## API documentation

Once you've added a dependency you can get class documentation like so:

```
cargo doc
```

## Usage

The Deck class contains zero or more Cards which are held in dealt or undealt piles. You can shuffle() the deck.
You can deal_one() card or deal_many(). You can reset() to return dealt cards to the undealt pile.

```
use deckofcards::*;

let mut deck = Deck::new();
```

By default if you don't shuffle, your deck will be sorted by suit then rank. You can shuffle the deck using a randomized
Knuth shuffle:

```
deck.shuffle();
```

You can deal cards out to a Vec<Card>:

```
let cards = deck.deal(5);
```

Or into the Hand object which provides additional sorting and filtering.

```
let mut hand = Hand::new();
deck.deal(&mut hand, 3);
```

Each card has a rank and a suit, both of which are strong enum types. Cards can be compared, sorted and have helpers
to print a long and short description. See sample/ folder for a simple sample that creates a deck, shuffles it and
deals out some cards from the deck.
