use std::vec;

use cards::Card;

pub struct Deck {
    // A deck contains zero or more cards
    cards: Vec<Card>,
    // Dealt cards are cards which have been dealt in calls but are still members of the deck
    // they remain dealt until the deck is reshuffled or reset.
    dealtCards: Vec<Card>
}

impl Deck {
    // insert cards
//    pub fn new() -> Deck {
//        Deck {
//
//        }
//    }

    // remove cards

    // shuffle

    // reset

    // deal

    // dealt / undealt
}
