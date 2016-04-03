extern crate rand;

use rand::Rng;

use std::vec::Vec;
use std::result::Result;

use card::Card;

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
        Deck::new_from(Card::all_cards())
    }

    pub fn new_from(cards : &[Card]) -> Deck {
        let mut deck = Deck {
            cards: Vec::with_capacity(cards.len()),
            dealt_cards: Vec::with_capacity(cards.len())
        };
        deck.populate(cards);
        deck
    }

    fn populate(&mut self, cards: &[Card]) {
        for card in cards {
            self.cards.push(*card);
        }
    }

    pub fn undealt_count(&self) -> usize {
        self.cards.len()
    }

    pub fn dealt_count(&self) -> usize {
        self.dealt_cards.len()
    }

    pub fn count(&self) -> usize {
        self.undealt_count() + self.dealt_count()
    }

    pub fn deal_one(&mut self) -> Result<Card, &'static str> {
        let result: Result<Card, &'static str>;
        if self.cards.is_empty() {
            result = Err("No cards left");
        }
        else {
            let card = self.cards.pop().unwrap();
            self.dealt_cards.push(card);
            result = Ok(card)
        }
        result
    }

    pub fn deal_many(&mut self, numcards : i32) -> Vec<Card> {
        let mut result : Vec<Card> = Vec::with_capacity(numcards as usize);
        for _ in 0..numcards {
            let card : Result<Card, &'static str> = self.deal_one();
            if card.is_ok() {
                result.push(card.unwrap());
            }
            else {
                // No cards so no point continuing
                break;
            }
        }
        result
    }

    pub fn undealt_as_slice(&self) -> &[Card] {
        self.cards.as_slice()
    }

    pub fn dealt_as_slice(&self) -> &[Card] {
        self.dealt_cards.as_slice()
    }

    pub fn shuffle(&mut self) {
        if self.cards.is_empty() {
            return;
        }

        let mut cards = self.cards.as_mut_slice();
        let mut rng =  rand::thread_rng();

        // Knuth shuffle
        let num_cards = cards.len();
        for i in (1 .. num_cards - 1).rev() {
            let j = rng.gen_range(0, i);
            cards.swap(i, j);
        }
    }

    // reset
    pub fn reset(&mut self) {
        // Put cards back into undealt deck in reverse order
        self.cards.extend(self.dealt_cards.iter().rev());
        self.dealt_cards.clear();
    }
}
