use std::vec::Vec;
use std::option::Option;

use cards::Card;
use cards::Suit;
use cards::Value;

pub struct Deck {
    // A deck contains zero or more cards
    cards: Vec<Card>,
    // Dealt cards are cards which have been dealt in calls but are still members of the deck
    // they remain dealt until the deck is reshuffled or reset.
    dealt_cards: Vec<Card>
}

impl Deck {
    // insert cards
    pub fn new() -> Deck {
        let mut deck = Deck {
            cards: Vec::with_capacity(52),
            dealt_cards: Vec::with_capacity(52)
        };
        deck.populate();
        deck
    }

    fn populate(&mut self) {
        for suit in Suit::iterator() {
            for value in Value::iterator() {
                self.cards.push(Card::new(*suit, *value));
            }
        }
    }

    pub fn deal_one(&mut self) -> Option<Card> {
        let result : Option<Card>;
        if self.cards.is_empty() {
            result = None;
        }
        else {
            let card = self.cards.pop().unwrap();
            self.dealt_cards.push(card);
            result = Some(card);
        }
        result
    }

    pub fn deal_many(&mut self, numcards : i32) -> Vec<Card> {
        let mut result : Vec<Card> = Vec::with_capacity(numcards as usize);
        for i in 0..numcards {
            let card : Option<Card> = self.deal_one();
            if card.is_some() {
                result.push(card.unwrap());
            }
        }
        result
    }

    // shuffle
    pub fn shuffle() {

    }

    // reset
    pub fn reset() {

    }
}
