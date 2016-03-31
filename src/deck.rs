extern crate rand;

use rand::Rng;

use std::vec::Vec;
use std::result::Result;

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

    // shuffle
    pub fn shuffle(&mut self) {
        let num_cards = self.cards.len();
        let mut shuffler : Vec<(Card, u32)> = Vec::with_capacity(num_cards);

        for card in self.cards {
            // make a tuple consisting of each card in the input and a random number
            let card_pos = (card, rand::thread_rng().gen::<u32>())
            
            // First assign numbers to each card in the undealt list of cards
            deck_order.push(card_pos);
        }

        // Sort the vector
        shuffler.sort_by(|a, b| a.cmp(b));

        // Clear the cards
    }

    // reset
    pub fn reset(&mut self) {

    }
}
