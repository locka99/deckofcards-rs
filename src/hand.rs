use super::*;

// TODO deck and hand have so much in common they should share a trait for cards, shuffle, sort

/// A hand is zero or more cards that represents some aspect of a game, e.g. the cards a person is holding
pub struct Hand {
    pub cards : Vec<Card>
}

impl Cards for Hand {
    fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [Card] {
        self.cards.as_mut_slice()
    }

    fn shuffle(&mut self) {
        super::shuffle(self.mut_cards());
    }

    fn sort_suit_ascending_value(&mut self) {
        super::sort_suit_ascending_value(self.mut_cards());
    }

    fn sort_descending_value_suit(&mut self) {
        super::sort_descending_value_suit(self.mut_cards());
    }
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
}
