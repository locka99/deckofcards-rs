use card::Card;

// TODO deck and hand have so much in common they should share a trait for cards, shuffle, sort

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

    /// Shuffles the hand
    pub fn shuffle(&mut self) {
        super::shuffle(self.cards.as_mut_slice());
    }

    /// Sorts the hand by suit then value (low to high)
    pub fn sort_by_suit_then_value(&mut self) {
        super::sort_by_suit_then_value(self.cards.as_mut_slice());
    }

    /// Sorts hand by value (high to low), then suit
    pub fn sort_high_to_low(&mut self) {
        super::sort_high_to_low(self.cards.as_mut_slice());
    }

    /// Returns the cards as a slice to look at
    pub fn  as_slice(&self) -> &[Card] {
        self.cards.as_slice()
    }
}
