use std::vec::Vec;
use std::result::Result;

use super::*;

/// This represents a deck of zero or more cards. Internally the
/// deck consists of an undealt and a dealt pile of cards.
/// The undealt pile starts off empty and receives cards as they
/// are dealt from the undealt pile.
/// The deck may be reset to return it to its original state. A
/// deck may be shuffled to randomize its order.
pub struct Deck {
    /// A deck contains zero or more cards
    cards: Vec<Card>,
    /// Dealt cards are cards which have been dealt in calls but are still members of the deck
    /// they remain dealt until the deck is reshuffled or reset.
    dealt_cards: Vec<Card>,
}

impl Cards for Deck {
    fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [Card] {
        self.cards.as_mut_slice()
    }
}

impl Deck {
    /// Creates a new deck containing the standard set of 52 cards
    pub fn new() -> Deck {
        Deck::from_cards(Card::all_cards())
    }

    /// Creates a new deck containing the specified cards
    pub fn from_cards(cards: &[Card]) -> Deck {
        let mut deck = Deck {
            cards: Vec::with_capacity(cards.len()),
            dealt_cards: Vec::with_capacity(cards.len()),
        };
        deck.populate(cards);
        deck
    }

    fn populate(&mut self, cards: &[Card]) {
        self.cards.extend(cards);
    }

    /// Returns the number of remaining undealt cards
    pub fn undealt_count(&self) -> usize {
        self.cards.len()
    }

    /// Returns the number of dealt cards
    pub fn dealt_count(&self) -> usize {
        self.dealt_cards.len()
    }

    /// Returns the number of cards, dealt or undealt, within the deck
    pub fn count(&self) -> usize {
        self.undealt_count() + self.dealt_count()
    }

    /// Deals the card from the undealt pile. If there are no cards left, the function
    /// will return an error.
    pub fn deal_one(&mut self) -> Result<Card, &'static str> {
        if self.cards.is_empty() {
            return Err("No cards left");
        }
        let card = self.cards.pop().unwrap();
        self.dealt_cards.push(card);
        Ok(card)
    }

    /// Deals one or more card from the undealt pile and returns them as an array.
    pub fn deal(&mut self, numcards: usize) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::with_capacity(numcards as usize);
        for _ in 0..numcards {
            let dealt: Result<Card, &'static str> = self.deal_one();
            if let Ok(card) = dealt {
                result.push(card);
            } else {
                // No cards so no point continuing
                break;
            }
        }
        result
    }

    /// Deals one or more card straight to the hand. Returns the number of cards dealt
    pub fn deal_to_hand(&mut self, hand: &mut Hand, numcards: usize) -> usize {
        let mut dealt: usize = 0;
        for _ in 0..numcards {
            let result: Result<Card, &'static str> = self.deal_one();
            if let Ok(card) = result {
                dealt += 1;
                hand.push_card(card);
            } else {
                // No cards so no point continuing
                break;
            }
        }
        dealt
    }

    /// Returns the undealt cards as a slice
    pub fn undealt_as_slice(&self) -> &[Card] {
        self.cards.as_slice()
    }

    /// Returns the dealt cards as a slice
    pub fn dealt_as_slice(&self) -> &[Card] {
        self.dealt_cards.as_slice()
    }

    /// Return the dealt cards back to the end of the undealt pile. Order is preserved according
    /// to the default order or the last shuffle.
    pub fn reset(&mut self) {
        // Put cards back into undealt deck in reverse order
        self.cards.extend(self.dealt_cards.iter().rev());
        self.dealt_cards.clear();
    }

    /// Resets and shuffles the deck
    pub fn reset_shuffle(&mut self) {
        self.reset();
        self.shuffle();
    }
}
