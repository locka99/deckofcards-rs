use card::Card;

/// A hand is zero or more cards that represents some aspect of a game, e.g. the cards a person is holding
pub struct Hand {
    pub cards : Vec<Card>
}

impl Hand {
    pub fn new(cards : &[Card]) -> Hand {
        Hand {
            cards: Vec::from(cards)
        }
    }

    pub fn push(&mut self, card : Card) {
        self.cards.push(card);
    }

    pub fn sort_values_high_to_low(&mut self) {
        unimplemented!();
    }

    pub fn sort_values_low_to_high(&mut self) {
        unimplemented!();
    }

    pub fn sort_by_suit(&mut self) {
        unimplemented!();
    }
}
