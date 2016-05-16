use std::fmt;

use super::*;

/// A hand is zero or more cards that represents some aspect of a game, e.g. the cards a person is holding.
/// A hand may be shuffled or sorted and there are functions for adding or removing cards.
/// Unlike a Deck, there is no concept of dealt or undealt cards.
pub struct Hand {
    pub cards : Vec<Card>
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (i, card) in self.cards.iter().enumerate() {
            result.push_str(&card.to_str());
            if i < self.cards.len() - 1 {
                result.push(',');
            }
        }
        write!(f, "{}", result)
    }
}

impl Cards for Hand {
    fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [Card] {
        self.cards.as_mut_slice()
    }
}

impl Hand {
    /// Make a new empty hand
    pub fn new() -> Hand {
        Hand {
            cards: Vec::new()
        }
    }

    /// Makes a hand from another hand
    pub fn from_hand(hand : Hand) -> Hand {
        Hand::from_cards(hand.cards())
    }

    /// Makes a hand from a slice
    pub fn from_cards(cards : &[Card]) -> Hand {
        Hand {
            cards: Vec::from(cards)
        }
    }

    /// Constructs a hand from a slice
    pub fn from_strings(card_slice : &[&str]) -> Hand {
        let mut cards : Vec<Card> = Vec::with_capacity(card_slice.len());
        for s in card_slice {
            let card = card!(s);
            cards.push(card);
        }
        Hand {
            cards: cards
        }
    }

    /// Adds one card to the hand
    pub fn push(&mut self, card : Card) {
        self.cards.push(card);
    }

    /// Adds zero or more cards to the hand
    pub fn push_cards(&mut self, cards: &[Card]) {
        self.cards.extend(cards);
    }

    /// Adds zero or more cards from some other hand
    pub fn push_hand(&mut self, other: &Hand) {
        self.cards.extend(other.cards());
    }

    /// Returns the number of cards
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Removes a card from the hand and returns it, panics if index does not exist
    pub fn remove(&mut self, index: usize) -> Card {
        self.cards.remove(index)
    }

    /// Removes the first instance of every matching card from the hand
    pub fn remove_cards(&mut self, cards: &[Card]) {
        for c in cards {
            self.remove_card(*c);
        }
    }

    /// Removes first instance of the matching card from the hand
    pub fn remove_card(&mut self, card: Card) {
        let found = self.cards.iter().position(|c| *c == card);
        if found.is_some() {
            self.cards.remove(found.unwrap());
        }
    }

    /// Returns cards of the specified rank
    pub fn cards_of_rank(&self, rank: Rank) -> Vec<Card> {
        cards_of_rank(&self.cards, rank)
    }

    /// Returns cards of the specified suit
    pub fn cards_of_suit(&self, suit: Suit) -> Vec<Card> {
        cards_of_suit(&self.cards, suit)
    }
}
