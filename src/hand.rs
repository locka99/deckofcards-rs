use std::fmt::{Display, Formatter, Result};
use std::ops::AddAssign;

use super::*;

/// A `Hand` is zero or more cards that represents some aspect of a game,
/// e.g. the cards a person is holding. A hand may be shuffled or sorted
/// and there are functions for adding or removing cards. Unlike a `Deck`,
/// there is no concept of dealt or undealt cards.
#[derive(Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = String::with_capacity(self.cards.len() * 3);
        self.cards.iter().enumerate().for_each(|(i, card)| {
            result.push_str(&card.to_str());
            if i < self.cards.len() - 1 {
                result.push(',');
            }
        });
        write!(f, "{}", result)
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self { cards: Vec::new() }
    }
}

impl<'a> AddAssign<&'a Hand> for Hand {
    fn add_assign(&mut self, rhs: &Hand) {
        self.push_hand(rhs);
    }
}

impl AddAssign<Card> for Hand {
    fn add_assign(&mut self, rhs: Card) {
        self.push_card(rhs);
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
    /// Create an empty hand
    pub fn new() -> Self { Self::default() }

    /// Makes a `Hand` from an existing hand
    pub fn from_hand(hand: &Hand) -> Hand {
        Hand::from_cards(hand.cards())
    }

    /// Makes a `Hand` from a slice
    pub fn from_cards(cards: &[Card]) -> Hand {
        Hand { cards: Vec::from(cards) }
    }

    /// Constructs a `Hand` from a slice of strings with abbreviated card rank / suit values
    pub fn from_strings(card_slice: &[&str]) -> Hand {
        let cards = card_slice.iter().map(|s| card!(s)).collect::<Vec<Card>>();
        Hand { cards }
    }

    /// Adds one `Card` to the `Hand`
    pub fn push_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Adds zero or more cards to the `Hand`
    pub fn push_cards(&mut self, cards: &[Card]) {
        self.cards.extend(cards);
    }

    /// Adds zero or more cards from some other `Hand`
    pub fn push_hand(&mut self, other: &Hand) {
        self.cards.extend(other.cards());
    }

    /// Returns the number of cards
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Clears the `Hand` (makes it empty)
    pub fn clear(&mut self) {
    	self.cards.clear();
    }

    /// Removes a `Card` from the `Hand` and returns it, panics if index does not exist
    pub fn remove(&mut self, index: usize) -> Card {
        self.cards.remove(index)
    }

    /// Removes the first instance of every matching card from the `Hand`
    pub fn remove_cards(&mut self, cards: &[Card]) {
        for c in cards {
            let _ = self.remove_card(c);
        }
    }

    /// Removes the every instance of every matching card from the `Hand`
    pub fn remove_all_cards(&mut self, cards: &[Card]) {
        for c in cards {
            while self.remove_card(c) {}
        }
    }

    /// Removes first instance of the matching card from the `Hand`
    pub fn remove_card(&mut self, card: &Card) -> bool {
        if let Some(pos) = self.cards.iter().position(|c| c == card) {
            let _ = self.cards.remove(pos);
            true
        } else {
            false
        }
    }

    /// Returns cards of the specified `Rank`
    pub fn cards_of_rank(&self, rank: Rank) -> Vec<Card> {
        cards_of_rank(&self.cards, rank)
    }

    /// Returns cards of the specified `Suit`
    pub fn cards_of_suit(&self, suit: Suit) -> Vec<Card> {
        cards_of_suit(&self.cards, suit)
    }
}
