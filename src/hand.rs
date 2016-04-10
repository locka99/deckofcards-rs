use std::cmp::Ordering;

use card::Card;

/// A hand is zero or more cards that represents some aspect of a game, e.g. the cards a person is holding
pub struct Hand {
    pub cards : Vec<Card>
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::new()
        }
    }

    pub fn from_cards(cards : &[Card]) -> Hand {
        Hand {
            cards: Vec::from(cards)
        }
    }

    pub fn push(&mut self, card : Card) {
        self.cards.push(card);
    }

    /// Sorts the cards from low to high. An Ace is considered high
    pub fn sort(&mut self) {
        self.cards.sort();
    }

    /// Sorts hand from highest to lowest
    pub fn sort_high_to_low(&mut self) {
        // Reverse sort
        let sort = |a : &Card, b : &Card| -> Ordering{
            let order = a.cmp(b);
            if order == Ordering::Less {
                return Ordering::Greater;
            }
            else if order == Ordering::Greater {
                return Ordering::Less;
            }
            Ordering::Equal
        };
        self.cards.sort_by(sort);
    }

    /// Returns the cards as a slice to look at
    pub fn  as_slice(&self) -> &[Card] {
        self.cards.as_slice()
    }
}
